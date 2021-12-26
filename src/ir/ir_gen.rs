use crate::{
    dis::bytecode_instruction::BytecodeInstruction,
    ir::blocker::Block,
};

pub enum Exp { //Expression.
    Error,
    Empty,
    //Next(Box<Exp>), //Next Exp in sequence.

    //Gotos :(
    Label(u32),
    Goto(Box<Exp>),

    //Slots
    Var(u16),

    //Constants
    Num(u16), //index into number constant table.
    Short(u16), //literal short.
    Str(u16), //slot into the Strings table
    Uv(u16), //slot into the uv table.
    Pri(u16), //primitive literal such as nil, false, true -> 0, 1, 2.
    //Knil(u16, u16) //sets A->D to nil.

    //Tables
    GlobalTable(Box<Exp>, Box<Exp>),
    Table(Box<Exp>, Box<Exp>), //str(name)[str(target) / num(index)] or table, name.name2.name3...

    //Binary Ops
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
    Mod(Box<Exp>, Box<Exp>),
    Pow(Box<Exp>, Box<Exp>),
    Cat(Box<Exp>, Box<Exp>),

    //Unary
    Unm(Box<Exp>, Box<Exp>),
    Move(Box<Exp>, Box<Exp>), //assignment. move Box<Exp> into slot u16
    Len(Box<Exp>, Box<Exp>),

    //Boolean
    Gt,     // >
    Gte,    // >=
    Lt,     // <
    Lte,    // <=
    NGt,    // not >
    NGte,   // not >=
    NLt,    // not <
    NLte,   // not <=

    NEquals, //~= or not ==
    Equals, // ==
    Comparison(Box<Exp>, Box<Exp>, Box<Exp>), //exp op exp
    Not(Box<Exp>, Box<Exp>),
    And(Box<Exp>, Box<Exp>),
    Or(Box<Exp>, Box<Exp>),
    
    //Branching
    UClo(u16, Box<Exp>),
    Target(u32),
    If(Box<Exp>, u16, u16), //comparison, start of scope, end of scope.
    Else(Box<Exp>, u16, u16),
    While(Box<Exp>, u16, u16),
    For(Box<Exp>, u16, u16),
    Repeat(Box<Exp>, u16, u16),

    //Functions
    VarArg,
    ParamCount(u32),
    ReturnCount(u32),
    Func(Box<Exp>, Box<Exp>, Box<Exp>), //name, param count or vararg, return count,
    Call(Box<Exp>),

    //Returns
    Return(Box<Exp>),
}

pub struct IrGen{}
impl IrGen {
    pub fn translate_block(block: &Block) -> Vec<Exp> {
        unimplemented!()
    }

    pub fn translate_bci(&self, bci: &BytecodeInstruction) -> Exp {
        match bci.op {
            0..=15  => self.comparison(bci),
            16..=19 => self.unary(bci),
            20..=24 => self.vv_vn(bci, false),
            25..=29 => self.nv(bci),
            30..=36 => self.vv_vn(bci, true),
            37..=42 => self.constant(bci),
            43..=48 => self.uv(bci),
            49      => self.proto(bci),
            //50..=60 => table ops
            //61..=68 => call/var args?
            //69..=72 => returns
            //73..=77 => for loops
            //78..=80 => iter loops
            //81..=83 => while/repeat loops
            84      => Exp::Target(bci.get_jump_target()), //still relevant until higher level statements are built. i.e. if, while
            //85..=92 => funcs
            //93 => GOTOs

            _ => Exp::Error,
        }
    }

    fn proto(&self, bci: &BytecodeInstruction) -> Exp {
        unimplemented!()
        //Exp::Func() //start of a new func
    }

    fn uv(&self, bci: &BytecodeInstruction) -> Exp {
        match bci.op {
            43      => Exp::Move(Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Uv(bci.d()))),
            44..=47 => self.uset(bci),
            48      => Exp::UClo(bci.a() as u16, Box::new(Exp::Target(bci.get_jump_target()))),
            _       => Exp::Error,
        }
    }

    fn uset(&self, bci: &BytecodeInstruction) -> Exp {
        let a = Exp::Uv(bci.a() as u16);
        let d = match bci.op {
            44  => Exp::Var(bci.d()),
            45  => Exp::Str(bci.d()),
            46  => Exp::Num(bci.d()),
            47  => Exp::Pri(bci.d()),
            _   => Exp::Error,
        };
        let a = Box::new(a);
        let d = Box::new(d);

        Exp::Move(a, d)
    }

    fn unary(&self, bci: &BytecodeInstruction) -> Exp {
        let (a, d) = self.var_a_var_d(bci);
        match bci.op {
            16 => Exp::Move(a, d),
            17 => Exp::Not(a, d),
            18 => Exp::Unm(a, d),
            19 => Exp::Len(a, d),
            _ => Exp::Error,
        }
    }

    fn var_a_var_d(&self, bci: &BytecodeInstruction) -> (Box<Exp>, Box<Exp>) {
        (Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Var(bci.d())))
    }

    fn comparison(&self, bci: &BytecodeInstruction) -> Exp {
        let a = Exp::Var(bci.a() as u16);
        let d = match bci.op {
            op if op < 6    => Exp::Var(bci.d()),
            op if op < 8    => Exp::Str(bci.d()),
            op if op < 10   => Exp::Num(bci.d()),
            op if op < 12   => Exp::Pri(bci.d()),
            _               => Exp::Error,
        };
        let op = self.comparison_op(bci);
        let a = Box::new(a);
        let d = Box::new(d);
        let op = Box::new(op);

        Exp::Comparison(a, op, d)
    }

    fn comparison_op(&self, bci: &BytecodeInstruction) -> Exp {
        match bci.op {
            0 if (bci.a() as u16) <= bci.d()            => Exp::NLt,
            0 if (bci.a() as u16) > bci.d()             => Exp::NGt,
            1 if (bci.a() as u16) <= bci.d()            => Exp::Lt,
            1 if (bci.a() as u16) > bci.d()             => Exp::Gt, 
            2 if (bci.a() as u16) <= bci.d()            => Exp::NLte,
            2 if (bci.a() as u16) > bci.d()             => Exp::NGte,
            3 if (bci.a() as u16) <= bci.d()            => Exp::Lte,
            3 if (bci.a() as u16) > bci.d()             => Exp::Gte,
            op if (4..=11).contains(&op) && op % 2 == 0 => Exp::Equals,
            op if (4..=11).contains(&op) && op % 2 == 1 => Exp::NEquals,
            //todo: ISTC/ISFC/IST/ISF
            _                                           => Exp::Error,
        }
    }

    fn constant(&self, bci: &BytecodeInstruction) -> Exp {
        let value = match bci.op {
            33 => Exp::Str(bci.d()),
            34 => unimplemented!("KCDATA"),
            35 => Exp::Short(bci.d()),
            36 => Exp::Var(bci.d()),
            37 => Exp::Pri(bci.d()),
            38 => unimplemented!("KNIL"),
            _ => Exp::Error,
        };
        let dst = Box::new(Exp::Var(bci.a() as u16));
        let value = Box::new(value);
        Exp::Move(dst, value)
    }

    fn arith_ab(&self, bci: &BytecodeInstruction) -> (Box<Exp>, Box<Exp>) {
        let a = Box::new(Exp::Var(bci.a() as u16));
        let b = Box::new(Exp::Var(bci.b() as u16));
        (a, b)
    }

    fn vv_vn(&self, bci: &BytecodeInstruction, is_vv: bool) -> Exp {
        let (a, b) = self.arith_ab(bci);
        let c;
        if is_vv {
            c = Box::new(Exp::Var(bci.c() as u16));
        } else { //is_vn
            c = Box::new(Exp::Num(bci.c() as u16));
        }
        let opr = Box::new(self.binop(bci, b, c));
        Exp::Move(a, opr)
    }

    fn nv(&self, bci: &BytecodeInstruction) -> Exp {
        let (a, b) = self.arith_ab(bci);
        let c = Box::new(Exp::Num(bci.c() as u16));
        let opr = Box::new(self.binop(bci, c, b));
        Exp::Move(a, opr)
    }

    fn binop(&self, bci: &BytecodeInstruction, b: Box<Exp>, c: Box<Exp>) -> Exp {
        match bci.op % 5 {
            0                   => Exp::Add(b, c),
            1 if bci.op == 31   => Exp::Pow(b, c),
            1                   => Exp::Sub(b, c),
            2 if bci.op == 32   => Exp::Cat(b, c),
            2                   => Exp::Mul(b, c),
            3                   => Exp::Div(b, c),
            4                   => Exp::Mod(b, c),
            _                   => Exp::Error,
        }
    }
}


/*
Source Code     then    Bytecode
if x < y        then    ISGE x y
if x <= y       then    ISGT x y
if x > y        then    ISGE y x
if x >= y       then    ISGT y x

if not (x < y)  then    ISLT x y
if not (x <= y) then    ISLE x y
if not (x > y)  then    ISLT y x
if not (x >= y) then    ISLE y x

if for slots A and D (A <= D = x y), (A > D = y x)
*/