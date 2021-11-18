// Sets up the prototype structures for the file using ljc_reader.rs to read the file.

//TODO:
// Setup Symbols Table hashmap where: location->symbol
// For debug printout of diassembled code, replace BCI's opcode value with op name.

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::prototype::*;
use crate::ljc_reader::LjcReader;
use crate::bytecode_instruction::BytecodeInstruction;
use crate::lua_table::LuaValue;

use std::io::Cursor;

pub struct Prototyper {
    pub ljfh: LuajitFileHeader,
    pub prototypes: Vec<Prototype>, //prototypes[proto.id]
    proto_id_stack: Vec<usize>,
    ljcr: LjcReader,
}

impl Prototyper {
    pub fn new(file_path: &str) -> Prototyper {
        Prototyper {
            ljfh: LuajitFileHeader{magic: 0, file_debug_flags: 0, file_name: None},
            prototypes: Vec::new(),
            proto_id_stack: Vec::new(),
            ljcr: LjcReader::new(file_path),
        }
    }

    pub fn start(&mut self) {
        self.read_luajit_file_header();
        self.read_prototypes();
    }

    fn read_luajit_file_header(&mut self) {
        let expected_magic = 0x1b4c4a01;
        let magic = u32::from_be_bytes([self.ljcr.read_byte(), self.ljcr.read_byte(), self.ljcr.read_byte(), self.ljcr.read_byte()]);
        assert!(magic == expected_magic, "Did not encounter expected luajit magic numbers. expected: {:#?}, actual: {:#?}", expected_magic, magic);
        let file_debug_flags = self.ljcr.read_byte();
        let mut file_name: Option<String> = None;
        if file_debug_flags == 0 {
            let file_name_len = self.ljcr.read_uleb();
            file_name = Some(String::from_utf8(self.ljcr.read_bytes(file_name_len as usize)).expect("Original file name could not be read.").replace("@", ""));
        }

        self.ljfh = LuajitFileHeader {
            magic: magic,
            file_debug_flags: file_debug_flags,
            file_name: file_name,
        }
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

    //Reads the prototype's debug header if the file has debug info present into the prototype.
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
        self.read_kgcs(&mut pt);
        self.read_kns(&mut pt);
        print!("start\n");
        self.read_debug_info(&mut pt); //TODO: Bug is here somewhere i think...
        print!("end\n");
        self.prototypes.push(pt);
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
            prototype.up_values = Some(Vec::new());
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
        prototype.up_values.as_mut().unwrap().push(UpValue {
            table_index: uv[0],
            table_location: uv[1]
        });
    }

    fn read_kgcs(&mut self, prototype: &mut Prototype) {
        if prototype.header.as_ref().unwrap().size_kgc > 0 {
            if let None = prototype.constants_table { prototype.constants_table = Some(Vec::new()); }
            let mut i = 0;
            let kgc_len = prototype.header.as_ref().unwrap().size_kgc;
            while i < kgc_len {
                self.read_kgc(prototype);
                i += 1;
            }
        }
    }

    fn read_kgc(&mut self, prototype: &mut Prototype) {
        let kgc = self.ljcr.read_kgc();
        match kgc {
            LuaValue::Nil => {
                let child = self.proto_id_stack.pop().expect("Tried to pop empty proto stack.");
                self.prototypes[child as usize].proto_parent = Some(prototype.id);
                if let None = prototype.proto_children { prototype.proto_children = Some(Vec::new()) }
                prototype.proto_children.as_mut().unwrap().push(child);
            },
            _ => prototype.constants_table.as_mut().unwrap().push(kgc),
        }
    }

    fn read_kns(&mut self, prototype: &mut Prototype) {
        if prototype.header.as_ref().unwrap().size_kn > 0 {
            if let None = prototype.constants_table { prototype.constants_table = Some(Vec::new()); }
            let mut i = 0;
            let kn_len = prototype.header.as_ref().unwrap().size_kn;
            while i < kn_len {
                prototype.constants_table.as_mut().unwrap().push(self.ljcr.read_kn());
                i += 1;
            }
        }
    }

    ///! Dbg info is separated into 2 sections:
    ///!  The Line Numbers section
    ///!     Line numbers are stored from first_line -> header.num_lines with a byte size that fits
    ///!     Line numbers appear to be separated into chunks delimited by 0x00 if the entry_size > 1.
    ///!  The Symbols section
    ///!      Contains variable names delimited by 0x00 and 2 unknown bytes.
    fn read_debug_info(&mut self, prototype: &mut Prototype) {
        self.read_line_num_section(prototype);
        if self.ljcr.remaining_bytes() != 1 { 
            self.read_symbols(prototype);
        }  
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
        let mut entry: u64 = 0;
        let mut stop = false;
        loop {
            if self.ljcr.remaining_bytes() == 1 { break; }

            let mut cursor = Cursor::new(self.ljcr.peek_bytes(entry_size as usize));
            let peek = cursor.read_uint::<LittleEndian>(entry_size as usize).unwrap();

            //last entry in the section. it may be repeated an unknown number of times. right after it, is the symbols section if any. (usually in case: entry_size == 1)
            if peek == num_lines as u64 {
                loop {
                    let mut cursor = Cursor::new(self.ljcr.peek_bytes(entry_size as usize));
                    let peek = cursor.read_uint::<LittleEndian>(entry_size as usize).unwrap();
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
            if (self.ljcr.remaining_bytes() == 1 || self.ljcr.peek_byte() == 0) { break; }
            symbols.push(self.read_symbol());
        }
        prototype.symbols = Some(symbols);
    }

    fn read_symbol(&mut self) -> String {
        let mut utf8: Vec<u8> = Vec::new();
        loop {
            if (self.ljcr.peek_byte() == 0) { break; }
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
        let mut ptr = Prototyper::new("mock.ljc");
        ptr.start();
        //debug_write_file(&ptr);
    }

    #[test]
    fn test_large_ljc() {
        let mut ptr = Prototyper::new("large_ljc_mock.ljc");
        ptr.start();
        //debug_write_file(&ptr);
    }

    #[test]
    fn test_symbols_ljc() {
        let mut ptr = Prototyper::new("_condi.ljc");
        ptr.start();
        debug_write_file(&ptr);
    }
}


