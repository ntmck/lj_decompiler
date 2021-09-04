// Lua table constant storage.
use std::vec::Vec;

#[derive(Debug)]
pub enum LuaValue {
    Nil,
    ChildProto,
    Table(LuaTable),
    True,
    False,
    SInt(i32),
    UInt(u32),
    ComplexNum((u32, u32)),
    Str(String),
    Double(f64),
}

#[derive(Debug)]
pub struct ArrayPart {
    pub values: Vec<LuaValue>,
}

#[derive(Debug)]
pub struct HashPart {
    pub keys: Vec<LuaValue>,
    pub values: Vec<LuaValue>,
}

#[derive(Debug)]
pub struct LuaTable {
    array_part: ArrayPart,
    hash_part: HashPart,
}

impl LuaTable {
    pub fn new(array_part: ArrayPart, hash_part: HashPart) -> LuaTable {
        LuaTable {
            array_part: array_part,
            hash_part: hash_part,
        }
    }
}
