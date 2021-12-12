// Generates modified 3-address code.
use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;

use crate::dis::bytecode_instruction::BytecodeInstruction;
use crate::dis::prototyper::Prototype;
use crate::ir::blocker::{Block, Blocker};

struct InfixOp {
    pub opr1: String,
    pub op: String,
    pub opr2: String,
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = String::from(&format!("{} {} {}", self.opr1, self.op, self.opr2));
        write!(f, "{}", s.trim_end())
    }
}

struct Statement {
    dst: String,
    infix: InfixOp,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} := {}", self.dst, self.infix)
    }
}

struct Branch {
    condition: InfixOp,
    left: Option<usize>, 
    right: Option<usize>,
}
impl fmt::Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut left = "#".to_string();
        let mut right = "#".to_string();
        if let Some(l) = self.left {
            left = format!("{}", l).to_string();
        }
        if let Some(r) = self.right {
            right = format!("{}", r).to_string();
        }
        write!(f, "if ({}) B{} else B{}", self.condition, left, right)
    }
}

struct IrBlock {
    statements: Vec<Statement>,
    branch: Branch,
}
impl fmt::Display for IrBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut out = String::new();
        for stmt in self.statements.iter() {
            out.push_str(&format!("{}\n", stmt));
        }
        out.push_str(&format!("{}\n", self.branch));
        out.trim_end();
        write!(f, "{}", out)
    }
}

struct IrGen{}
impl IrGen { 
    fn make_ir_block(block_index: usize, blocks: &Vec<Block>, pt: &Prototype) -> IrBlock {
        let (left, right) = IrGen::find_branch_block_targets(block_index, blocks);
        IrBlock {
            statements: IrGen::make_statements(&blocks[block_index], pt),
            branch: IrGen::make_branch(&blocks[block_index].instr[2], pt, left, right),
        }
    }

    fn find_branch_block_targets(block_index: usize, blocks: &Vec<Block>) -> (Option<usize>, Option<usize>) {
        //assumes that there are jumps must be paired with conditionals.
        let conditional_index = blocks.len() - 3;
        let jmp_index = blocks.len() - 2;
        let left = IrGen::find_branch_block_target(block_index, conditional_index, blocks);
        let right = IrGen::find_branch_block_target(block_index, jmp_index, blocks);
        println!("left {:?}, right {:?}", left, right);
        (left, right)
    }

    fn find_branch_block_target(block_index: usize, instr_index: usize, blocks: &Vec<Block>) -> Option<usize> {
        let jmp = &blocks[block_index].instr[instr_index];
        if jmp.is_conditional() {
            return Some(block_index + 1);
        }
        if !jmp.is_jump() {
            return None;
        }
        let target = Blocker::get_jump_target(jmp);
        for i in 0..blocks.len() {
            if blocks[i].start_index == target {
                return Some(i);
            }
        }   
        return None;
    }

    fn make_statements(block: &Block, pt: &Prototype) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        for bci in block.instr.iter() {
            if let Some(stmt) = IrGen::make_statement(&bci, pt) {
                statements.push(stmt);
            }
        }
        statements
    }

    //TODO: Maybe move this code to a statement builder file.
    fn make_statement(bci: &BytecodeInstruction, pt: &Prototype) -> Option<Statement> {
        if bci.is_constant() {
            let dst = String::from(&pt.symbols.as_ref().unwrap()[bci.a() as usize]);
            let infix = InfixOp {
                opr1: String::from(&format!("{}", bci.d())),
                op: "".to_string(),
                opr2: "".to_string(),
            };
            return Some(Statement {
                dst: dst,
                infix: infix,
            });
        }
        None
    }

    fn make_branch(comparison: &BytecodeInstruction, pt: &Prototype, left: Option<usize>, right: Option<usize>) -> Branch {
        assert!(comparison.is_conditional(), "Bci is not a comparison instruction!");

        let opr1 = String::from(&pt.symbols.as_ref().unwrap()[(comparison.a() - 1) as usize]);

        Branch {
            condition: InfixOp {
                opr1: opr1,
                op: IrGen::get_branch_op(comparison),
                opr2: IrGen::get_branch_opr2(comparison, pt),
            },
            left: left,
            right: right,
        }
    }

    fn get_branch_opr2(comparison: &BytecodeInstruction, pt: &Prototype) -> String {
        let mut opr2 = "".to_string();

        if comparison.op < 6 { //symbol comparison
            opr2 = String::from(&pt.symbols.as_ref().unwrap()[(comparison.d() - 1) as usize]);
        } else if comparison.op < 9 { //string constant comparison
            opr2 = String::from(&pt.constants.as_ref().unwrap().strings[(comparison.d() - 1) as usize]);
        } else if comparison.op < 12 { //num constant comparison
            opr2 = String::from(&format!("{}", pt.constants.as_ref().unwrap().non_strings[(comparison.d() - 1) as usize]));
        } else { //primitive comparison: 0:nil, 1:false, 2:true
            match comparison.d() {
                0 => opr2 = "nil".to_string(),
                1 => opr2 = "false".to_string(),
                2 => opr2 = "true".to_string(),
                _ => opr2 = "ERR".to_string()
            }
        }

        opr2.to_string()
    }

/*

    if x < y        then	ISGE x y
    if x <= y       then	ISGT x y
    if x > y        then	ISGE y x
    if x >= y       then    ISGT y x

    if not (x < y)  then	ISLT x y -> "~<"
    if not (x <= y) then	ISLE x y -> "~<="
    if not (x > y)  then	ISLT y x -> "~>"
    if not (x >= y) then	ISLE y x -> "~>="

*/
    //Maybe if slot A = x, slot B = y -> (A <= B = yx), (A > B = xy)? Requires new bytecode to test.
    fn get_branch_op(bci: &BytecodeInstruction) -> String {
        let mut op_s = "";
        match bci.op {
            op if op == 0 && (bci.a() as u16) > bci.d()     => op_s = "~<",
            op if op == 0 && (bci.a() as u16) <= bci.d()    => op_s = "~>",

            op if op == 1 && (bci.a() as u16) >= bci.d()    => op_s = "<",
            op if op == 1 && (bci.a() as u16) < bci.d()     => op_s = ">",

            op if op == 2 && (bci.a() as u16) > bci.d()     => op_s = "~<=",
            op if op == 2 && (bci.a() as u16) <= bci.d()    => op_s = "~>=",

            op if op == 3 && (bci.a() as u16) > bci.d()     => op_s = "<=",
            op if op == 3 && (bci.a() as u16) <= bci.d()    => op_s = ">=",

            op if op >= 4 && op % 2 == 0                    => op_s = "==",
            op if op >= 4 && op % 2 == 1                    => op_s = "~=",

            _ => op_s = "ERR",
        }
        return op_s.to_string();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    use std::io::Write;
    use std::fs::File;

    use crate::dis::bytecode_instruction::*;
    use crate::ir::blocker::Block;
    use crate::ir::ir_gen::*;
    use crate::dis::prototyper::*;

    fn debug_write_file(irb: &IrBlock) {
        let mut file = File::create("debug.txt").unwrap();
        write!(file, "{}", irb);
    }

    #[test]
    fn test_block0_ir() {
        let test_blocks = mock_blocks();
        let test_pt = mock_prototype();
        let ir_block = IrGen::make_ir_block(0, &test_blocks, &test_pt);

        let expected = "var_pt0_0 := 1";
        let actual = format!("{}", ir_block.statements[0]);
        assert!(actual == expected, "expected:\n\t{} actual:\n\t{}", expected, actual);

        let expected = "var_pt0_1 := 2";
        let actual = format!("{}", ir_block.statements[1]);
        assert!(actual == expected, "expected:\n\t{} actual:\n\t{}", expected, actual);

        let expected = "if (var_pt0_0 > var_pt0_1) B1 else B4";
        let actual = format!("{}", ir_block.branch);
        assert!(actual == expected, "expected:\n\t{} actual:\n\t{}", expected, actual);

        debug_write_file(&ir_block);
    }

    fn mock_blocks() -> Vec<Block> {
        //singleif.ljc's blocks.
        let blocks: Vec<Block> = vec![
            Block {
                id: 0,
                start_index: 0,
                target_index: Some(4),
                instr: vec![
                    BytecodeInstruction::new(0, 39, 0, 1, 0),
                    BytecodeInstruction::new(1, 39, 1, 2, 0),
                    BytecodeInstruction::new(2, 1, 1, 2, 0),
                    BytecodeInstruction::new(3, 84, 0, 17, 128),
                ]
            },
            Block {
                id: 1,
                start_index: 4,
                target_index: Some(11),
                instr: vec![
                    BytecodeInstruction::new(4, 52, 0, 0, 0),
                    BytecodeInstruction::new(5, 39, 1, 1, 0),
                    BytecodeInstruction::new(6, 62, 0, 2, 1),
                    BytecodeInstruction::new(7, 39, 0, 2, 0),
                    BytecodeInstruction::new(8, 39, 1, 3, 0),
                    BytecodeInstruction::new(9, 1, 1, 0, 0),
                    BytecodeInstruction::new(10, 84, 0, 10, 128),
                ]
            },
            Block {
                id: 2,
                start_index: 11,
                target_index: Some(18),
                instr: vec![
                    BytecodeInstruction::new(11, 52, 0, 0, 0),
                    BytecodeInstruction::new(12, 39, 1, 2, 0),
                    BytecodeInstruction::new(13, 62, 0, 2, 1),
                    BytecodeInstruction::new(14, 39, 0, 3, 0),
                    BytecodeInstruction::new(15, 39, 1, 4, 0),
                    BytecodeInstruction::new(16, 1, 1, 0, 0),
                    BytecodeInstruction::new(17, 84, 0, 3, 128),
                ]
            },
            Block {
                id: 3,
                start_index: 18,
                target_index: Some(21),
                instr: vec![
                    BytecodeInstruction::new(18, 52, 0, 0, 0),
                    BytecodeInstruction::new(19, 39, 1, 3, 0),
                    BytecodeInstruction::new(20, 62, 0, 2, 1),
                ]
            },
            Block {
                id: 4,
                start_index: 21,
                target_index: None,
                instr: vec![
                    BytecodeInstruction::new(21, 71, 0, 1, 0),
                ]
            },
        ];
        blocks
    }

    fn mock_prototype() -> Prototype {
        Prototype {
            id: 0,
            header: Some(
                PrototypeHeader {
                    flags: 2,
                    num_params: 0,
                    frame_size: 2,
                    size_uv: 0,
                    size_kgc: 1,
                    size_kn: 0,
                    instruction_count: 22,
                    prototype_size: 101,
                    dbg_info_header: None,
                },
            ),
            raw_uvs: None,
            bound_uvs: None,
            constants: Some(
                Constants {
                    strings: VecDeque::from_iter([
                        String::from("print"),
                    ]),
                    non_strings: vec![],
                },
            ),
            symbols: Some(
                vec![
                    String::from("var_pt0_0"),
                    String::from("var_pt0_1"),
                ],
            ),
            instructions: Some(
                vec![
                    BytecodeInstruction {
                        index: 0,
                        op: 39,
                        registers: Registers {
                            a: 0,
                            c: 1,
                            b: 0,
                            d: 1,
                        },
                    },
                    BytecodeInstruction {
                        index: 4,
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 2,
                            b: 0,
                            d: 2,
                        },
                    },
                    BytecodeInstruction {
                        index: 8,
                        op: 1,
                        registers: Registers {
                            a: 1,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        index: 12,
                        op: 84,
                        registers: Registers {
                            a: 0,
                            c: 17,
                            b: 128,
                            d: 32785,
                        },
                    },
                    BytecodeInstruction {
                        index: 16,
                        op: 52,
                        registers: Registers {
                            a: 0,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        index: 20,
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 1,
                            b: 0,
                            d: 1,
                        },
                    },
                    BytecodeInstruction {
                        index: 24,
                        op: 62,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 1,
                            d: 258,
                        },
                    },
                    BytecodeInstruction {
                        index: 28,
                        op: 39,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 0,
                            d: 2,
                        },
                    },
                    BytecodeInstruction {
                        index: 32,
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 3,
                            b: 0,
                            d: 3,
                        },
                    },
                    BytecodeInstruction {
                        index: 36,
                        op: 1,
                        registers: Registers {
                            a: 1,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        index: 40,
                        op: 84,
                        registers: Registers {
                            a: 0,
                            c: 10,
                            b: 128,
                            d: 32778,
                        },
                    },
                    BytecodeInstruction {
                        index: 44,
                        op: 52,
                        registers: Registers {
                            a: 0,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        index: 48,
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 2,
                            b: 0,
                            d: 2,
                        },
                    },
                    BytecodeInstruction {
                        index: 52,
                        op: 62,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 1,
                            d: 258,
                        },
                    },
                    BytecodeInstruction {
                        index: 56,
                        op: 39,
                        registers: Registers {
                            a: 0,
                            c: 3,
                            b: 0,
                            d: 3,
                        },
                    },
                    BytecodeInstruction {
                        index: 60,
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 4,
                            b: 0,
                            d: 4,
                        },
                    },
                    BytecodeInstruction {
                        index: 64,
                        op: 1,
                        registers: Registers {
                            a: 1,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        index: 68,
                        op: 84,
                        registers: Registers {
                            a: 0,
                            c: 3,
                            b: 128,
                            d: 32771,
                        },
                    },
                    BytecodeInstruction {
                        index: 72,
                        op: 52,
                        registers: Registers {
                            a: 0,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        index: 76,
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 3,
                            b: 0,
                            d: 3,
                        },
                    },
                    BytecodeInstruction {
                        index: 80,
                        op: 62,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 1,
                            d: 258,
                        },
                    },
                    BytecodeInstruction {
                        index: 84,
                        op: 71,
                        registers: Registers {
                            a: 0,
                            c: 1,
                            b: 0,
                            d: 1,
                        },
                    },
                ],
            ),
            proto_parent: None,
            proto_children: None,
        }        
    }
}