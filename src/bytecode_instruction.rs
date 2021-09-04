#[derive(Debug)]
pub struct Registers {
    a: u8,
    c: u8,
    b: u8,
    d: u16,
//BUG: There is an error with the D register union.
/*
    BytecodeInstruction {
    op: 71, //RET0
    registers: Registers {
        a: 0,
        c: 1,
        b: 0,
        d: 256,
    },
}

compared to:
--Bytecode Instructions--
(TNEW): A = 0, C = 0, B = 0, [D: 0];
(GSET): A = 0, C = 0, B = 0, [D: 0];
(RET0): A = 0, C = 1, B = 0, [D: 1];
*/

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

#[derive(Debug)]
pub struct BytecodeInstruction {
    op: u8,
    registers: Registers,
}

impl BytecodeInstruction {
    pub const INSTRUCTION_SIZE: u8 = 4;

    pub fn new(op: u8, a: u8, c: u8, b: u8) -> BytecodeInstruction {
        BytecodeInstruction {
            op: op,
            registers: Registers::new(a, c, b),
        }
    }

    pub fn get_formatted_instruction(&self) -> String {
        format!("[ {:6} => A: [{:3}], C: [{:3}], B: [{:3}], [D]: [{:3}] ]\n",
            self.get_operation_name(),
            self.registers.a,
            self.registers.c,
            self.registers.b,
            self.registers.d
        )
    }

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

    pub fn is_any_loop(&self) -> bool {
        self.is_for_loop() || self.is_iter_loop() || self.is_norm_loop()
    }

    pub fn is_jump(&self) -> bool {
        self.op == 84
    }

    const OP_LOOKUP: [&'static str; 93] = [
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
        "KNIL",

        "UGET",
        "USETV",
        "USETS",
        "USETN",
        "USETP",

        "UCLO",
        "FNEW",
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
        "TSETM",

        "CALLM",
        "CALL",
        "CALLMT",
        "CALLT",
        "ITERC",
        "ITERN",
        "VARG",
        "ISNEXT",

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
