// Reads the entire luajit compiled file and allows reading of the bytes as a stream.

//TODO: make a file/struct for holding dynamic values and their type.
use std::vec::Vec;

use crate::lua_table::*;

//unit test imports
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::Read;

pub struct LjcReader {
    file: Vec<u8>,
    pub offset: u64,
}

impl LjcReader {
    pub fn new(file_path: &str) -> LjcReader {
        let mut file = File::open(file_path).expect(&format!("File not found: {}", file_path));
        let file_meta = std::fs::metadata(file_path).expect(&format!("Metadata for file {} could not be read.", file_path));
        let mut buf = vec![0; file_meta.len() as usize];
        file.read(&mut buf).expect(&format!("Buffer overflow for file: {}. meta_len: {}, buf_len: {}", file_path, file_meta.len(), buf.len()));
        let file_start = LjcReader::seek_to_luajit_magic(&buf);
        LjcReader {
            file: buf,
            offset: file_start,
        }
    }

    //Find the luajit magic numbers and seek past them ending up at the debug info flag.
    fn seek_to_luajit_magic(file: &Vec<u8>) -> u64 {
        for i in 0..file.len() {
            if file[i..i+4] == [0x1b, 0x4c, 0x4a, 0x01] {
                return i as u64;
            }
        }
        panic!("LJ Magic not found.");
    }

    pub fn remaining_bytes(&self) -> u64 {
        self.file.len() as u64 - self.offset
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        assert!((self.offset as usize) < self.file.len(), "LjcReader::read_bytes() -> Offset is equal to or greater than file length. offset: {}, len: {}", self.offset, self.file.len());
        let result = self.file[self.offset as usize..self.offset as usize + count].to_vec();
        self.offset += count as u64;
        result
    }

    pub fn read_byte(&mut self) -> u8 {
        self.read_bytes(1)[0]
    }

    pub fn peek_bytes(&mut self, count: usize) -> Vec<u8> {
        let bytes = self.read_bytes(count);
        self.offset -= count as u64;
        bytes
        //self.file[self.offset as usize .. self.offset as usize + count].to_vec()
    }

    pub fn peek_byte(&mut self) -> u8 {
        let byte = self.read_bytes(1)[0];
        self.offset -= 1 as u64;
        byte
        //self.file[self.offset as usize + 1]
    }

    pub fn read_uleb(&mut self) -> u32 {
        let mut count = 0;
        let mut value: u32 = 0;
        let mut shift = 1;
        loop {
            let byte = self.read_byte();
            let data = byte as u32 & 127;
            let cont = byte as u32 & 128;
            value += data * shift;
            shift *= 128;
            if cont == 0 { break; }
        }
        value
    }

    //Read a luajit number constant.
    pub fn read_kn(&mut self) -> LuaValue {
        let mut kn_a = self.read_uleb();
        let is_a_double = (kn_a & 1) > 0;
        kn_a >>= 1;
        if is_a_double {
            let kn_b = self.read_uleb();
            let mut kn_union: u64 = kn_a as u64;
            kn_union <<= 16;
            kn_union |= kn_b as u64;
            return LuaValue::Double(kn_union as f64)
        } else {
            return LuaValue::UInt(kn_a)
        }
    }

    //Read a luajit global constant as type, value
    pub fn read_kgc(&mut self) -> LuaValue {
        let type_byte = self.read_byte();
        //let type_byte = self.read_uleb();
        match type_byte {
            0   => LuaValue::ChildProto, //signal that the prototyper needs to handle a child prototype by popping from the id stack and setting up parent/child relationship between the 2 prototypes.
            1   => LuaValue::Table(self.read_lua_table()), //add table constant -> array_part_len = uleb, hash_part_len = uleb, see TableConstant for more details.
            2   => LuaValue::SInt(self.read_uleb() as i32),
            3   => LuaValue::UInt(self.read_uleb()),
            4   => LuaValue::ComplexNum(self.read_complex_lua_number()),
            x   => LuaValue::Str(self.read_lua_string((x-5) as usize)),
        }
    }

    //returns type, value
    pub fn read_table_value(&mut self) -> LuaValue {
        let type_byte = self.read_byte();
        //let type_byte = self.read_uleb();
        match type_byte {
            0   => LuaValue::Nil,
            1   => LuaValue::False,
            2   => LuaValue::True,
            3   => LuaValue::UInt(self.read_uleb()),
            4   => LuaValue::ComplexNum(self.read_complex_lua_number()),
            x   => LuaValue::Str(self.read_lua_string((x-5) as usize)),
        }
    }

    pub fn read_lua_table(&mut self) -> LuaTable {
        let array_part_len = self.read_uleb();
        let hash_part_len = self.read_uleb();
        let mut array_part = ArrayPart {
            values: Vec::new(),
        };
        let mut hash_part = HashPart {
            keys: Vec::new(),
            values: Vec::new(),
        };
        self.read_table_array_part(&mut array_part, array_part_len as usize);
        self.read_table_hash_part(&mut hash_part, hash_part_len as usize);
        LuaTable::new(array_part, hash_part)
    }

    fn read_table_array_part(&mut self, array_part: &mut ArrayPart, len: usize) {
        for _ in 0..len {
            array_part.values.push(self.read_table_value());
        }
    }

    fn read_table_hash_part(&mut self, hash_part: &mut HashPart, len: usize) {
        for _ in 0..len {
            hash_part.keys.push(self.read_table_value());
            hash_part.values.push(self.read_table_value());
        }
    }

    fn read_complex_lua_number(&mut self) -> (u32, u32) {
        (self.read_uleb(), self.read_uleb()) //I think that it is in the form: XeY where X = first uleb, Y = second uleb. X may be a 32bit float and Y may be an integer.
    }

    fn read_lua_string(&mut self, len: usize) -> String {
        assert!(len > 0, "LjcReader::read_lua_string() -> Cannot read string length of 0 or less.");
        let utf8_string = self.read_bytes(len);
        String::from_utf8(utf8_string).expect("Lua string could not be read.")
    }
}
