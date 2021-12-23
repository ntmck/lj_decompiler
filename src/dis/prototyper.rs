use std::{
    collections::VecDeque
};

use crate::{
    dis::{
        lj_file_reader::LJFileReader,
        lj_reader::LJReader,
        bytecode_instruction::BytecodeInstruction,
        lua_table::*,
    },
};

pub struct PrototypeHeader {
    pub id: usize,
    pub flags: u8,
    pub num_params: u8,
    pub frame_size: u8,
    pub size_uv: u8,
    pub size_kgc: u32,
    pub size_kn: u32,
    pub instruction_count: u32,
    pub dbg_info_header: Option<DebugInfoHeader>,
}

pub struct DebugInfoHeader {
    pub size_dbg: u32,
    pub first_line: u32,
    pub num_lines: u32,
}

pub struct UpValue {
    pub table_index: u8,
    pub table_location: u8,
}
impl UpValue { const UPVALUE_SIZE: u8 = 2; }

pub struct Constants {
    pub strings: VecDeque<String>, 
    pub non_strings: Vec<LuaValue>,
}

pub struct LuajitFileHeader {
    pub file_debug_flags: u8,
    pub file_name: Option<String>,
}

pub struct Prototype {
    pub header: PrototypeHeader,
    pub uvs: Vec<UpValue>,
    pub constants: Constants,
    pub symbols: Vec<String>,
    pub instructions: Vec<BytecodeInstruction>,
    pub proto_children: Vec<usize>,
}

impl Prototype {
    pub fn new(ptr: &mut Prototyper, raw_prototype: Vec<u8>) -> Prototype {
        let mut ljr = LJReader::new(raw_prototype);

        let header = Prototype::read_header(&mut ljr, &ptr);
        let bcis = Prototype::read_instructions(&mut ljr, &header);
        let uvs = Prototype::read_raw_upvalues(&mut ljr, &header);
        let mut kgcs = Prototype::read_kgcs(&mut ljr, &header);
        let kns = Prototype::read_kns(&mut ljr, &header);
        let symbols = Prototype::get_symbols(&mut ljr, &header);

        let mut constants = Constants {
            strings: VecDeque::new(),
            non_strings: vec![],
        };
        let mut child_protos: Vec<usize> = vec![];

        { //Sort kgcs and kns into Constants type.
            for kgc in kgcs.iter_mut() {
                match kgc {
                    LuaValue::ChildProto => child_protos.push(ptr.proto_id_stack.pop().unwrap()),
                    LuaValue::Str(s) => constants.strings.push_front(s.to_string()),
                    _ => constants.non_strings.push(std::mem::take(kgc)),
                }
            }
            constants.non_strings.extend(kns);
        }

        { //Update id and id stack.
            let id = ptr.next_id;
            ptr.proto_id_stack.push(id);
            ptr.next_id += 1;
        }

        Prototype {
            header: header,
            uvs: uvs,
            constants: constants,
            symbols: symbols,
            instructions: bcis,
            proto_children: child_protos,
        }
    }

    fn get_symbols(ljr: &mut LJReader, header: &PrototypeHeader) -> Vec<String> {
        if let Some(dih) = &header.dbg_info_header {
            //separate dbg info based on dbg size.
            //read line num section
            //if there are any remaining bytes, they are the symbols most likely.
            unimplemented!()
        } else {
            Prototype::generate_symbols(header)
        }
    }

    fn generate_symbols(header: &PrototypeHeader) -> Vec<String> {
        let mut symbols: Vec<String> = Vec::new();
        for i in 0..header.frame_size {
            symbols.push(String::from(format!("var_pt{}_{}", header.id, i)));
        }
        symbols
    }

    fn read_kns(ljr: &mut LJReader, header: &PrototypeHeader) -> Vec<LuaValue> {
        let mut kns: Vec<LuaValue> = vec![];

        for _ in 0..header.size_kn {
            kns.push(ljr.read_kn());
        }
        kns
    }

    fn read_kgcs(ljr: &mut LJReader, header: &PrototypeHeader) -> Vec<LuaValue> {
        let mut kgcs: Vec<LuaValue> = vec![];

        for _ in 0..header.size_kgc {
            kgcs.push(ljr.read_kgc());
        }
        kgcs
    }

    fn read_raw_upvalues(ljr: &mut LJReader, header: &PrototypeHeader) -> Vec<UpValue> {
        let mut raw_uvs: Vec<UpValue> = vec![];

        for _ in 0..header.size_uv {
            raw_uvs.push(Prototype::read_raw_upvalue(ljr));
        }
        raw_uvs
    }

    fn read_raw_upvalue(ljr: &mut LJReader) -> UpValue {
        let uv = ljr.read_bytes(UpValue::UPVALUE_SIZE as usize);

        UpValue {
            table_index: uv[0],
            table_location: uv[1]
        }
    }

    ///Reads the bytecode instructions section of the prototype chunk.
    fn read_instructions(ljr: &mut LJReader, header: &PrototypeHeader) -> Vec<BytecodeInstruction> {
        let mut bcis: Vec<BytecodeInstruction> = vec![];

        for i in 0..header.instruction_count {
            bcis.push(Prototype::read_instruction(ljr, i as usize));
        }
        bcis
    }

    fn read_instruction(ljr: &mut LJReader, index: usize) -> BytecodeInstruction {
        let instr_bytes = ljr.read_bytes(BytecodeInstruction::INSTRUCTION_SIZE as usize);
        BytecodeInstruction::new(
            index,
            instr_bytes[0], //op
            instr_bytes[1], //a
            instr_bytes[2], //c
            instr_bytes[3]  //b
        )
    }

    fn read_header(ljr: &mut LJReader, ptr: &Prototyper) -> PrototypeHeader {
        let mut pth = PrototypeHeader {
            id: ptr.next_id,
            flags: ljr.read_byte(),
            num_params: ljr.read_byte(),
            frame_size: ljr.read_byte(),
            size_uv: ljr.read_byte(),
            size_kgc: ljr.read_uleb(),
            size_kn: ljr.read_uleb(),
            instruction_count: ljr.read_uleb(),
            dbg_info_header: None,
        };

        if ptr.ljfh.file_debug_flags & 0x02 == 0 {
            pth.dbg_info_header = Prototype::read_dbg_header(ljr);
        }
        pth
    }

    fn read_dbg_header(ljr: &mut LJReader) -> Option<DebugInfoHeader> {
        let dbg_size = ljr.read_uleb();
        if dbg_size > 0 {
            Some(DebugInfoHeader {
                size_dbg: dbg_size,
                first_line: ljr.read_uleb(),
                num_lines: ljr.read_uleb(),
            })
        } else { None }
    }
}

pub struct Prototyper {
    next_id: usize,
    reader: LJFileReader,
    ljfh: LuajitFileHeader,
    proto_id_stack: Vec<usize>,
}

impl Prototyper {
    pub fn new(file_path: &str) -> Prototyper {
        let mut reader = LJFileReader::new(file_path);
        assert!(0x1b4c4a01 == u32::from_be_bytes(
            [
                reader.read_byte(), reader.read_byte(),
                reader.read_byte(), reader.read_byte()
            ]
        ));

        let dbg_flags = reader.read_byte();
        let mut file_name: Option<String> = None;
        if dbg_flags == 0 {
            let file_name_len = reader.read_uleb();
            file_name = Some(String::from_utf8(reader.read_bytes(file_name_len as usize)).expect("Original file name could not be read.").replace("@", ""));
        }

        let ljfh = LuajitFileHeader {
            file_debug_flags: dbg_flags,
            file_name: file_name,
        };
        
        Prototyper {
            next_id: 0,
            reader: reader,
            ljfh: ljfh,
            proto_id_stack: vec![],
        }
    }

    pub fn next(&mut self) -> Prototype {
        let raw = self.get_raw_prototype();
        Prototype::new(self, raw)
    }

    fn get_raw_prototype(&mut self) -> Vec<u8> {
        let prototype_size = self.reader.read_uleb();
        self.reader.read_bytes(prototype_size as usize)
    }
}

#[cfg(test)]
mod tests {
    //use std::fs::File;
    //use std::io::Write;

    use super::*;

    #[test]
    fn test_new_prototyper() {
        let ptr = Prototyper::new("singleif.ljc");
        assert!(ptr.ljfh.file_name == None);
        assert!(ptr.ljfh.file_debug_flags != 0);
    }

    #[test]
    fn test_next_prototype() {
        let mut ptr = Prototyper::new("singleif.ljc");
        let pt = ptr.next();
        assert!(pt.header.id == 0);
    }

    /*fn dump_prototype(pt: &Vec<u8>) {
        let mut f = File::create("pt_dump").unwrap();
        f.write_all(&pt);
    }*/

    /*#[test]
    fn test_dump_prototype() {
        let mut ptr = Prototyper::new("singleif.ljc");
        dump_prototype(&ptr.get_raw_prototype());
    }*/
}