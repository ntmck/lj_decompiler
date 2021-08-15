// Lua table constant storage.
use std::any::Any;
use std::vec::Vec;

pub type ArrayPart = Option<Vec<Option<Box<dyn Any>>>>;
pub type HashPart = Option<Vec<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)>>;

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
