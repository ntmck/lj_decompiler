use crate::{
    dis::bytecode_instruction::BytecodeInstruction,
    ir::blocker::Block,
};

pub enum Expression {
    Error,
    Empty,

    //Gotos :(
    Label(u32),
    Goto(Box<Expression>),

    //Slots
    Var(u16),

    //Constants
    Num(u16),
    Str(u16),
    Uv(u16),
    Pri(u16), //primitive such as nil, false, true -> 0, 1, 2.

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
    UnaryMinus(Box<Expression>),
    Move(Box<Expression>, Box<Expression>), //assignment. move Box<Expression> into slot u16
    Len(Box<Expression>),

    //Boolean
    Gt,     // >
    Gte,    // >=
    Lt,     // <
    Lte,    // <=
    Equals, // ==
    Comparison(Box<Expression>, Box<Expression>, Box<Expression>), //exp op exp
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    
    //Branching
    Target(u32),                                //Instruction targeted by jump.
    Branch(Box<Expression>, Box<Expression>),   //Target if true, Target if false
    If(Box<Expression>, Box<Expression>),       //Comparison, Branch
    While(Box<Expression>, Box<Expression>),    //Comparison, Branch
    For(Box<Expression>, Box<Expression>),      //Comparison, Branch

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
            0..=11  => self.comparison(bci),
            //12..=15 => unary test/copy op..
            //arithmetic
            20..=24 => self.vv_vn(bci, false),
            25..=29 => self.nv(bci),
            30..=36 => self.vv_vn(bci, true),

            //37..=42 => self.constant(bci),

            _ => Expression::Error,
        }
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

    fn comparison(&self, bci: &BytecodeInstruction) -> Expression {
        let a = Expression::Var(bci.a() as u16);
        let d = match bci.op {
            op if op < 6    => Expression::Var(bci.d()),
            op if op < 8    => Expression::Str(bci.d()),
            op if op < 10   => Expression::Num(bci.d()),
            op if op < 12   => Expression::Pri(bci.d()),
            _               => Expression::Error,
        };
        let exp_op = match bci.op {
            0 if (bci.a() as u16) <= bci.d()            => Expression::Not(Box::new(Expression::Lt)),
            0 if (bci.a() as u16) > bci.d()             => Expression::Not(Box::new(Expression::Gt)),
            1 if (bci.a() as u16) <= bci.d()            => Expression::Lt,
            1 if (bci.a() as u16) > bci.d()             => Expression::Gt, 
            2 if (bci.a() as u16) <= bci.d()            => Expression::Not(Box::new(Expression::Lte)),
            2 if (bci.a() as u16) > bci.d()             => Expression::Not(Box::new(Expression::Gte)),
            3 if (bci.a() as u16) <= bci.d()            => Expression::Lte,
            3 if (bci.a() as u16) > bci.d()             => Expression::Gte,
            op if (4..=11).contains(&op) && op % 2 == 0 => Expression::Equals,
            op if (4..=11).contains(&op) && op % 2 == 1 => Expression::Not(Box::new(Expression::Equals)),
            _                                           => Expression::Error,
        };
        let comparison = Box::new(Expression::Comparison(Box::new(a), Box::new(exp_op), Box::new(d)));
        let t_target = Box::new(Expression::Target(bci.get_jump_target()));
        let f_target = Box::new(Expression::Empty); //TODO: Set this in the following JMP instruction.
        let branch = Box::new(Expression::Branch(t_target, f_target));

        Expression::If(comparison, branch)
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