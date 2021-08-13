// Data structures for luajit's prototypes. A luajit compiled file is a collection of prototypes.

// Prototype Format:
//  TODO

use std::vec::Vec;

use crate::bytecode_instruction::*;

pub struct DebugInfoHeader {
    size_dbg: u32,
    first_line: u32,
    num_lines: u32,
}

pub struct PrototypeHeader {
    flags: u8,
    num_params: u8,
    frame_size: u8, //TODO: using the frame size, make a symbols table using either the fetched variable names or generate variable names. Symbols table should be per-prototype.
    size_uv: u8,
    size_kgc: u32,
    size_kn: u32,
    instruction_count: u32,
    prototype_size: u32,
    dbg_info_header: Option<DebugInfoHeader>,
    //upvalues
    //upvalue targets??
    //constants
}

pub struct Prototype {
    id: usize,
    header: PrototypeHeader,
    instructions: Vec<BytecodeInstruction>,
    //option<prototype children>
    //option<proto parent>
}
