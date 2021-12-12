use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub c: u8,
    pub b: u8,
    pub d: u16,
}

impl Registers {
    pub fn new(a: u8, c: u8, b: u8) -> Registers {
        let mut d: u16 = b as u16;
        d <<= 8;
        d |= c as u16;
        Registers {
            a: a,
            c: c,
            b: b,
            d: d
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BytecodeInstruction {
    pub index: usize,
    pub op: u8,
    pub registers: Registers,
}

impl fmt::Display for BytecodeInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:4}: [ {:6} => A: [{:3}], C: [{:3}], B: [{:3}], D: [{:5}] ]",
        self.index,
        self.get_operation_name(), 
        self.registers.a, 
        self.registers.c, 
        self.registers.b, 
        self.registers.d
        )
    }
}

impl BytecodeInstruction {
    pub const INSTRUCTION_SIZE: u8 = 4;

    pub fn new(index: usize, op: u8, a: u8, c: u8, b: u8) -> BytecodeInstruction {
        BytecodeInstruction {
            index: index,
            op: op,
            registers: Registers::new(a, c, b),
        }
    }

    pub fn a(&self) -> u8   { self.registers.a }
    pub fn c(&self) -> u8   { self.registers.c }
    pub fn b(&self) -> u8   { self.registers.b }
    pub fn d(&self) -> u16  { self.registers.d }

    pub fn get_operation_name(&self) -> String {
        String::from(BytecodeInstruction::OP_LOOKUP[self.op as usize])
    }

    pub fn is_conditional(&self) -> bool {
        self.op < 12
    }

    pub fn is_unary_test_or_copy(&self) -> bool {
        self.op >= 12 && self.op < 16
    }

    pub fn is_unary(&self) -> bool {
        self.op >= 16 && self.op < 20
    }

    pub fn is_vn(&self) -> bool {
        self.op >= 20 && self.op < 25
    }

    pub fn is_nv(&self) -> bool {
        self.op >= 25 && self.op < 30
    }

    pub fn is_vv(&self) -> bool {
        self.op >= 30 && self.op < 35
    }

    pub fn is_ret(&self) -> bool {
        self.op >= 69 && self.op < 73
    }

    pub fn is_for_loop(&self) -> bool {
        self.op >= 73 && self.op < 78
    }

    pub fn is_iter_loop(&self) -> bool {
        self.op >= 78 && self.op < 81
    }

    pub fn is_norm_loop(&self) -> bool {
        self.op >= 81 && self.op < 84
    }

    pub fn is_loop(&self) -> bool {
        self.is_for_loop() || self.is_iter_loop() || self.is_norm_loop()
    }

    pub fn is_jump(&self) -> bool {
        self.op == 84
    }

    pub fn is_constant(&self) -> bool {
        self.op > 36 && self.op <= 42
    }

    pub fn is_pow(&self) -> bool {
        self.op == 35
    }

    pub fn is_cat(&self) -> bool {
        self.op == 36
    }

    pub fn is_fnew(&self) -> bool {
        self.op == 49
    }

    pub fn is_arith(&self) -> bool {
        self.is_nv() || self.is_vv() || self.is_vn()
    }

    pub fn is_add(&self) -> bool {
        self.is_arith() && self.op % 5 == 0
    }

    pub fn is_sub(&self) -> bool {
        self.is_arith() && self.op % 5 == 1
    }
    
    pub fn is_mult(&self) -> bool {
        self.is_arith() && self.op % 5 == 2
    }

    pub fn is_div(&self) -> bool {
        self.is_arith() && self.op % 5 == 3
    }

    pub fn is_mod(&self) -> bool {
        self.is_arith() && self.op % 5 == 4
    }

    pub const OP_LOOKUP: [&'static str; 93] = [
        "ISLT",
        "ISGE",
        "ISLE",
        "ISGT",
        "ISEQV",
        "ISNEV",

        "ISEQS",
        "ISNES",

        "ISEQN",
        "ISNEN",
        
        "ISEQP",
        "ISNEP", //0-11 = conditional

        "ISTC",
        "ISFC",
        "IST",
        "ISF", //12-15 = unary test/copy

        "MOV",
        "NOT",
        "UNM",
        "LEN", //16-19 = unary

        "ADDVN",
        "SUBVN",
        "MULVN",
        "DIVVN",
        "MODVN", //20-24 = vn

        "ADDNV",
        "SUBNV",
        "MULNV",
        "DIVNV",
        "MODNV", //25-29 = nv

        "ADDVV",
        "SUBVV",
        "MULVV",
        "DIVVV",
        "MODVV", //30-34 = vv

        "POW",
        "CAT",

        "KSTR",
        "KCDATA",
        "KSHORT",
        "KNUM",
        "KPRI",
        "KNIL", //37-42 = constants

        "UGET",
        "USETV",
        "USETS",
        "USETN",
        "USETP",
        "UCLO", //43-48 = upvalue ops

        "FNEW", //49

        "TNEW", 
        "TDUP", 
        "GGET", 
        "GSET", 
        "TGETV",
        "TGETS",
        "TGETB",
        "TSETV",
        "TSETS",
        "TSETB",
        "TSETM", //50-60 = table ops

        "CALLM",
        "CALL",
        "CALLMT",
        "CALLT",
        "ITERC",
        "ITERN",
        "VARG",
        "ISNEXT", //61-68 call and var-arg

        "RETM",
        "RET",
        "RET0",
        "RET1", //69-72 = ret

        "FORI",
        "JFORI",
        "FORL",
        "IFORL",
        "JFORL", //73-77 = for loop

        "ITERL",
        "IITERL",
        "JITERL", //78-80 = iter loop

        "LOOP",
        "ILOOP",
        "JLOOP", //81-83 = loop

        "JMP", //goto

        "FUNCF", //func
        "IFUNCF",
        "JFUNCF",
        "FUNCV",
        "IFUNCV",
        "JFUNCV",
        "FUNCC",
        "FUNCCW",
    ];
}
