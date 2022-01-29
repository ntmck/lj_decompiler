use crate::{
    dis::bytecode_instruction::Bci,
    ir::{
        blocker::Block, ir_gen::Exp
    },
};

pub struct Translator{}
impl Translator {
    pub fn translate_block(block: &Block) -> Vec<Exp> {
        unimplemented!()
    }

    pub fn translate_bci(&self, bci: &Bci) -> Exp {
        match bci.op {
            0..=15  => self.comparison(bci),
            16..=19 => self.unary(bci),
            20..=36 => self.arith(bci),
            37..=42 => self.constant(bci),
            43..=48 => self.uv(bci),
            49      => self.assign(Exp::Var(bci.a() as u16), self.fnew(bci.d())),
            50..=60 => self.table(bci),
            61..=68 => self.call(bci),
            69..=72 => self.ret(bci),
            73..=77 => self.for_loop(bci),
            //78..=80 => iter loops
            //81..=83 => while/repeat loops
            84      => Exp::Target(bci.get_jump_target()),
            //85..=92 => funcs
            //93 => GOTOs

            _ => Exp::Error(format!("translate_bci: {}", bci).to_string()),
        }
    }

    fn for_loop(&self, bci: &Bci) -> Exp {
        //FORI denotes start of block for loop.
        //FORL is a backwards jump targeting the first instruction of the loop block.
        match bci.op {
            73 => self.fori(bci), //FORI
            74 => Exp::Error("JFORI unimplemented.".to_string()),
            75 => Exp::Redundant("FORL".to_string()), //FORLs are largely redundant information due to FORI.
            76 => Exp::Error("IFORL unimplemented.".to_string()),
            77 => Exp::Error("JFORL unimplemented.".to_string()),
            _ => Exp::Error("for_loop".to_string())
        }
    }

    fn fori(&self, bci: &Bci) -> Exp {
        let a = bci.a() as u16;
        //Fori slots range from a to a+2 inclusive.
        let step = a+2;
        let stop = a+1;
        let start = a;

        //End of scope is INCLUSIVE.
        let scope_end = bci.get_jump_target();
        
        //Vars for now. Simplification is needed later because they are typically KSHORT (Num Expressions). 
        let step = Box::new(Exp::Var(step));
        let stop = Box::new(Exp::Var(stop));
        let start = Box::new(Exp::Var(start));
        //a is for loop itself. a+1 is the first instruction of the scope. 
        let scope = Box::new(Exp::Range((bci.index + 1) as u32, scope_end - 1));
        
        Exp::For(start, stop, step, scope)
    }

    fn call(&self, bci: &Bci) -> Exp {
        //  [3] = print //get fname. usually GGET
        //  [4] = [1] //copy reference of variable(s) with MOV(s)
        //  [3](4..4) //arguments: (A+1...A+C-1) for CALL. slot 4 inclusive and 4 inclusive
        let a = bci.a() as u16;
        let b = bci.b() as u16;
        let c = bci.c() as u16;
        let d = bci.d();
        match bci.op {
            61 => self.callm(bci),
            //CALL: A(A+!...A+C-1) but A+C for exclusive range.
            62 => Exp::Call(Box::new(Exp::Var(a)), 
                Box::new(Exp::Range((a+1) as u32, (a+c-1) as u32)), 
                Box::new(Exp::Range((a+1) as u32, (a+b-1) as u32)), false),
            63 => Exp::Return(Box::new(self.callm(bci))),
            //CALLT: return A(A+1...A+D-1) but A+D for exclusive range.
            64 => Exp::Return(Box::new(Exp::Call(Box::new(Exp::Var(a)), 
                Box::new(Exp::Range((a+1) as u32, (a+d-1) as u32)), 
                Box::new(Exp::Range((a+1) as u32, (a+b-1) as u32)), false))),
            65 => Exp::Error("ITERC is unimplemented.".to_string()),
            66 => Exp::Error("ITERN is unimplemented.".to_string()),
            67 => Exp::VarArg(Box::new(Exp::Range(a as u32, (a+b-2) as u32))),
            68 => Exp::Error("ISNEXT is unimplemented.".to_string()),
            _  => Exp::Error("call".to_string()),
        }
    }

    fn callm(&self, bci: &Bci) -> Exp {
        //CALLM has an additional param of '...' unless another CALLM is a
        // parameter to the current CALLM. In which case, give the varg to 
        // the nested CALLMs.

        //fn name in slot A. Slot A is reused as a return slot for fn if returning.
        //fixed returns go in slots: A to A+B (+1 exclusive?)
        //fixed params are slots: A+1 to A+C (+1 exclusive?)
        let a = bci.a() as u16;
        let c = bci.c() as u16;
        let b = bci.b() as u16;

        let f_name = Box::new(Exp::Var(a));
        let param_range = Box::new(Exp::Range((a+1) as u32, (a+c+1) as u32));
        let return_range = Box::new(Exp::Range(a as u32, (a+b) as u32));

        Exp::Call(f_name, param_range, return_range, true)
    }

    fn ret(&self, bci: &Bci) -> Exp {
        let a = bci.a() as u16;
        let d = bci.d();
        match bci.op {
            69          => Exp::Error("RETM is unimplemented.".to_string()),
            70          => Exp::Return(Box::new(Exp::Range(a as u32, (a+d-2) as u32))), //RET
            71          => Exp::Return(Box::new(Exp::Empty)), //RET0
            72          => Exp::Return(Box::new(Exp::Var(a))), //RET1
            _           => Exp::Error("ret".to_string()),
        }
    }

    fn table(&self, bci: &Bci) -> Exp {
        if bci.op == 60 { return Exp::Error("TSETM is unimplemented.".to_string()) }

        let a = Exp::Var(bci.a() as u16);
        let tbl;
        
        let is_global = (52..=53).contains(&bci.op);
        if is_global {
            let d = Box::new(Exp::Str(bci.d()));
            tbl = Exp::Table(Box::new(Exp::Global), d);
        } else {
            let b = Box::new(Exp::Var(bci.b() as u16));
            let c = match bci.op {
                54 | 57 => Box::new(Exp::Var(bci.c() as u16)),
                55 | 58 => Box::new(Exp::Str(bci.c() as u16)),
                56 | 59 => Box::new(Exp::Lit(bci.c() as u16)),
                _       => Box::new(Exp::Error("table.c".to_string())),
            };
            tbl = Exp::Table(b, c);
        }

        let is_set = bci.op == 53 || (57..=60).contains(&bci.op);
        if is_set {
            self.assign(tbl, a)
        } else {
            self.assign(a, tbl)
        }
    }

    //makes a new func expression with proto index, but nothing is known about the function yet.
    fn fnew(&self, proto_index: u16) -> Exp {
        Exp::Func(proto_index, Box::new(Exp::Empty))
    }

    fn assign(&self, a: Exp, d: Exp) -> Exp {
        Exp::Move(Box::new(a), Box::new(d)) //A := D
    }

    fn uv(&self, bci: &Bci) -> Exp {
        match bci.op {
            43      => Exp::Move(Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Uv(bci.d()))),
            44..=47 => self.uset(bci),
            48      => Exp::UClo(bci.a() as u16, Box::new(Exp::Target(bci.get_jump_target()))),
            _       => Exp::Error("uv".to_string()),
        }
    }

    fn uset(&self, bci: &Bci) -> Exp {
        let a = Exp::Uv(bci.a() as u16);
        let d = match bci.op {
            44  => Exp::Var(bci.d()),
            45  => Exp::Str(bci.d()),
            46  => Exp::Num(bci.d()),
            47  => Exp::Pri(bci.d()),
            _   => Exp::Error("uset.d".to_string()),
        };
        let a = Box::new(a);
        let d = Box::new(d);

        Exp::Move(a, d)
    }

    fn unary(&self, bci: &Bci) -> Exp {
        let (a, d) = self.var_ad(bci);
        match bci.op {
            16 => Exp::Move(a, d),
            17 => Exp::Move(a, Box::new(Exp::Not(d))),
            18 => Exp::Move(a, Box::new(Exp::Unm(d))),
            19 => Exp::Move(a, Box::new(Exp::Len(d))),
            _ => Exp::Error("unary".to_string()),
        }
    }

    fn comparison(&self, bci: &Bci) -> Exp {
        if (12..=15).contains(&bci.op) { //unary test/copy
            let a = if bci.op == 12 || bci.op == 13 {
                Box::new(Exp::Move(Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Var(bci.d()))))
            } else {
                Box::new(Exp::Empty)
            };

            let mut d = Box::new(Exp::Var(bci.d()));
            let isf = bci.op == 13 || bci.op == 15;
            if isf {
                d = Box::new(Exp::Not(d));
            }
            Exp::IsT(a, d) //IsF = Not D. if it is a copy, A has a move instruction. if not copy, it is empty.

        } else {
            let a = Exp::Var(bci.a() as u16);
            let d = match bci.op {
                op if op < 6    => Exp::Var(bci.d()),
                op if op < 8    => Exp::Str(bci.d()),
                op if op < 10   => Exp::Num(bci.d()),
                op if op < 12   => Exp::Pri(bci.d()),
                _               => Exp::Error("comparison.d".to_string()),
            };
            let op = self.comparison_op(bci);
            let a = Box::new(a);
            let d = Box::new(d);
            let op = Box::new(op);

            Exp::Comparison(a, op, d)
        }
    }

    fn comparison_op(&self, bci: &Bci) -> Exp {
        match bci.op {
            0 if (bci.a() as u16) <= bci.d()            => Exp::NLt,
            0 if (bci.a() as u16) > bci.d()             => Exp::NGt,
            1 if (bci.a() as u16) <= bci.d()            => Exp::Lt,
            1 if (bci.a() as u16) > bci.d()             => Exp::Gt, 
            2 if (bci.a() as u16) <= bci.d()            => Exp::NLte,
            2 if (bci.a() as u16) > bci.d()             => Exp::NGte,
            3 if (bci.a() as u16) <= bci.d()            => Exp::Lte,
            3 if (bci.a() as u16) > bci.d()             => Exp::Gte,
            op if (4..=11).contains(&op) && op % 2 == 0 => Exp::Equals,
            op if (4..=11).contains(&op) && op % 2 == 1 => Exp::NEquals,
            _                                           => Exp::Error("comparison_op".to_string()),
        }
    }

    fn constant(&self, bci: &Bci) -> Exp {
        let value = match bci.op {
            37 => Exp::Str(bci.d()),
            38 => Exp::Error("KCDATA is unimplemented.".to_string()),
            39 => Exp::Lit(bci.d()),
            40 => Exp::Var(bci.d()),
            41 => Exp::Pri(bci.d()),
            42 => Exp::Error("KNIL is unimplemented.".to_string()),
            _  => Exp::Error("constant.value".to_string()),
        };
        let dst = Box::new(Exp::Var(bci.a() as u16));
        let value = Box::new(value);
        Exp::Move(dst, value)
    }

    fn var_ab(&self, bci: &Bci) -> (Box<Exp>, Box<Exp>) {
        let a = Box::new(Exp::Var(bci.a() as u16));
        let b = Box::new(Exp::Var(bci.b() as u16));
        (a, b)
    }

    fn var_ad(&self, bci: &Bci) -> (Box<Exp>, Box<Exp>) {
        (Box::new(Exp::Var(bci.a() as u16)), Box::new(Exp::Var(bci.d())))
    }

    fn binop(&self, bci: &Bci, b: Box<Exp>, c: Box<Exp>) -> Exp {
        match bci.op % 5 {
            0                   => Exp::Add(b, c),
            1 if bci.op == 31   => Exp::Pow(b, c),
            1                   => Exp::Sub(b, c),
            2 if bci.op == 32   => Exp::Cat(b, c),
            2                   => Exp::Mul(b, c),
            3                   => Exp::Div(b, c),
            4                   => Exp::Mod(b, c),
            _                   => Exp::Error("binop".to_string()),
        }
    }

    fn arith(&self, bci: &Bci) -> Exp {
        let (a, b) = self.var_ab(bci);
        let c;
        if (30..=34).contains(&bci.op) { //vv op
            c = Box::new(Exp::Var(bci.c() as u16));
        } else { //vn or nv
            c = Box::new(Exp::Num(bci.c() as u16));
        }

        if (25..=29).contains(&bci.op) { //nv
            Exp::Move(a, Box::new(self.binop(bci, c, b)))
        } else { //vx
            Exp::Move(a, Box::new(self.binop(bci, b, c)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dis::{
            prototyper::*,
            bytecode_instruction::Bci,
        },
        ir::{
            blocker::*,
            ir_gen::*,
        }
    };

    use std::fs::File;
    use std::io::Write;
    use super::*;

    fn debug_write_file(contents: &str) {
        let mut file = File::create("debug_ir.txt").unwrap();
        write!(&mut file, "{}", contents).unwrap();
    }

    #[test]
    fn test_write() {
        let mut ptr = Prototyper::new("dec.lua");
        let blr = Blocker{};
        let t = Translator{};
        let pt = ptr.next().unwrap(); //dec.ifs
        let pt = ptr.next().unwrap(); //dec.loops
        //let pt = ptr.next().unwrap(); //dec.gotos
        //let pt = ptr.next().unwrap(); //dec.equivgoto
        //let pt = ptr.next().unwrap(); //dec.vargs
        let blocks = blr.make_blocks(&pt);

        let mut contents = "".to_string();
        for (i, block) in blocks.iter().enumerate() {
            contents.push_str(&format!("Block: {}\n", i));
            for bci in block.instructions.iter() {
                contents.push_str(&format!("\t{}: {}\n", bci.index, t.translate_bci(bci)));
            }
            contents.push_str("\n");
        }
        debug_write_file(&contents);
    }
}

/* Comparisons:
Source Code     then    Bytecode
if x < y        then    ISGE x y
if x <= y       then    ISGT x y
if x > y        then    ISGE y x
if x >= y       then    ISGT y x

if not (x < y)  then    ISLT x y
if not (x <= y) then    ISLE x y
if not (x > y)  then    ISLT y x
if not (x >= y) then    ISLE y x

if for slots A and D (A <= D = x y), (A > D = y x)
*/