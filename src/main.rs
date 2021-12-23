#![feature(map_first_last)]
#![allow(dead_code)] // REMOVE BEFORE RELEASE AND CORRECT WARNINGS.
mod dis;
mod ir;

//use std::io::Write;
//use std::fs::File;
//use std::fs::OpenOptions;

//use crate::dis::prototyper::*;
//use crate::ir::blocker::*;

const FILE_NAMES: [&str; 1] = [
    "beam_system_client.lua",
];

fn main() {
    /*for name in FILE_NAMES.iter() {
        let ptr = Prototyper::new(name);
        let blr = Blocker{};
        let ptbs = blr.make_prototype_blocks(&ptr);
        let mut file = OpenOptions::new().truncate(true).open(&format!("{}.disasm", name)).unwrap();
        assert!(ptbs.len() == 34, "beam_system_client should have 34 prototypes. Found: {}", ptbs.len());
        for blocks in ptbs.iter() {
            for b in blocks.iter() {
                writeln!(&mut file, "{}", b);
            }
        }
    }*/
}