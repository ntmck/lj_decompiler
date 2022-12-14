mod merge_literals;

use crate::ir::translator::{IRPrototype, IRBlock};

pub trait Rule {
    fn apply(block: &mut IRBlock);
}