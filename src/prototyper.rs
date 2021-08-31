// Sets up the prototype structures for the file using ljc_reader.rs to read the file.

use crate::prototype::*;
use crate::ljc_reader::LjcReader;
use crate::bytecode_instruction::BytecodeInstruction;
use crate::lua_table::LuaValue;

//unit test imports
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::Read;

pub struct Prototyper {
    prototypes: Vec<Prototype>, //prototypes[proto.id]
    proto_id_stack: Vec<usize>,
    ljcr: LjcReader,
}

impl Prototyper {
    pub fn new(file_path: &str) -> Prototyper {
        Prototyper {
            prototypes: Vec::new(),
            proto_id_stack: Vec::new(),
            ljcr: LjcReader::new(file_path),
        }
    }

    pub fn start(&mut self) {
        let ljfh = self.read_luajit_file_header();
        self.read_prototypes(&ljfh)
    }

    fn read_luajit_file_header(&mut self) -> LuajitFileHeader {
        let expected_magic = 0x1b4c4a01;
        let magic = u32::from_be_bytes([self.ljcr.read_byte(), self.ljcr.read_byte(), self.ljcr.read_byte(), self.ljcr.read_byte()]);
        assert!(magic == expected_magic, "Did not encounter expected luajit magic numbers. expected: {:#?}, actual: {:#?}", expected_magic, magic);
        let file_debug_flags = self.ljcr.read_byte();
        let mut file_name: Option<String> = None;
        if file_debug_flags == 0 {
            let file_name_len = self.ljcr.read_uleb();
            file_name = Some(String::from_utf8(self.ljcr.read_bytes(file_name_len as usize)).expect("Original file name could not be read."));
        }

        LuajitFileHeader {
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
    fn read_prototype_debug_header(&mut self, prototype: &mut Prototype, ljfh: &LuajitFileHeader) {
        if ljfh.file_debug_flags & 0x02 == 0 {
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

    fn read_prototypes(&mut self, ljfh: &LuajitFileHeader) {
        let mut i = 0;
        loop {
            let proto_size = self.ljcr.read_uleb();
            if proto_size <= 0 { break; }
            self.read_prototype(i, proto_size, ljfh);
            i += 1;
        }
    }

    fn read_prototype(&mut self, proto_index: usize, proto_size: u32, ljfh: &LuajitFileHeader) {
        let pth = self.read_prototype_header(proto_size);
        let pthc = pth;
        let mut pt = Prototype::new(proto_index, pth);
        print!("dbg_header\n");
        self.read_prototype_debug_header(&mut pt, ljfh);

        //DEBUG: Remove derives in Prototype after done as well as all the prints here.
        print!("{:#?}\n", pthc);

        print!("instructions\n");
        self.read_instructions(&mut pt);

        print!("upvalues\n");
        self.read_upvalues(&mut pt);

        print!("kgcs\n");
        self.read_kgcs(&mut pt);

        print!("kns\n");
        self.read_kns(&mut pt);

        print!("dbg_info\n");
        self.read_debug_info(&mut pt);

        print!("done\n");
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
                let instr_bytes = self.ljcr.read_bytes(BytecodeInstruction::INSTRUCTION_SIZE as usize);
                let bci = BytecodeInstruction::new(
                    instr_bytes[0], //op
                    instr_bytes[1], //a
                    instr_bytes[2], //c
                    instr_bytes[3]  //b
                );
                prototype.instructions.as_mut().unwrap().push(bci);
                i += BytecodeInstruction::INSTRUCTION_SIZE as u32;
            }
        }
    }

    fn read_upvalues(&mut self, prototype: &mut Prototype) {
        if prototype.header.as_ref().unwrap().size_uv > 0 {
            prototype.up_values = Some(Vec::new());
            let mut i = 0;
            let uv_len = prototype.header.as_ref().unwrap().size_uv * 2; //upvalues are byte pairs.
            while i < uv_len {
                let uv = self.ljcr.read_bytes(2);
                prototype.up_values.as_mut().unwrap().push(UpValue {
                    table_index: uv[0],
                    table_location: uv[1]
                });
                i += 2;
            }
        }
    }

    fn read_kgcs(&mut self, prototype: &mut Prototype) {
        if prototype.header.as_ref().unwrap().size_kgc > 0 {
            if let None = prototype.constants_table { prototype.constants_table = Some(Vec::new()); }
            let mut i = 0;
            let kgc_len = prototype.header.as_ref().unwrap().size_kgc;
            while i < kgc_len {
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
                i += 1;
            }
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

    fn read_debug_info(&mut self, prototype: &mut Prototype) {
        self.seek_to_symbol_names(prototype);
        unimplemented!();
    }

    //BUG: read to dbg_size+1 (i think?). Line numbers always present with dbg info. symbols are OPTIONALLY HERE.
    fn seek_to_symbol_names(&mut self, prototype: &Prototype) {

        let num_lines = prototype.header.as_ref().unwrap().dbg_info_header.as_ref().unwrap().num_lines;
        //Find the end of the line numbers.
        print!("Hey we are probably about to blow up.\n");
        print!("num_lines: {}\n", num_lines);
        print!("remaining_bytes: {}\n", self.ljcr.remaining_bytes());
        while self.ljcr.read_byte() as u32 != num_lines {()}
        //Last line_num may be duplicated.
        while self.ljcr.peek_byte() as u32 == num_lines { self.ljcr.read_byte(); }
    }

    fn collect_symbols(&mut self, prototype: &mut Prototype) {
        prototype.symbols = Some(Vec::new());
        while self.ljcr.remaining_bytes() > 0 {
            prototype.symbols.as_mut().unwrap().push(self.collect_symbol());
        }
    }

    fn collect_symbol(&mut self) -> String {
        let mut utf8: Vec<u8> = Vec::new();
        while let byte = self.ljcr.read_byte() {
            if byte == 0 {
                self.ljcr.read_bytes(2); //skip over 2 bytes of unknown data.
                break;
            } else {
                utf8.push(byte);
            }
        }
        self.recover_and_clean_utf8_name(&mut utf8)
    }

    fn recover_and_clean_utf8_name(&mut self, utf8: &mut Vec<u8>) -> String {
        utf8.retain(|&x| x > 32); //clean by removing any utf8 value less than alpha/numeric char.
        String::from_utf8(utf8.to_vec()).expect("Variable name could not be read.")
    }
}

///Unit Tests

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

#[test]
fn test_prototype() {
    setup_mock_ljc_file();
    let mut ptr = Prototyper::new("mock.ljc");
    ptr.start();
}
