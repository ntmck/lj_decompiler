pub struct Registers {
    a: u8,
    c: u8,
    b: u8,
    d: u16,
}

impl Registers {
    pub fn new(a: u8, c: u8, b: u8) -> Registers {
        let mut d: u16 = c as u16;
        d <<= 8;
        d |= b as u16;
        Registers {
            a: a,
            c: c,
            b: b,
            d: d
        }
    }
}

pub struct BytecodeInstruction {
    op: u32,
    registers: Registers,
}

impl BytecodeInstruction {
    pub const INSTRUCTION_SIZE: u8 = 4;

    pub fn new(op: u32, a: u8, c: u8, b: u8) -> BytecodeInstruction {
        BytecodeInstruction {
            op: op,
            registers: Registers::new(a, c, b),
        }
    }

    pub fn get_formatted_instruction(&self) -> String {
        format!("[ {:6} => A: {:3}, C: {:3}, B: {:3}, [D]: {:5} ]\n",
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
        "ISNEP",

        "ISTC",
        "ISFC",
        "IST",
        "ISF",

        "MOV",
        "NOT",
        "UNM",
        "LEN",

        "ADDVN",
        "SUBVN",
        "MULVN",
        "DIVVN",
        "MODVN",

        "ADDNV",
        "SUBNV",
        "MULNV",
        "DIVNV",
        "MODNV",

        "ADDVV",
        "SUBVV",
        "MULVV",
        "DIVVV",
        "MODVV",

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
        "RET1",

        "FORI",
        "JFORI",
        "FORL",
        "IFORL",
        "JFORL",
        "ITERL",
        "IITERL",
        "JITERL",
        "LOOP",
        "ILOOP",
        "JLOOP",

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
