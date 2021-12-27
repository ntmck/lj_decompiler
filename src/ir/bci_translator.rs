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
            //61..=68 => call/var args?
            //69..=72 => self.ret(bci),
            //73..=77 => for loops
            //78..=80 => iter loops
            //81..=83 => while/repeat loops
            84      => Exp::Target(bci.get_jump_target()),
            //85..=92 => funcs
            //93 => GOTOs

            _ => Exp::Error,
        }
    }

    fn ret(&self, bci: &Bci) -> Exp {
        unimplemented!()
    }

    fn table(&self, bci: &Bci) -> Exp {
        if bci.op == 60 { unimplemented!("TSETM"); }

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
                _       => Box::new(Exp::Error),
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
            _       => Exp::Error,
        }
    }

    fn uset(&self, bci: &Bci) -> Exp {
        let a = Exp::Uv(bci.a() as u16);
        let d = match bci.op {
            44  => Exp::Var(bci.d()),
            45  => Exp::Str(bci.d()),
            46  => Exp::Num(bci.d()),
            47  => Exp::Pri(bci.d()),
            _   => Exp::Error,
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
            _ => Exp::Error,
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
                _               => Exp::Error,
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
            //todo: ISTC/ISFC/IST/ISF
            _                                           => Exp::Error,
        }
    }

    fn constant(&self, bci: &Bci) -> Exp {
        println!("constant: {}, op: {}", bci, bci.op);
        let value = match bci.op {
            37 => Exp::Str(bci.d()),
            38 => unimplemented!("KCDATA"),
            39 => Exp::Lit(bci.d()),
            40 => Exp::Var(bci.d()),
            41 => Exp::Pri(bci.d()),
            42 => unimplemented!("KNIL"),
            _ => Exp::Error,
        };
        let dst = Box::new(Exp::Var(bci.a() as u16));
        let value = Box::new(value);
        println!("dst: {}, value: {}", &dst, &value);
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
            _                   => Exp::Error,
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
        let pt = ptr.next(); //dec.ifs
        let pt = ptr.next(); //dec.loops
        let pt = ptr.next(); //dec.gotos
        let pt = ptr.next(); //dec.equivgoto
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
        panic!()
    }

    #[test]
    fn test_translate_constants() {
        let t = Translator{};
        unimplemented!()
    }

    #[test]
    fn test_translate_arithmetic() {
        let t = Translator{};

        //VN
        let exp = t.translate_bci(&Bci::new(0, 20, 0, 1, 2)); //addvn
        //debug_write_file(&format!("{}", exp));
        println!("{}", exp);
        //panic!();
        //NV
        //VV
        //POW
        //CAT
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