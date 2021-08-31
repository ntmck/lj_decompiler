// Lua table constant storage.
use std::vec::Vec;

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

pub struct ArrayPart {
    pub values: Vec<LuaValue>,
}

pub struct HashPart {
    pub keys: Vec<LuaValue>,
    pub values: Vec<LuaValue>,
}

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
