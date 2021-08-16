// Sets up the prototype structures for the file using ljc_reader.rs to read the file.

use crate::prototype::*;
use crate::ljc_reader::LjcReader;

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

    fn read_prototypes(&mut self, ljfh: &LuajitFileHeader) {
        for i in 0..ljfh.proto_count {
            self.read_prototype();
        }
    }

    fn read_prototype(&mut self) {
        unimplemented!();
    }

    fn read_prototype_header(&self) {//-> PrototypeHeader {
        unimplemented!();
    }
}
