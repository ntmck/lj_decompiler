use std::vec::Vec;

use crate::bytecode_instruction::*;

pub struct PrototypeHeader {
    flags: u8,
    num_params: u8,
    frame_size: u8,
    size_uv: u8,
    size_kgc: u32,
    size_kn: u32,
    instruction_count: u32,
}

//const BYTECODE_INSTRUCTION_SIZE: usize = 4;

pub struct Prototype {
    header: PrototypeHeader,
    instructions: Vec<BytecodeInstruction>,
    //option<prototype children>
    //option<proto parent>
}
