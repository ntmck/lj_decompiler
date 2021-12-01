// Sets up the prototype structures for the file using ljc_reader.rs to read the file.

use std::io::Cursor;
use std::collections::VecDeque;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use super::ljc_reader::LjcReader;
use super::bytecode_instruction::BytecodeInstruction;
use super::lua_table::LuaValue;

// Start Prototype Definition

/// LuaJit File Format Format:
///
/// TODO:
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

// End Prototype Definition

pub struct Prototyper {
    pub prototypes: VecDeque<Prototype>,
    ljfh: LuajitFileHeader,
    proto_id_stack: Vec<usize>,
    ljcr: LjcReader,
}

impl Prototyper {
    pub fn new(file_path: &str) -> Prototyper {
        let mut ptr = Prototyper {
            prototypes: VecDeque::new(),
            ljfh: LuajitFileHeader{magic: 0, file_debug_flags: 0, file_name: None},
            proto_id_stack: Vec::new(),
            ljcr: LjcReader::new(file_path),
        };
        ptr.start();
        ptr
    }

    fn start(&mut self) {
        self.read_luajit_file_header();   
        self.read_prototypes();
        self.bind_uvs_to_symbol();
    }

    fn read_luajit_file_header(&mut self) {
        let expected_magic = 0x1b4c4a01;
        let magic = u32::from_be_bytes([self.ljcr.read_byte(), self.ljcr.read_byte(), self.ljcr.read_byte(), self.ljcr.read_byte()]);
        
        assert!(magic == expected_magic, "Did not encounter expected luajit magic numbers. expected: {:#?}, actual: {:#?}", expected_magic, magic);
        let file_debug_flags = self.ljcr.read_byte();
        
        let mut file_name: Option<String> = None;
        if file_debug_flags == 0 {
            file_name = Some(self.read_luajit_file_name());
        }
        
        self.ljfh = LuajitFileHeader {
            magic: magic,
            file_debug_flags: file_debug_flags,
            file_name: file_name,
        }
    }

    fn read_luajit_file_name(&mut self) -> String {
        let file_name_len = self.ljcr.read_uleb();
        String::from_utf8(self.ljcr.read_bytes(file_name_len as usize)).expect("Original file name could not be read.").replace("@", "")
    }

    fn read_prototype_header(&mut self, proto_size: u32) -> PrototypeHeader {
        PrototypeHeader {
            flags: self.ljcr.read_byte(),
            num_params: self.ljcr.read_byte(),
            frame_size: self.ljcr.read_byte(),
            size_uv: self.ljcr.read_byte(),
            size_kgc: self.ljcr.read_uleb(),
            size_kn: self.ljcr.read_uleb(),
            instruction_count: self.ljcr.read_uleb(),
            prototype_size: proto_size,
            dbg_info_header: None,
        }
    }

    //Reads the prototype's debug header if the file has debug info present in the prototype.
    fn read_prototype_debug_header(&mut self, prototype: &mut Prototype) {
        if self.ljfh.file_debug_flags & 0x02 == 0 {
            let dbg_size = self.ljcr.read_uleb();
            if dbg_size > 0 {
                prototype.header
                    .as_mut().unwrap()
                    .dbg_info_header = Some(DebugInfoHeader {
                        size_dbg: dbg_size,
                        first_line: self.ljcr.read_uleb(),
                        num_lines: self.ljcr.read_uleb(),
                    });
            }
        }
    }

    fn read_prototypes(&mut self) {
        let mut i = 0;
        loop {
            if self.ljcr.remaining_bytes() == 0 { break; }
            let proto_size = self.ljcr.read_uleb();
            if proto_size <= 0 { break; }
            self.read_prototype(i, proto_size);
            i += 1;
        }
    }

    fn read_prototype(&mut self, proto_index: usize, proto_size: u32) {
        let pth = self.read_prototype_header(proto_size);
        let mut pt = Prototype::new(proto_index, pth);
        self.read_prototype_debug_header(&mut pt);
        self.read_instructions(&mut pt);    
        self.read_upvalues(&mut pt);

        let mut temp_constants: Vec<LuaValue> = Vec::new();
        self.read_kgcs(&mut pt, &mut temp_constants);
        self.read_kns(&mut pt, &mut temp_constants);
        pt.constants = Some(Constants::new(temp_constants));

        self.read_debug_info(&mut pt);
        self.prototypes.push_back(pt);
        self.proto_id_stack.push(proto_index);
    }

    //Reads bytecode instructions into the prototype.
    fn read_instructions(&mut self, prototype: &mut Prototype) {
        if prototype.header.as_ref().unwrap().instruction_count > 0 {
            prototype.instructions = Some(Vec::new());
            let mut i = 0;
            let instr_len = prototype.header.as_ref().unwrap().instruction_count * BytecodeInstruction::INSTRUCTION_SIZE as u32;
            while i < instr_len {
                self.read_instruction(prototype, BytecodeInstruction::INSTRUCTION_SIZE as usize);
                i += BytecodeInstruction::INSTRUCTION_SIZE as u32;
            }
        }
    }

    fn read_instruction(&mut self, prototype: &mut Prototype, instruction_size: usize) {
        let instr_bytes = self.ljcr.read_bytes(instruction_size);
        let bci = BytecodeInstruction::new(
            instr_bytes[0], //op
            instr_bytes[1], //a
            instr_bytes[2], //c
            instr_bytes[3]  //b
        );
        prototype.instructions.as_mut().unwrap().push(bci);
    }

    fn read_upvalues(&mut self, prototype: &mut Prototype) {
        if prototype.header.as_ref().unwrap().size_uv > 0 {
            prototype.raw_uvs = Some(Vec::new());
            let mut i = 0;
            let uv_len = prototype.header.as_ref().unwrap().size_uv * 2; //upvalues are byte pairs.
            while i < uv_len {
                self.read_upvalue(prototype);
                i += 2;
            }
        }
    }

    fn read_upvalue(&mut self, prototype: &mut Prototype) {
        let uv = self.ljcr.read_bytes(2); //upvalues are byte pairs.
        prototype.raw_uvs.as_mut().unwrap().push(UpValue {
            table_index: uv[0],
            table_location: uv[1]
        });
    }

    fn bind_uvs_to_symbol(&mut self) {
        let n = self.prototypes.len();
        for i in 0..n {
            if let None = self.prototypes[i].raw_uvs { continue; }
            self.bind_uvs(i);
        }
    }

    fn bind_uvs(&mut self, pt_id: usize) {
        let mut bound_uvs: Vec<String> = Vec::new();

        let n = self.prototypes[pt_id].raw_uvs.as_ref().unwrap().len();
        for i in 0..n {
            let uv = &self.prototypes[pt_id].raw_uvs.as_ref().unwrap()[i];
            bound_uvs.push(self.recursive_find_uv_symbol(pt_id, uv))
        }

        self.prototypes[pt_id].bound_uvs = Some(bound_uvs);
    }

    fn recursive_find_uv_symbol(&self, pt_id: usize, uv: &UpValue) -> String {
        if let None = self.prototypes[pt_id].proto_parent.as_ref() {
            panic!("UpValue cannot be determined.");
        }
        let parent_id = self.prototypes[pt_id].proto_parent.unwrap();
        //the first bit of 192 determines if we look at the constants section table or not.
        //the second bit of 192 means if it is mutable or not. 1 = immutable upvalue. Disregarding the mutable/immutable bit for now.
        if uv.table_location & 0x80 == 0x80 {
            if let Some(symbols) = self.prototypes[parent_id].symbols.as_ref() {
                return format!("{}", symbols[uv.table_index as usize]) //*should be a matching index for dst register in symbols table and constants table...
            } else {
                return String::from(format!("uv_{}_{}", self.prototypes[parent_id].id, uv.table_index))
            }
        }

        self.recursive_find_uv_symbol(parent_id, &self.prototypes[parent_id].raw_uvs.as_ref().unwrap()[uv.table_index as usize])
    }

    fn read_kgcs(&mut self, prototype: &mut Prototype, temp_constants: &mut Vec<LuaValue>) {
        if prototype.header.as_ref().unwrap().size_kgc > 0 {
            let mut i = 0;
            let kgc_len = prototype.header.as_ref().unwrap().size_kgc;
            while i < kgc_len {
                self.read_kgc(prototype, temp_constants);
                i += 1;
            }
        }
    }

    fn read_kgc(&mut self, prototype: &mut Prototype, temp_constants: &mut Vec<LuaValue>) {
        let kgc = self.ljcr.read_kgc();
        match kgc {
            LuaValue::ChildProto => { // Establish child/parent relationship between prototypes based on the stack.
                let child = self.proto_id_stack.pop().expect("Tried to pop empty proto stack.");
                self.prototypes[child as usize].proto_parent = Some(prototype.id);
                if let None = prototype.proto_children { prototype.proto_children = Some(Vec::new()) }
                prototype.proto_children.as_mut().unwrap().push(child);
            },
            _ => temp_constants.push(kgc),
        }
    }

    fn read_kns(&mut self, prototype: &mut Prototype, temp_constants: &mut Vec<LuaValue>) {
        if prototype.header.as_ref().unwrap().size_kn > 0 {
            let mut i = 0;
            let kn_len = prototype.header.as_ref().unwrap().size_kn;
            while i < kn_len {
                temp_constants.push(self.ljcr.read_kn());
                i += 1;
            }
        }
    }

    /// Dbg info is separated into 2 sections:
    /// The Line Numbers section
    ///     Line numbers are stored from first_line -> header.num_lines with a byte size that fits
    ///     Line numbers appear to be separated into chunks delimited by 0x00 if the entry_size > 1.
    ///  The Symbols section
    ///      Contains variable names delimited by 0x00 and 2 unknown bytes.
    fn read_debug_info(&mut self, prototype: &mut Prototype) {
        let mut has_dbg_info = true;
        if let None = prototype.header.as_ref().unwrap().dbg_info_header.as_ref() { has_dbg_info = false; }
        if !has_dbg_info {
            self.generate_symbols(prototype); 
            return 
        }
        self.read_line_num_section(prototype);
        if self.ljcr.remaining_bytes() != 1 { 
            self.read_symbols(prototype);
        }
    }

    fn generate_symbols(&mut self, prototype: &mut Prototype) {
        let n = prototype.header.as_ref().unwrap().frame_size as usize;
        let mut symbols: Vec<String> = Vec::new();
        for i in 0..n {
            symbols.push(String::from(format!("var_pt{}_{}", prototype.id, i)));
        }
        prototype.symbols = Some(symbols);
    }

    //Read through the line number section, but skip it for now.
    fn read_line_num_section(&mut self, prototype: &Prototype) {
        let num_lines = prototype.header.as_ref().unwrap().dbg_info_header.as_ref().unwrap().num_lines;
        let entry_size = self.line_entry_size(num_lines);

        let mut chunk_count = 1;
        if entry_size != 1 {
            chunk_count = (num_lines >> ((entry_size - 1) * 8)) + 1;
        }

        for _ in 0..chunk_count {
            self.read_line_num_chunk(num_lines, entry_size);
        }
    }
    
    fn read_line_num_chunk(&mut self, num_lines: u32, entry_size: u32) {
        let mut stop = false;
        loop {
            if self.ljcr.remaining_bytes() == 1 { break; }

            let peek = self.peek_entry(entry_size as usize);

            //last entry in the section. it may be repeated an unknown number of times. right after it, is the symbols section if any. (usually in case: entry_size == 1)
            if peek == num_lines as u64 {
                loop {
                    let peek = self.peek_entry(entry_size as usize);
                    if peek != num_lines as u64 { break; }
                    self.ljcr.read_bytes(entry_size as usize);
                }
                stop = true;
            }

            if stop { break; }

            //last part of the chunk is delimited by 0x00 when entry_size > 1.
            if self.ljcr.peek_byte() == 0 {
                self.ljcr.read_byte();
                break;
            }

            self.ljcr.read_bytes(entry_size as usize); //skip entries for now. use cursor + byteorder to parse if needed later.
        }
    }

    fn peek_entry(&mut self, count: usize) -> u64 {
        let mut cursor = Cursor::new(self.ljcr.peek_bytes(count));
        cursor.read_uint::<LittleEndian>(count).unwrap()
    }

    fn line_entry_size(&self, num_lines: u32) -> u32 {
        match num_lines {
            size if size <= u8::MAX.into() => 1,
            size if size <= u16::MAX.into() => 2,
            size if size <= 16777215 => 3, //u24::MAX.
            size if size <= u32::MAX => 4,
            _ => panic!("Size of num_lines exceeds u32!"),
        }
    }

    fn read_symbols(&mut self, prototype: &mut Prototype) {
        let mut symbols: Vec<String> = Vec::new();
        loop {
            if self.ljcr.remaining_bytes() == 1 || self.ljcr.peek_byte() == 0 { break; }
            symbols.push(self.read_symbol());
        }
        prototype.symbols = Some(symbols);
    }

    fn read_symbol(&mut self) -> String {
        let mut utf8: Vec<u8> = Vec::new();
        loop {
            if self.ljcr.peek_byte() == 0 { break; }
            utf8.push(self.ljcr.read_byte());
        }
        self.ljcr.read_bytes(3); // 2 unknown bytes + null terminator.
        String::from_utf8(utf8).expect("Failed to convert symbol to utf8.")
    }
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::fs::File;
    use std::io::Read;
    use std::io::prelude::*;

    use super::*;

    fn setup_mock_ljc_file() {
        File::create("mock.ljc").expect("Mock file could not be created.");
        let mut f = OpenOptions::new().read(true).write(true).open("mock.ljc").expect("File could not be opened.");
        let luajit_file_with_bs_header_and_debug_info: [u8; 93] = [
        0x55, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, //bitsquid stuff
        0x1B, 0x4C, 0x4A, 0x01, //magic
        0x00, //file debug flags
        0x2D, //len of string filename (45)
    
        //file name
        0x40, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x73, 0x2F, 0x67, 0x61,
        0x6D, 0x65, 0x2F, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73,
        0x2F, 0x67, 0x61, 0x6D, 0x65, 0x2F, 0x66, 0x6F, 0x6E, 0x74, 0x5F,
        0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x2E, 0x6C, 0x75, 0x61,
        //end file name
    
        0x20, //prototype len (32)
        0x02, //proto debug flags
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x04, 0x00, 0x03, 0x32, 0x00,
        0x00, 0x00, 0x35, 0x00, 0x00, 0x00, 0x47, 0x00, 0x01, 0x00, 0x0A, 0x46,
        0x6F, 0x6E, 0x74, 0x73, 0x01, 0x02, 0x02, 0x00, 0x00
    ];
        f.write(&luajit_file_with_bs_header_and_debug_info);
    }

    fn debug_write_file(prototyper: &Prototyper) {
        let mut file = File::create("debug.txt").unwrap();
        writeln!(&mut file, "{:#?}", prototyper.ljfh);
        for pt in prototyper.prototypes.iter() {
            writeln!(&mut file, "{:#?}", pt).unwrap();
        }
    }

    #[test]
    fn test_prototype() {
        setup_mock_ljc_file();
        let _ptr = Prototyper::new("mock.ljc");
        //debug_write_file(&ptr);
    }

    #[test]
    fn test_large_ljc() {
        let _ptr = Prototyper::new("large_ljc_mock.ljc");
        //debug_write_file(&ptr);
    }

    #[test]
    fn test_symbols_ljc() {
        let _ptr = Prototyper::new("_condi.ljc");
        //debug_write_file(&ptr);
    }

    #[test]
    fn test_upvalues_and_no_dbg_info_ljc() {
        let ptr = Prototyper::new("funcs.ljc");
        debug_write_file(&ptr);
    }
}
