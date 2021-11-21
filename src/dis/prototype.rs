// Data structures for luajit's prototypes. A luajit compiled file is a collection of prototypes.

// Prototype Format:
//  TODO: describe format.

use std::vec::Vec;
use std::collections::VecDeque;

use super::bytecode_instruction::*;
use super::lua_table::LuaValue;

#[derive(Debug)]
pub struct Prototype {
    pub id: usize,
    pub header: Option<PrototypeHeader>,
    pub raw_uvs: Option<Vec<UpValue>>,
    pub bound_uvs: Option<Vec<String>>,
    pub constants: Option<Constants>,
    pub symbols: Option<Vec<String>>,
    pub instructions: Option<Vec<BytecodeInstruction>>,
    pub proto_parent: Option<usize>,
    pub proto_children: Option<Vec<usize>>,
}

impl Default for Prototype {
    fn default() -> Prototype {
        Prototype {
            id: usize::MAX,
            header: None,
            raw_uvs: None,
            bound_uvs: None,
            constants: None,
            symbols: None,
            instructions: None,
            proto_parent: None,
            proto_children: None,
        }
    }
}

impl Prototype {
    pub fn new(id: usize, header: PrototypeHeader) -> Prototype {
        Prototype {
            id: id,
            header: Some(header),
            .. Default::default()
        }
    }
}

#[derive(Debug)]
pub struct LuajitFileHeader {
    pub magic: u32,
    pub file_debug_flags: u8,
    pub file_name: Option<String>,
}

#[derive(Debug, Copy, Clone)]
pub struct DebugInfoHeader {
    pub size_dbg: u32,
    pub first_line: u32,
    pub num_lines: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct PrototypeHeader {
    pub flags: u8,
    pub num_params: u8,
    pub frame_size: u8,
    pub size_uv: u8,
    pub size_kgc: u32,
    pub size_kn: u32,
    pub instruction_count: u32,
    pub prototype_size: u32,
    pub dbg_info_header: Option<DebugInfoHeader>,
}

#[derive(Debug)]
pub struct UpValue {
    pub table_index: u8,
    pub table_location: u8,
}

/// LJ accesses global constants from 3 arrays. 1 for numbers, 1 for strings, (and 1 for tables?)

#[derive(Debug)]
pub struct Constants {
    strings: VecDeque<String>, 
    non_strings: Vec<LuaValue>,
}

impl Constants {
    pub fn new(unsplit_constants: Vec<LuaValue>) -> Constants {
        let mut cons = Constants {
            strings: VecDeque::new(),
            non_strings: Vec::new(),
        };

        for c in unsplit_constants {
            match c {
                LuaValue::Str(s) => cons.strings.push_front(s),
                LuaValue::ChildProto => (),
                _ => cons.non_strings.push(c),
            }
        }

        cons
    }
}