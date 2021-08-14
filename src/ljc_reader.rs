// Helps read portions the luajit compiled file.

use std::fs::File;
use std::io::{Read, SeekFrom, Seek};
use std::vec::Vec;

// Test imports
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
}

fn setup_mock_binary_file() {
    File::create("mock.f").expect("Mock file could not be created.");
    let mut f = OpenOptions::new().read(true).write(true).open("mock.f").expect("File could not be opened.");
    let byte: [u8; 1] = [20];
    let bytes: [u8; 4] = [11, 32, 44, 99];
    let uleb_12345: [u8; 2] = [185, 96];
    f.write(&byte);
    f.write(&bytes);
    f.write(&uleb_12345);
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
