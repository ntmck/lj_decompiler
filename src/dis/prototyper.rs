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
        let mut bcis = Prototype::read_instructions(&mut ljr, &header);
        let uvs = Prototype::read_raw_upvalues(&mut ljr, &header);
        let mut kgcs = Prototype::read_kgcs(&mut ljr, &header);
        let kns = Prototype::read_kns(&mut ljr, &header);
        let symbols = Prototype::read_symbols(&mut ljr, &header);

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

        let unconditional_jumps = Prototype::find_unconditional_jumps(&bcis);
        Prototype::set_gotos(&mut bcis, unconditional_jumps);

        Prototype {
            header: header,
            uvs: uvs,
            constants: constants,
            symbols: symbols,
            instructions: bcis,
            proto_children: child_protos,
        }
    }

    ///Returns the indices of goto instructions.
    fn find_unconditional_jumps(bcis: &Vec<BytecodeInstruction>) -> Vec<usize> {
        //For each comparison :: bci[i]
        //  bci[i+1] is an expected jmp.
        //  bci[bci[i+1].target - 1] is an expected jmp. (aka the target of the first expected jmp - 1)
        //  Any jump not expected is a goto.
        //  Note: This does not catch ALL gotos in original source code,
        //   but that is fine as equivalent code can still be reproduced without catching them all.
        let mut expected: Vec<bool> = vec![false; bcis.len()];
        for i in 0..bcis.len() {
            if bcis[i].op < 16 { //comparison ops.
                expected[i+1] = true;
                let target = (bcis[i+1].get_jump_target() - 1) as usize;
                expected[target] = true;
            }
        }

        let mut gotos: Vec<usize> = vec![];
        for i in 0..expected.len() {
            if !expected[i] && bcis[i].op == 84 || bcis[i].op == 48 { //gotos are either JMP or UCLO.
                gotos.push(i);
            }
        }
        gotos
    }

    ///Changes bytecode operation at given indices to GOTO (93).
    fn set_gotos(bcis: &mut Vec<BytecodeInstruction>, indices: Vec<usize>) {
        for i in indices.iter() {
            bcis[*i].op = 93;
        }
    }

    fn read_symbols(ljr: &mut LJReader, header: &PrototypeHeader) -> Vec<String> {
        if let Some(dih) = &header.dbg_info_header {
            let dbg_info: Vec<u8> = ljr.read_bytes(dih.size_dbg as usize);
            println!("dbg_len: {}", dbg_info.len());
            println!("{:#?}", dbg_info);
            let mut offset = 0;
            Prototype::read_line_num_section(header, dih, &dbg_info, &mut offset); //ignore return as line number info is unecessary at present.
            if offset < dbg_info.len() {
                Prototype::extract_symbols(&dbg_info, &mut offset)
            } else { Prototype::generate_symbols(header) }
        } else {
            Prototype::generate_symbols(header)
        }
    }

    fn read_line_num_section(header: &PrototypeHeader, dih: &DebugInfoHeader, dbg_info: &Vec<u8>, offset: &mut usize) -> Vec<u8> {
        let entry_size = Prototype::line_entry_size(dih.num_lines);
        let line_sec_size = 1 + (entry_size * (header.instruction_count - 1)) as usize;
        println!("entry size: {}, line_sec_size: {}", &entry_size, &line_sec_size);
        *offset += line_sec_size;
        dbg_info[0..line_sec_size].to_vec()
    }

    fn extract_symbols(dbg_info: &Vec<u8>, offset: &mut usize) -> Vec<String> {
        let mut symbols: Vec<String> = vec![];
        loop {
            if *offset >= dbg_info.len() - 1 { break; } // +1 since this section terminates in 0x00.
            let sym = Prototype::extract_symbol(&dbg_info, offset);
            println!("Symbol: {}", sym);
            symbols.push(sym);
        }
        symbols
    }

    fn extract_symbol(dbg_info: &Vec<u8>, offset: &mut usize) -> String {
        let mut utf8: Vec<u8> = vec![];
        loop {
            if dbg_info[*offset] == 0 { break; }
            utf8.push(dbg_info[*offset]);
            *offset += 1;
        }
        *offset += 3; //skip null terminator + 2 unknown bytes. Unknown bytes *could* be 2 ulebs...not 100% sure. -> lj_debug.c/ line:172 -> line:176
        String::from_utf8(utf8).expect("Failed to convert symbol to utf8.")
    }

    fn line_entry_size(num_lines: u32) -> u32 {
        match num_lines {
            size if size < u8::MAX.into() => 1,
            size if size < u16::MAX.into() => 2,
            size if size < u32::MAX => 4,
            _ => panic!("Size of num_lines exceeds u32!"),
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

        //header checking
        assert!(pt.header.id == 0);
        assert!(pt.header.flags == 2);
        assert!(pt.header.num_params == 0);
        assert!(pt.header.frame_size == 2);
        assert!(pt.header.size_uv == 0);
        assert!(pt.header.size_kgc == 1);
        assert!(pt.header.size_kn == 0);
        assert!(pt.header.instruction_count == 22);
        assert!(pt.header.dbg_info_header.is_none());

        //prototype checking
        assert!(pt.constants.strings[0] == "print");
    }
}