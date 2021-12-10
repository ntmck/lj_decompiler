// Generates basic instruction blocks from prototypes.

use std::fmt;
use std::vec::Vec;
use std::collections::BTreeSet;

use crate::dis::bytecode_instruction::BytecodeInstruction;

pub struct Block {
    pub id: usize,
    pub start_index: usize,
    pub target_index: Option<usize>,
    pub instr: Vec<BytecodeInstruction>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bcis = String::new();
        for bci in self.instr.iter() {
            bcis.push_str(&format!("\t{}\n", bci));
        }
        write!(f, "B{} (start: {}, target: {:?}): \n{}", self.id, self.start_index, self.target_index, bcis)
    }
}

/// Takes one prototype's bytecode instructions and converts it to basic blocks.
pub struct Blocker{}
impl Blocker {
    pub fn make_blocks(&self, bcis: &Vec<BytecodeInstruction>) -> Vec<Block> {
        let blr = Blocker{};
        let mut targets = blr.find_jump_targets(&blr.find_jump_indices(&bcis), &bcis);
        let mut blocks: Vec<Block> = vec![];
        
        let mut t1 = targets.pop_first().unwrap();
        let mut id = 0;
        loop {
            if let Some(t2) = targets.pop_first() {
                blocks.push(Block {
                    id: id,
                    start_index: t1,
                    target_index: Some(t2),
                    instr: Vec::from(&bcis[t1..t2]),
                });
                t1 = t2;
            } else {
                blocks.push(Block {
                    id: id,
                    start_index: t1,
                    target_index: None,
                    instr: Vec::from(&bcis[t1..]),
                });
                break;
            }
            id += 1;
        }
        blocks
    }

    fn find_jump_indices(&self, bcis: &Vec<BytecodeInstruction>) -> Vec<isize> {
        let mut jump_indices: Vec<isize> = vec![];
        for (i, bci) in bcis.iter().enumerate() {
            if BytecodeInstruction::is_jump(bci) {
                jump_indices.push(i as isize);
            } else if BytecodeInstruction::is_conditional(bci) {
                jump_indices.push(-(i as isize)); //mark distance 1 jumps.
            }
        }
        jump_indices
    }

    fn find_jump_targets(&self, jump_indices: &Vec<isize>, bcis: &Vec<BytecodeInstruction>) -> BTreeSet<usize> {
        let mut targets: BTreeSet<usize> = BTreeSet::new();
        targets.insert(0);
        for i in jump_indices.iter() {
            if *i < 0 {
                targets.insert(2 + (-*i) as usize);
            } else {
                let jmp = &bcis[*i as usize];
                targets.insert(self.get_jump_target(*i as usize, &jmp));
            }
        }
        targets
    }

    fn get_jump_target(&self, jmp_index: usize, jmp: &BytecodeInstruction) -> usize {
        1 + jmp_index + ((jmp.b() as usize) << 8 | jmp.c() as usize) - 0x8000
    }
}

#[cfg(test)]
mod tests {
    use crate::dis::bytecode_instruction::BytecodeInstruction;
    use std::fs::File;
    use std::io::Write;
    use super::*;

    fn mock_bcis() -> Vec<BytecodeInstruction> {
        //From singleif.ljc
        let bcis: Vec<BytecodeInstruction> = vec![
            BytecodeInstruction::new(39, 0, 1, 0),
            BytecodeInstruction::new(39, 1, 2, 0),
            BytecodeInstruction::new(1, 1, 2, 0), //isge i2
            BytecodeInstruction::new(84, 0, 17, 128), //jmp i3

            BytecodeInstruction::new(52, 0, 0, 0),
            BytecodeInstruction::new(39, 1, 1, 0),
            BytecodeInstruction::new(62, 0, 2, 1),
            BytecodeInstruction::new(39, 0, 2, 0),
            BytecodeInstruction::new(39, 1, 3, 0),
            BytecodeInstruction::new(1, 1, 0, 0), //isge i9
            BytecodeInstruction::new(84, 0, 10, 128), //jmp i10

            BytecodeInstruction::new(52, 0, 0, 0),
            BytecodeInstruction::new(39, 1, 2, 0),
            BytecodeInstruction::new(62, 0, 2, 1),
            BytecodeInstruction::new(39, 0, 3, 0),
            BytecodeInstruction::new(39, 1, 4, 0),
            BytecodeInstruction::new(1, 1, 0, 0), //isge i16
            BytecodeInstruction::new(84, 0, 3, 128), //jmp i17

            BytecodeInstruction::new(52, 0, 0, 0),
            BytecodeInstruction::new(39, 1, 3, 0),
            BytecodeInstruction::new(62, 0, 2, 1),

            BytecodeInstruction::new(71, 0, 1, 0),
        ];
        bcis
    }

    fn debug_write_file(blocks: &Vec<Block>) {
        let mut file = File::create("debug.txt").unwrap();
        for block in blocks.iter() {
            writeln!(&mut file, "{}", block).unwrap();
        }
    }

    #[test]
    fn test_find_jump_indices() {
        let bcis = mock_bcis();
        let blr = Blocker{};
        let indices = blr.find_jump_indices(&bcis);
        assert!(indices.len() == 6);
        assert!(indices[0] == -2); //ISGE
        assert!(indices[1] == 3); //JMP
        assert!(indices[2] == -9); //ISGE
        assert!(indices[3] == 10); //JMP
        assert!(indices[4] == -16); //ISGE
        assert!(indices[5] == 17); //JMP
    }

    #[test]
    fn test_find_jump_targets() {
        let bcis = mock_bcis();
        let blr = Blocker{};
        let jumps = blr.find_jump_targets(&blr.find_jump_indices(&bcis), &bcis);
        let expected_targets: BTreeSet<usize> = [0, 4, 11, 18, 21].iter().cloned().collect();
        assert!(expected_targets.difference(&jumps).count() == 0);
    }

    #[test]
    fn test_make_blocks() {
        let bcis = mock_bcis();
        let blr = Blocker{};
        let blocks = blr.make_blocks(&bcis);
        assert!(blocks.len() == 5);
        assert!(blocks[0].instr.len() == 4);
        assert!(blocks[1].instr.len() == 7);
        assert!(blocks[2].instr.len() == 7);
        assert!(blocks[3].instr.len() == 3);
        assert!(blocks[4].instr.len() == 1);

        assert!(blocks[0].instr[..] == bcis[0..4]);
        assert!(blocks[1].instr[..] == bcis[4..11]);
        assert!(blocks[2].instr[..] == bcis[11..18]);
        assert!(blocks[3].instr[..] == bcis[18..21]);
        assert!(blocks[4].instr[..] == bcis[21..]);

        debug_write_file(&blocks);
    }
}