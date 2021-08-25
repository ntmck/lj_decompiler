// Sets up the prototype structures for the file using ljc_reader.rs to read the file.

use crate::prototype::*;
use crate::ljc_reader::LjcReader;
use crate::bytecode_instruction::BytecodeInstruction;

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
        let debug_flags = self.ljcr.read_byte();
        let proto_count = self.ljcr.read_uleb();

        LuajitFileHeader {
            magic: magic,
            debug_flags: debug_flags,
            proto_count: proto_count,
        }
    }

    fn read_prototype_header(&mut self) -> PrototypeHeader {
        let proto_size = self.ljcr.read_uleb();
        PrototypeHeader {
            flags: self.ljcr.read_byte(),
            num_params: self.ljcr.read_byte(),
            frame_size: self.ljcr.read_byte(),
            size_uv: self.ljcr.read_byte(),
            size_kgc: self.ljcr.read_uleb(),
            size_kn: self.ljcr.read_uleb(),
            instruction_count: self.ljcr.read_uleb() * BytecodeInstruction::INSTRUCTION_SIZE as u32,
            prototype_size: proto_size,
            dbg_info_header: None,
        }
    }

    //Reads the prototype's debug header if the file has debug info present into the prototype.
    fn read_prototype_debug_header(&mut self, prototype: &mut Prototype, ljfh: &LuajitFileHeader) {
        if ljfh.debug_flags & 0x02 == 0 {
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
        for i in 0..ljfh.proto_count as usize {
            self.read_prototype(i, ljfh);
        }
    }

    fn read_prototype(&mut self, proto_index: usize, ljfh: &LuajitFileHeader) {
        let pth = self.read_prototype_header();
        let mut pt = Prototype::new(proto_index, pth);
        self.read_prototype_debug_header(&mut pt, ljfh);
        self.read_instructions(&mut pt);
        self.read_upvalues(&mut pt);
        self.read_kgcs(&mut pt);
        self.read_kns(&mut pt);
        self.read_debug_info(&mut pt);
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
            prototype.constants_table = Some(Vec::new());
            let mut i = 0;
            let kgc_len =  prototype.header.as_ref().unwrap().size_kgc;
            while i < kgc_len {
                let kgc = self.ljcr.read_kgc();
                if kgc.0 == 0 {
                    let child = self.proto_id_stack.pop().expect("Tried to pop empty proto stack.");
                    self.prototypes[child as usize].proto_parent = Some(prototype.id);
                    if let None = prototype.proto_children { prototype.proto_children = Some(Vec::new()) }
                    prototype.proto_children.as_mut().unwrap().push(child);
                } else {
                    prototype.constants_table.as_mut().unwrap().push(kgc);
                }
                i += 1;
            }
        }
    }

    fn read_kns(&mut self, prototype: &mut Prototype) {
        unimplemented!()
    }

    fn read_debug_info(&mut self, prototype: &mut Prototype) {
        unimplemented!()
    }
}
