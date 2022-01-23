use std::fmt;

pub enum Exp { //Expression.
    Error(String),
    Empty,

    //Pain
    Label(u32),
    Goto(Box<Exp>),

    //Slots
    Var(u16),

    //Slot Range.
    Range(u16, u16),

    //Constants
    Num(u16),   //index into number constant table.
    Lit(u16),   //literal number not index.
    Str(u16),   //slot into the Strings table
    Uv(u16),    //slot into the uv table.
    Pri(u16),   //primitive literal such as nil, false, true -> 0, 1, 2.
    //Knil(u16, u16) //sets A->D to nil.

    //Tables
    Global, //_G in Table(Exp::Global, target)
    Table(Box<Exp>, Box<Exp>), //name.target

    //Binary Ops
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
    Mod(Box<Exp>, Box<Exp>),
    Pow(Box<Exp>, Box<Exp>),
    Cat(Box<Exp>, Box<Exp>),

    //Unary
    Move(Box<Exp>, Box<Exp>), //assignment. move Box<Exp> into slot u16
    Unm(Box<Exp>),
    Len(Box<Exp>),

    //IST/F(C) -> A, D. if A is not empty, then it is an IST/FC op.
    IsT(Box<Exp>, Box<Exp>), //NotD = ISF.

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
    Not(Box<Exp>),
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
    Func(u16, Box<Exp>), //proto index, func info?
    VarArg(Box<Exp>), //var args Range(from, to)
    ParamCount(u16),
    ReturnCount(u16),
    Call(Box<Exp>, Box<Exp>, Box<Exp>, bool), //Name, Param Range, Return Range, isVarArg

    //Returns
    Return(Box<Exp>),
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "".to_string();

        match self {
            Exp::Empty                  => result.push_str("(empty)"),
            Exp::Error(v)               => result.push_str(&format!("(error: {})", v)),
            Exp::Range(v1, v2)          =>  result.push_str(&format!("{}->{}", v1, v2)),
            Exp::Label(v)               => result.push_str(&format!("label({})", v)),
            Exp::Goto(v)                => result.push_str(&format!("goto({})", v)),
            Exp::Var(v)                 => result.push_str(&format!("var({})", v)),
            Exp::Num(v)                 => result.push_str(&format!("num({})", v)),
            Exp::Lit(v)                 => result.push_str(&format!("lit({})", v)),
            Exp::Str(v)                 => result.push_str(&format!("str({})", v)),
            Exp::Uv(v)                  => result.push_str(&format!("uv({})", v)),
            Exp::Pri(v)                 => result.push_str(&format!("pri({})", v)),
            Exp::Global                 => result.push_str("_G"),
            Exp::Table(v1, v2)          => result.push_str(&format!("({}.{})", v1, v2)),
            Exp::Add(v1, v2)            => result.push_str(&format!("({} + {})", v1, v2)),
            Exp::Sub(v1, v2)            => result.push_str(&format!("({} - {})", v1, v2)),
            Exp::Mul(v1, v2)            => result.push_str(&format!("({} * {})", v1, v2)),
            Exp::Div(v1, v2)            => result.push_str(&format!("({} / {})", v1, v2)),
            Exp::Mod(v1, v2)            => result.push_str(&format!("({} % {})", v1, v2)),
            Exp::Pow(v1, v2)            => result.push_str(&format!("({}^{})", v1, v2)),
            Exp::Cat(v1, v2)            => result.push_str(&format!("({} .. {})", v1, v2)),
            Exp::Unm(v)                 => result.push_str(&format!("-({})", v)),
            Exp::Move(v1, v2)           => result.push_str(&format!("{} := {}", v1, v2)),
            Exp::Len(v)                 => result.push_str(&format!("len({})", v)),
            Exp::Gt                     => result.push_str(">"),
            Exp::Gte                    => result.push_str(">="),
            Exp::NGt                    => result.push_str("~>"),
            Exp::NGte                   => result.push_str("~>="),
            Exp::Lt                     => result.push_str("<"),
            Exp::Lte                    => result.push_str("<="),
            Exp::NLt                    => result.push_str("~<"),
            Exp::NLte                   => result.push_str("~<="),
            Exp::Equals                 => result.push_str("=="),
            Exp::NEquals                => result.push_str("~="),
            Exp::Comparison(v1, v2, v3) => result.push_str(&format!("({} {} {})", v1, v2, v3)),
            Exp::Not(v)                 => result.push_str(&format!("not({})", v)),
            Exp::And(v1, v2)            => result.push_str(&format!("({} and {})", v1, v2)),
            Exp::Or(v1, v2)             => result.push_str(&format!("({} or {})", v1, v2)),
            Exp::UClo(v1, v2)           => result.push_str(&format!("uclo({}, {})", v1, v2)),
            Exp::Target(v1)             => result.push_str(&format!("jmp({})", v1)),
            Exp::If(v1, v2, v3)         => result.push_str(&format!("if {} then {}:{}", v1, v2, v3)),
            Exp::Else(v1, v2, v3)       => result.push_str(&format!("else {} then {}:{}", v1, v2, v3)),
            Exp::While(v1, v2, v3)      => result.push_str(&format!("while {} then {}:{}", v1, v2, v3)),
            Exp::For(v1, v2, v3)        => result.push_str(&format!("for {} then {}:{}", v1, v2, v3)),
            Exp::Repeat(v1, v2, v3)     => result.push_str(&format!("repeat {} then {}:{}", v1, v2, v3)),
            Exp::Func(v1, v2)           => result.push_str(&format!("func(proto:{}, info:{})", v1, v2)),
            Exp::VarArg(v)              => result.push_str(&format!("varg({})", v)), 
            Exp::ParamCount(v)          => result.push_str(&format!("params({})", v)),
            Exp::ReturnCount(v)         => result.push_str(&format!("returns({})", v)),
            Exp::Call(v1, v2, v3, v4)   => result.push_str(&format!("call({}, params({}), returns({}), isVarArg({}))", v1, v2, v3, v4)),
            Exp::Return(v)              => result.push_str(&format!("return({})", v)),
            Exp::IsT(v1, v2)            => result.push_str(&format!("IsT({}, {})", v2, v1)),
        }
        
        write!(f, "{}", result)
    }
}

pub struct IrGen{}
impl IrGen{}