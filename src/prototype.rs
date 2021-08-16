// Data structures for luajit's prototypes. A luajit compiled file is a collection of prototypes.

// Prototype Format:
//  TODO

use std::vec::Vec;
use std::any::Any;

use crate::bytecode_instruction::*;

pub struct UpValue {
    pub table_index: u32,

    //192 = within this prototype's global constants. 0 = look at upvalue table at table_index in this prototype's parent.
    pub table_location: u32,
}

/*
private void SetUpvalueNames()
{
    upvalues = new List<(string, BaseConstant)>();
    for(int i = 0; i < upvalueTargets.Count; i++)
        upvalues.Add(RecursiveGetUpvalue(this, upvalueTargets[i]));
}

private (string, BaseConstant) RecursiveGetUpvalue(Prototype pt, UpValue uv)
{
    //apparently the first bit of 192 determines if we look at the constants section table or not.
    //the second bit of 192 means if it is mutable or not. 1 = immutable upvalue -- whatever that means in terms of upvalues...
    if (pt.parent == null) //NEWLY ADDED. foundation/scripts/util/engine/resolution.lua crashes the disassembler otherwise.
        return ("null", new CInt(0));

    if (uv.TableLocation == 192)
        return (pt.parent.symbols[uv.TableIndex], pt.parent.constants[uv.TableIndex]); //possible array index out of bounds here...
    return RecursiveGetUpvalue(pt.parent, pt.parent.upvalueTargets[uv.TableIndex]);
}
*/

pub struct LuajitFileHeader {
    pub magic: u32,
    pub debug_flags: u8,
    pub proto_count: u32,
}

pub struct DebugInfoHeader {
    pub size_dbg: u32,
    pub first_line: u32,
    pub num_lines: u32,
}

pub struct PrototypeHeader {
    pub flags: u8,
    pub num_params: u8,
    pub frame_size: u8, //TODO: using the frame size, make a symbols table using either the fetched variable names or generate variable names. Symbols table should be per-prototype.
    pub size_uv: u8,
    pub size_kgc: u32,
    pub size_kn: u32,
    pub instruction_count: u32,
    pub prototype_size: u32,
    pub dbg_info_header: Option<DebugInfoHeader>,
}

pub struct Prototype {
    pub id: usize,
    pub header: Option<PrototypeHeader>,
    pub up_values: Option<Vec<usize>>,
    pub constants_table: Option<Vec<Box<dyn Any>>>,
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
            up_values: None,
            constants_table: None,
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
