// Lua table constant storage.
use std::any::Any;
use std::vec::Vec;

//TODO: Refactor these types so that they become readable.
//type, Option<value>
pub type ArrayPart = Option<Vec<(u8, Option<Box<dyn Any>>)>>;
pub type HashPart = Option<Vec<((u8, Option<Box<dyn Any>>), (u8, Option<Box<dyn Any>>))>>;

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
