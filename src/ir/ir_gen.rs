// Generates modified 3-address code.
use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;

use crate::dis::bytecode_instruction::BytecodeInstruction;
use crate::dis::prototyper::Prototype;
use crate::ir::blocker::Block;

struct InfixOp {
    pub opr1: String,
    pub op: String,
    pub opr2: String,
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.opr1, self.op, self.opr2)
    }
}

struct Statement {
    dst: String,
    infix: InfixOp,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} := {}", self.dst, self.infix)
    }
}

struct Branch {
    condition: InfixOp,
    left: isize, 
    right: isize,
}
impl fmt::Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "if ({}) B{} else B{}", self.condition, self.left, self.right)
    }
}

struct IrBlock {
    statements: Vec<Statement>,
    branch: Branch,
}

struct IrGen{}
impl IrGen { 
    pub fn make_ir_block(block: &Block, pt: &Prototype) -> IrBlock {
        IrBlock {
            statements: IrGen::make_statements(block, pt),
            branch: IrGen::make_branch(&block.instr[2], pt, -1, -1),
        }
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

    fn make_branch(comparison: &BytecodeInstruction, pt: &Prototype, left: isize, right: isize) -> Branch {
        assert!(comparison.is_conditional(), "Bci is not a comparison instruction!");

        let opr1 = String::from(&pt.symbols.as_ref().unwrap()[(comparison.a() - 1) as usize]);

        Branch {
            condition: InfixOp {
                opr1: opr1,
                op: IrGen::get_branch_op(comparison.op),
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

    fn get_branch_op(op: u8) -> String {
        let mut op_s = "";
        match op {
            0                           => op_s = "<",
            1                           => op_s = ">=",
            2                           => op_s = "<=",
            3                           => op_s = ">",
            4 | 6 | 8 | 10 | 11         => op_s = "==",
            5 | 7 | 9                   => op_s = "~=",
            _   => op_s = "ERR",
        }
        return op_s.to_string();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::iter::FromIterator;

    use crate::dis::bytecode_instruction::*;
    use crate::ir::blocker::Block;
    use crate::ir::ir_gen::IrGen;
    use crate::dis::prototyper::*;

    #[test]
    fn test_block0_ir() {
        let test_blocks = mock_blocks();
        let test_pt = mock_prototype();
        let ir_block = IrGen::make_ir_block(&test_blocks[0], &test_pt);

        let expected = "var_pt0_0 := 1  \n";
        let actual = format!("{}", ir_block.statements[0]);
        assert!(actual == expected, "expected:\n\t{} actual:\n\t{}", expected, actual);

        let expected = "var_pt0_1 := 2  \n";
        let actual = format!("{}", ir_block.statements[1]);
        assert!(actual == expected, "expected:\n\t{} actual:\n\t{}", expected, actual);

        let expected = "if (var_pt0_0 >= var_pt0_1) B1 else B4\n";
        let actual = format!("{}", ir_block.branch);
        assert!(actual == expected, "expected:\n\t{} actual:\n\t{}", expected, actual);
    }

    fn mock_blocks() -> Vec<Block> {
        //singleif.ljc's blocks.
        let blocks: Vec<Block> = vec![
            Block {
                id: 0,
                start_index: 0,
                target_index: Some(4),
                instr: vec![
                    BytecodeInstruction::new(39, 0, 1, 0),
                    BytecodeInstruction::new(39, 1, 2, 0),
                    BytecodeInstruction::new(1, 1, 2, 0),
                    BytecodeInstruction::new(84, 0, 17, 128),
                ]
            },
            Block {
                id: 1,
                start_index: 4,
                target_index: Some(11),
                instr: vec![
                    BytecodeInstruction::new(52, 0, 0, 0),
                    BytecodeInstruction::new(39, 1, 1, 0),
                    BytecodeInstruction::new(62, 0, 2, 1),
                    BytecodeInstruction::new(39, 0, 2, 0),
                    BytecodeInstruction::new(39, 1, 3, 0),
                    BytecodeInstruction::new(1, 1, 0, 0),
                    BytecodeInstruction::new(84, 0, 10, 128),
                ]
            },
            Block {
                id: 2,
                start_index: 11,
                target_index: Some(18),
                instr: vec![
                    BytecodeInstruction::new(52, 0, 0, 0),
                    BytecodeInstruction::new(39, 1, 2, 0),
                    BytecodeInstruction::new(62, 0, 2, 1),
                    BytecodeInstruction::new(39, 0, 3, 0),
                    BytecodeInstruction::new(39, 1, 4, 0),
                    BytecodeInstruction::new(1, 1, 0, 0),
                    BytecodeInstruction::new(84, 0, 3, 128),
                ]
            },
            Block {
                id: 3,
                start_index: 18,
                target_index: Some(21),
                instr: vec![
                    BytecodeInstruction::new(52, 0, 0, 0),
                    BytecodeInstruction::new(39, 1, 3, 0),
                    BytecodeInstruction::new(62, 0, 2, 1),
                ]
            },
            Block {
                id: 4,
                start_index: 21,
                target_index: None,
                instr: vec![
                    BytecodeInstruction::new(71, 0, 1, 0),
                ]
            },
        ];
        blocks
    }

    fn mock_prototype() -> Prototype {
        //singleif.ljc's only prototype.
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
                    strings: VecDeque::from_iter(
                        [
                            String::from("print"),
                        ]
                    ),
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
                        op: 39,
                        registers: Registers {
                            a: 0,
                            c: 1,
                            b: 0,
                            d: 1,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 2,
                            b: 0,
                            d: 2,
                        },
                    },
                    BytecodeInstruction {
                        op: 1,
                        registers: Registers {
                            a: 1,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        op: 84,
                        registers: Registers {
                            a: 0,
                            c: 17,
                            b: 128,
                            d: 32785,
                        },
                    },
                    BytecodeInstruction {
                        op: 52,
                        registers: Registers {
                            a: 0,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 1,
                            b: 0,
                            d: 1,
                        },
                    },
                    BytecodeInstruction {
                        op: 62,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 1,
                            d: 258,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 0,
                            d: 2,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 3,
                            b: 0,
                            d: 3,
                        },
                    },
                    BytecodeInstruction {
                        op: 1,
                        registers: Registers {
                            a: 1,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        op: 84,
                        registers: Registers {
                            a: 0,
                            c: 10,
                            b: 128,
                            d: 32778,
                        },
                    },
                    BytecodeInstruction {
                        op: 52,
                        registers: Registers {
                            a: 0,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 2,
                            b: 0,
                            d: 2,
                        },
                    },
                    BytecodeInstruction {
                        op: 62,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 1,
                            d: 258,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 0,
                            c: 3,
                            b: 0,
                            d: 3,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 4,
                            b: 0,
                            d: 4,
                        },
                    },
                    BytecodeInstruction {
                        op: 1,
                        registers: Registers {
                            a: 1,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        op: 84,
                        registers: Registers {
                            a: 0,
                            c: 3,
                            b: 128,
                            d: 32771,
                        },
                    },
                    BytecodeInstruction {
                        op: 52,
                        registers: Registers {
                            a: 0,
                            c: 0,
                            b: 0,
                            d: 0,
                        },
                    },
                    BytecodeInstruction {
                        op: 39,
                        registers: Registers {
                            a: 1,
                            c: 3,
                            b: 0,
                            d: 3,
                        },
                    },
                    BytecodeInstruction {
                        op: 62,
                        registers: Registers {
                            a: 0,
                            c: 2,
                            b: 1,
                            d: 258,
                        },
                    },
                    BytecodeInstruction {
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