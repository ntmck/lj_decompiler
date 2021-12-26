use crate::{
    dis::bytecode_instruction::BytecodeInstruction,
    ir::blocker::Block,
};

pub enum Expression {
    Error,
    Empty,
    //Next(Box<Expression>), //Next expression in sequence.

    //Gotos :(
    Label(u32),
    Goto(Box<Expression>),

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
    GlobalTable(Box<Expression>, Box<Expression>),
    Table(Box<Expression>, Box<Expression>), //str(name)[str(target) / num(index)] or table, name.name2.name3...

    //Binary Ops
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    Cat(Box<Expression>, Box<Expression>),

    //Unary
    Unm(Box<Expression>, Box<Expression>),
    Move(Box<Expression>, Box<Expression>), //assignment. move Box<Expression> into slot u16
    Len(Box<Expression>, Box<Expression>),

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
    Comparison(Box<Expression>, Box<Expression>, Box<Expression>), //exp op exp
    Not(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    
    //Branching
    Target(u32),
    If(Box<Expression>, u16, u16), //comparison, start of scope, end of scope.
    Else(Box<Expression>, u16, u16),
    While(Box<Expression>, u16, u16),
    For(Box<Expression>, u16, u16),
    Repeat(Box<Expression>, u16, u16),

    //Functions
    VarArg,
    ParamCount(u32),
    ReturnCount(u32),
    Func(Box<Expression>, Box<Expression>, Box<Expression>), //name, param count or vararg, return count,
    Call(Box<Expression>),

    //Returns
    Return(Box<Expression>),
}

pub struct IrGen{}
impl IrGen {
    pub fn translate_block(block: &Block) -> Vec<Expression> {
        unimplemented!()
    }

    pub fn translate_bci(&self, bci: &BytecodeInstruction) -> Expression {
        match bci.op {
            0..=15  => self.comparison(bci),
            16..=19 => self.unary(bci),
            20..=24 => self.vv_vn(bci, false),
            25..=29 => self.nv(bci),
            30..=36 => self.vv_vn(bci, true),
            37..=42 => self.constant(bci),
            //43..=48 => upvalue ops
            //49      => new prototype FNEW
            //50..=60 => table ops
            //61..=68 => call/var args?
            //69..=72 => returns
            //73..=77 => for loops
            //78..=80 => iter loops
            //81..=83 => while/repeat loops
            84      => Expression::Target(bci.get_jump_target()), //still relevant until higher level statements are built. i.e. if, while
            //85..=92 => funcs
            //93 => GOTOs

            _ => Expression::Error,
        }
    }

    fn unary(&self, bci: &BytecodeInstruction) -> Expression {
        let (a, d) = self.var_a_var_d(bci);
        match bci.op {
            16 => Expression::Move(a, d),
            17 => Expression::Not(a, d),
            18 => Expression::Unm(a, d),
            19 => Expression::Len(a, d),
            _ => Expression::Error,
        }
    }

    fn var_a_var_d(&self, bci: &BytecodeInstruction) -> (Box<Expression>, Box<Expression>) {
        (Box::new(Expression::Var(bci.a() as u16)), Box::new(Expression::Var(bci.d())))
    }

    fn comparison(&self, bci: &BytecodeInstruction) -> Expression {
        let a = Expression::Var(bci.a() as u16);
        let d = match bci.op {
            op if op < 6    => Expression::Var(bci.d()),
            op if op < 8    => Expression::Str(bci.d()),
            op if op < 10   => Expression::Num(bci.d()),
            op if op < 12   => Expression::Pri(bci.d()),
            _               => Expression::Error,
        };
        let op = self.comparison_op(bci);
        let a = Box::new(a);
        let d = Box::new(d);
        let op = Box::new(op);

        Expression::Comparison(a, op, d)
    }

    fn comparison_op(&self, bci: &BytecodeInstruction) -> Expression {
        match bci.op {
            0 if (bci.a() as u16) <= bci.d()            => Expression::NLt,
            0 if (bci.a() as u16) > bci.d()             => Expression::NGt,
            1 if (bci.a() as u16) <= bci.d()            => Expression::Lt,
            1 if (bci.a() as u16) > bci.d()             => Expression::Gt, 
            2 if (bci.a() as u16) <= bci.d()            => Expression::NLte,
            2 if (bci.a() as u16) > bci.d()             => Expression::NGte,
            3 if (bci.a() as u16) <= bci.d()            => Expression::Lte,
            3 if (bci.a() as u16) > bci.d()             => Expression::Gte,
            op if (4..=11).contains(&op) && op % 2 == 0 => Expression::Equals,
            op if (4..=11).contains(&op) && op % 2 == 1 => Expression::NEquals,
            //todo: ISTC/ISFC/IST/ISF
            _                                           => Expression::Error,
        }
    }

    fn constant(&self, bci: &BytecodeInstruction) -> Expression {
        let value = match bci.op {
            33 => Expression::Str(bci.d()),
            34 => unimplemented!("KCDATA"),
            35 => Expression::Short(bci.d()),
            36 => Expression::Var(bci.d()),
            37 => Expression::Pri(bci.d()),
            38 => unimplemented!("KNIL"),
            _ => Expression::Error,
        };
        let dst = Box::new(Expression::Var(bci.a() as u16));
        let value = Box::new(value);
        Expression::Move(dst, value)
    }

    fn arith_ab(&self, bci: &BytecodeInstruction) -> (Box<Expression>, Box<Expression>) {
        let a = Box::new(Expression::Var(bci.a() as u16));
        let b = Box::new(Expression::Var(bci.b() as u16));
        (a, b)
    }

    fn vv_vn(&self, bci: &BytecodeInstruction, is_vv: bool) -> Expression {
        let (a, b) = self.arith_ab(bci);
        let c;
        if is_vv {
            c = Box::new(Expression::Var(bci.c() as u16));
        } else { //is_vn
            c = Box::new(Expression::Num(bci.c() as u16));
        }
        let opr = Box::new(self.binop(bci, b, c));
        Expression::Move(a, opr)
    }

    fn nv(&self, bci: &BytecodeInstruction) -> Expression {
        let (a, b) = self.arith_ab(bci);
        let c = Box::new(Expression::Num(bci.c() as u16));
        let opr = Box::new(self.binop(bci, c, b));
        Expression::Move(a, opr)
    }

    fn binop(&self, bci: &BytecodeInstruction, b: Box<Expression>, c: Box<Expression>) -> Expression {
        match bci.op % 5 {
            0                   => Expression::Add(b, c),
            1 if bci.op == 31   => Expression::Pow(b, c),
            1                   => Expression::Sub(b, c),
            2 if bci.op == 32   => Expression::Cat(b, c),
            2                   => Expression::Mul(b, c),
            3                   => Expression::Div(b, c),
            4                   => Expression::Mod(b, c),
            _                   => Expression::Error,
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