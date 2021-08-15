// Reads the entire luajit compiled file and allows reading of the bytes as a stream.

use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::any::Any;

use crate::lua_table::*;

// Unit Test imports
use std::fs::OpenOptions;
use std::io::Write;

pub struct Reader {
    file: Vec<u8>,
    offset: u64,
}

impl Reader {
    pub fn new(file_path: &str) -> Reader {
        let mut file = File::open(file_path).expect(&format!("File not found: {}", file_path));
        let file_meta = std::fs::metadata(file_path).expect(&format!("Metadata for file {} could not be read.", file_path));
        let mut buf = vec![0; file_meta.len() as usize];
        file.read(&mut buf).expect(&format!("Buffer overflow for file: {}. meta_len: {}, buf_len: {}", file_path, file_meta.len(), buf.len()));
        Reader {
            file: buf,
            offset: 0,
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        self.read_bytes(1)[0]
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        assert!((self.offset as usize) < self.file.len(), "Reader::read_bytes() -> Offset is equal to or greater than file length.");
        let result = self.file[self.offset as usize..self.offset as usize + count].to_vec();
        self.offset += count as u64;
        result
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
            count += 1;
            if cont == 0 { break; }
        }
        self.offset += count;
        value
    }

    //Read a luajit global constant.
    pub fn read_kgc(&mut self) -> Option<Box<dyn Any>> {
        let type_byte = self.read_uleb();
        match type_byte {
            0   => return None, //signal that the prototyper needs to handle a child prototype by popping from the id stack and setting up parent/child relationship between the 2 prototypes.
            1   => return Some(Box::new(1 as u32)), //add table constant -> array_part_len = uleb, hash_part_len = uleb, see TableConstant for more details.
            2   => return Some(Box::new(self.read_uleb() as i32)),
            3   => return Some(Box::new(self.read_uleb() as u32)),
            4   => return Some(Box::new(self.read_complex_lua_number())),
            x   => return Some(Box::new(self.read_lua_string((x-5) as usize))),
        }
    }

    pub fn read_table_value(&mut self) -> Option<Box<dyn Any>> {
        let type_byte = self.read_uleb();
        match type_byte {
            0   => return None,
            1   => return Some(Box::new(false)),
            2   => return Some(Box::new(true)),
            3   => return Some(Box::new(self.read_uleb())),
            4   => return Some(Box::new(self.read_complex_lua_number())),
            x   => return Some(Box::new(self.read_lua_string((x-5) as usize))),
        }
    }

    pub fn read_lua_table(&mut self) -> LuaTable {
        let array_part_len = self.read_uleb();
        let hash_part_len = self.read_uleb();
        let mut array_part: ArrayPart = None;
        let mut hash_part: HashPart = None;

        if array_part_len > 0 {
            array_part = self.read_table_array_part(array_part_len as usize);
        }
        if hash_part_len > 0 {
            hash_part = self.read_table_hash_part(hash_part_len as usize);
        }
        LuaTable::new(array_part, hash_part)
    }

    fn read_table_array_part(&mut self, len: usize) -> ArrayPart {
        let mut array_part: ArrayPart = Some(Vec::new());
        for _ in 0..len {
            array_part.as_mut().unwrap().push(self.read_table_value());
        }
        array_part
    }

    fn read_table_hash_part(&mut self, len: usize) -> HashPart {
        let mut hash_part: HashPart = Some(Vec::new());
        for _ in 0..len {
            hash_part.as_mut().unwrap().push((self.read_table_value(), self.read_table_value()));
        }
        hash_part
    }

    fn read_complex_lua_number(&mut self) -> (u32, u32) {
        (self.read_uleb(), self.read_uleb()) //I think that it is in the form: XeY where X = first uleb, Y = second uleb. X may be a 32bit float and Y may be an integer.
    }

    fn read_lua_string(&mut self, len: usize) -> String {
        assert!(len > 0, "Cannot read string length of 0 or less.");
        let utf8_string = self.read_bytes(len);
        String::from_utf8(utf8_string).expect("String could not be read.")
    }

    //Read a luajit number constant.
    pub fn read_kn(&mut self) -> Box<dyn Any> {
        let mut kn_a = self.read_uleb();
        let is_a_double = (kn_a & 1) > 0;
        kn_a >>= 1;
        if is_a_double {
            let kn_b = self.read_uleb();
            let mut kn_union: u64 = kn_a as u64;
            kn_union <<= 16;
            kn_union |= kn_b as u64;
            return Box::new(kn_union as f64)
        } else {
            return Box::new(kn_a)
        }
    }
}

//TODO:
//  Test read_kn, read_kgc, read_lua_string, read_complex_lua_number, read_table_value, read_lua_table

fn setup_mock_binary_file() {
    File::create("mock.f").expect("Mock file could not be created.");
    let mut f = OpenOptions::new().read(true).write(true).open("mock.f").expect("File could not be opened.");
    let byte: [u8; 1] = [20];
    let bytes: [u8; 4] = [11, 32, 44, 99];
    let uleb_12345: [u8; 2] = [185, 96];
    f.write(&byte).expect("byte write failed.");
    f.write(&bytes).expect("bytes write failed.");
    f.write(&uleb_12345).expect("uleb write failed.");
}

#[test]
fn test_read_byte() {
    setup_mock_binary_file();
    let mut reader = Reader::new("mock.f");
    let byte = reader.read_byte();
    assert!(byte == 20, "actual: {}\n", byte);
}

#[test]
fn test_read_bytes() {
    setup_mock_binary_file();
    let mut reader = Reader::new("mock.f");
    let bytes = reader.read_bytes(5);
    assert!(bytes == [20, 11, 32, 44, 99], "actual: {:#?}\n", bytes);
}

#[test]
fn test_read_uleb() {
    setup_mock_binary_file();
    let mut reader = Reader::new("mock.f");
    reader.read_bytes(5); //advance offset to the uleb.
    let uleb = reader.read_uleb();
    assert!(uleb == 12345, "actual: {:#?}\n", uleb);
}
