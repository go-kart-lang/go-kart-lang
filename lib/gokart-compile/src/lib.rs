use std::collections::VecDeque;

use gokart_ast::{Exp, Pat, Var};
use gokart_vm::{GOpCode, OpCode, PrimOp};
use rpds::List;

enum EnvUnit<'a> {
    Constructed(&'a Pat),
    Annotated(&'a Pat, i32),
}

type VOpCode = GOpCode<i32>;

type Env<'a> = List<EnvUnit<'a>>;

fn E(var: &Var, env: &Env, lvl: u32) -> Option<VecDeque<VOpCode>> {
    let mut lvl = lvl;
    let mut iter = env.iter();

    loop {
        match iter.next() {
            None => {
                break None;
            }
            Some(EnvUnit::Constructed(pat)) => {
                if let Some(mut code) = P(var, pat) {
                    code.push_front(VOpCode::Acc(lvl));
                    break Some(code);
                } else {
                    lvl += 1;
                }
            }
            Some(EnvUnit::Annotated(pat, lbl)) => {
                if let Some(mut code) = P(var, pat) {
                    code.push_front(VOpCode::Call(*lbl));
                    code.push_front(VOpCode::Rest(lvl));
                    break Some(code);
                }
            }
        }
    }
}

fn P(var: &Var, p: &Pat) -> Option<VecDeque<VOpCode>> {
    match p {
        Pat::EmptyPattern => None,
        Pat::Variable(y) => {
            if var == y {
                // Some(VecDeque::from([VOpCode::Skip]))
                Some(VecDeque::new())
            } else {
                None
            }
        }
        Pat::Pair(p1, p2) => {
            if let Some(mut code) = P(var, p1) {
                code.push_front(VOpCode::Rest(1));
                Some(code)
            } else if let Some(mut code) = P(var, p2) {
                code.push_front(VOpCode::Acc(0));
                Some(code)
            } else {
                None
            }
        }
        Pat::Layer(y, pat) => {
            if var == y {
                // Some(VecDeque::from([VOpCode::Skip]))
                Some(VecDeque::new())
            } else {
                P(var, pat)
            }
        }
    }
}

fn prim_func_name_to_prim_op(x: &String) -> Option<PrimOp> {
    if x == "IntPlus" {
        Some(PrimOp::IntPlus)
    } else if x == "IntMinus" {
        Some(PrimOp::IntMinus)
    } else if x == "IntDiv" {
        Some(PrimOp::IntDiv)
    } else if x == "IntMul" {
        Some(PrimOp::IntMul)
    } else if x == "IntLe" {
        Some(PrimOp::IntLe)
    } else if x == "IntEq" {
        Some(PrimOp::IntEq)
    } else {
        None
    }
}

fn transform_postponed_label(postponed_labels: &Vec<u32>, l: i32) -> u32 {
    if l < 0 {
        let idx = (-l - 1) as usize;
        postponed_labels[idx]
    } else {
        l as u32
    }
}

pub struct Compiler<'a> {
    code: VecDeque<VOpCode>,
    queue: VecDeque<(&'a Exp, Env<'a>)>,
}

impl<'a> Compiler<'a> {
    pub fn compile(exp: &'a Exp) -> VecDeque<OpCode> {
        let mut compiler = Compiler {
            code: VecDeque::new(),
            queue: VecDeque::new(),
        };

        compiler.C(exp, List::new());
        compiler.code.push_back(VOpCode::Stop);

        let mut postponed_labels = vec![];

        let mut idx = 0;
        loop {
            if idx >= compiler.queue.len() {
                break;
            }
            postponed_labels.push(compiler.code.len() as u32);
            let (e, env) = &compiler.queue[idx];
            compiler.R(e, env.clone());
            idx += 1;
        }

        let mut result = VecDeque::new();

        for op in std::mem::replace(&mut compiler.code, VecDeque::new()) {
            let newop = match op {
                GOpCode::Acc(x) => OpCode::Acc(x),
                GOpCode::Rest(x) => OpCode::Rest(x),
                GOpCode::QuoteInt(x) => OpCode::QuoteInt(x),
                GOpCode::Push => OpCode::Push,
                GOpCode::Swap => OpCode::Swap,
                GOpCode::Prim(prim_op) => OpCode::Prim(prim_op),
                GOpCode::Cur(l) => OpCode::Cur(transform_postponed_label(&postponed_labels, l)),
                GOpCode::Return => OpCode::Return,
                GOpCode::Clear => OpCode::Clear,
                GOpCode::Cons => OpCode::Cons,
                GOpCode::App => OpCode::App,
                GOpCode::Pack(c) => OpCode::Pack(c),
                GOpCode::Skip => OpCode::Skip,
                GOpCode::Stop => OpCode::Stop,
                GOpCode::Call(l) => OpCode::Call(transform_postponed_label(&postponed_labels, l)),
                GOpCode::Gotofalse(l) => {
                    OpCode::Gotofalse(transform_postponed_label(&postponed_labels, l))
                }
                GOpCode::Switch(c, l) => {
                    OpCode::Switch(c, transform_postponed_label(&postponed_labels, l))
                }
                GOpCode::Goto(l) => OpCode::Goto(transform_postponed_label(&postponed_labels, l)),
            };
            result.push_back(newop);
        }
        result
    }

    fn C(&mut self, exp: &'a Exp, env: Env<'a>) {
        println!("{:?}", exp);
        match exp {
            Exp::Variable(var) => {
                self.code.extend(E(var, &env, 0).unwrap());
            }
            Exp::ConstInt(x) => {
                self.code.push_back(VOpCode::QuoteInt(*x));
            }
            Exp::Sys(func_name, exp1, exp2) => {
                if let Some(prim_op) = prim_func_name_to_prim_op(func_name) {
                    self.T(exp1, exp2, env);
                    self.code.push_back(VOpCode::Prim(prim_op));
                } else {
                    panic!("prim op not found")
                }
            }
            Exp::EmptyTuple => {
                self.code.push_back(VOpCode::Clear);
            }
            Exp::Pair(e1, e2) => {
                self.T(e1, e2, env);
                self.code.push_back(VOpCode::Cons);
            }
            Exp::Constructor(con, exp) => {
                self.C(exp, env);
                self.code.push_back(VOpCode::Pack(*con));
            }
            Exp::App(e1, e2) => {
                self.T(e2, e1, env);
                self.code.push_back(VOpCode::App);
            }
            Exp::Abstraction(pat, exp) => {
                let tmp_lbl = self.queue.len() as i32;
                self.queue
                    .push_back((exp, env.push_front(EnvUnit::Constructed(pat))));
                self.code.push_back(VOpCode::Cur(-tmp_lbl - 1));
            }
            Exp::Conditional(e1, e2, e3) => {
                self.code.push_back(VOpCode::Push);
                self.C(e1, env.clone());
                self.code.push_back(VOpCode::Gotofalse(0));
                let gotofalse_lbl = self.code.len() - 1;
                self.C(e2, env.clone());
                self.code.push_back(VOpCode::Goto(0));
                let goto_lbl = self.code.len() - 1;

                self.code[gotofalse_lbl] = VOpCode::Gotofalse(self.code.len() as i32);
                self.C(e3, env);
                // self.code.push_back(VOpCode::Skip);
                // self.code[goto_lbl] = VOpCode::Goto((self.code.len() - 1) as i32);
                self.code[goto_lbl] = VOpCode::Goto(self.code.len() as i32);
            }
            Exp::Case(e, vec) => {
                self.code.push_back(VOpCode::Push);
                self.C(e, env.clone());
                let switch_lbl = self.code.len();
                for _ in vec {
                    self.code.push_back(VOpCode::Switch(0, 0));
                }

                let mut goto_lbls = vec![];

                for (idx, (c, p, e)) in vec.iter().enumerate() {
                    self.code[switch_lbl + idx] = VOpCode::Switch(*c, self.code.len() as i32);
                    self.C(e, env.push_front(EnvUnit::Constructed(p)));
                    if idx != vec.len() - 1 {
                        self.code.push_back(VOpCode::Goto(0));
                        goto_lbls.push(self.code.len() - 1);
                    }
                }

                // self.code.push_back(VOpCode::Skip);
                // let skip_lbl = self.code.len() - 1;
                let skip_lbl = self.code.len();
                for idx in goto_lbls {
                    self.code[idx] = VOpCode::Goto(skip_lbl as i32);
                }
            }
            Exp::Local(p, e1, e) => {
                self.code.push_back(VOpCode::Push);
                self.C(e1, env.clone());
                self.code.push_back(VOpCode::Cons);
                self.C(e, env.push_front(EnvUnit::Constructed(p)));
            }
            Exp::LocalRec(p1, e1, e) => {
                let lbl = self.queue.len() as i32;
                self.queue
                    .push_back((e1, env.push_front(EnvUnit::Annotated(p1, -lbl - 1))));
                self.C(e, env.push_front(EnvUnit::Annotated(p1, -lbl - 1)));
            }
        }
    }

    fn T(&mut self, e1: &'a Exp, e2: &'a Exp, env: Env<'a>) {
        self.code.push_back(VOpCode::Push);
        self.C(e1, env.clone());
        self.code.push_back(VOpCode::Swap);
        self.C(e2, env.clone());
    }

    fn R(&mut self, exp: &'a Exp, env: Env<'a>) {
        self.C(exp, env.clone());
        self.code.push_back(VOpCode::Return);
    }
}

#[cfg(test)]
mod tests {
    use gokart_ast::{Exp, Pat};
    use gokart_vm::{OpCode, PrimOp};

    use super::Compiler;

    fn g<'a>(o: &'a OpCode) -> Option<&'a OpCode> {
        Some(o)
    }

    fn evar(s: &'static str) -> Box<Exp> {
        Box::new(Exp::Variable(String::from(s)))
    }

    fn pvar(s: &'static str) -> Pat {
        Pat::Variable(String::from(s))
    }

    fn pvar_(s: &'static str) -> Box<Pat> {
        Box::new(pvar(s))
    }

    #[test]
    fn it_compiles_abstraction1() {
        // \x -> 1 + x
        let exp = Exp::Abstraction(
            pvar("x"),
            Box::new(Exp::Sys(
                String::from("IntPlus"),
                Box::new(Exp::ConstInt(1)),
                evar("x"),
            )),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Cur(2)));
        assert_eq!(iter.next(), g(&OpCode::Stop));
        // lbl:2
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(1)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntPlus)));
        assert_eq!(iter.next(), g(&OpCode::Return));
    }

    #[test]
    fn it_compiles_abstraction2() {
        // \f -> \x -> f (f x)
        let exp = Exp::Abstraction(
            pvar("f"),
            Box::new(Exp::Abstraction(
                pvar("x"),
                Box::new(Exp::App(
                    evar("f"),
                    Box::new(Exp::App(evar("f"), evar("x"))),
                )),
            )),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Cur(2)));
        assert_eq!(iter.next(), g(&OpCode::Stop));
        // lbl:2
        assert_eq!(iter.next(), g(&OpCode::Cur(4)));
        assert_eq!(iter.next(), g(&OpCode::Return));
        // lbl:4
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(1)));
        assert_eq!(iter.next(), g(&OpCode::App));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(1)));
        assert_eq!(iter.next(), g(&OpCode::App));
        assert_eq!(iter.next(), g(&OpCode::Return));
    }

    #[test]
    fn it_compiles_abstraction3() {
        // \(f, x) -> f (f x)
        let exp = Exp::Abstraction(
            Pat::Pair(Box::new(pvar("f")), Box::new(pvar("x"))),
            Box::new(Exp::App(
                evar("f"),
                Box::new(Exp::App(evar("f"), evar("x"))),
            )),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Cur(2)));
        assert_eq!(iter.next(), g(&OpCode::Stop));
        // lbl:2
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Rest(1)));
        assert_eq!(iter.next(), g(&OpCode::App));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Rest(1)));
        assert_eq!(iter.next(), g(&OpCode::App));
        assert_eq!(iter.next(), g(&OpCode::Return));
    }

    #[test]
    fn it_compiles_alternative() {
        // \n -> if n < 0 then n else -n
        let cond = Box::new(Exp::Sys(
            String::from("IntLe"),
            evar("n"),
            Box::new(Exp::ConstInt(0)),
        ));

        let eelse = Box::new(Exp::Sys(
            String::from("IntMinus"),
            Box::new(Exp::ConstInt(0)),
            evar("n"),
        ));

        let exp = Exp::Abstraction(
            pvar("n"),
            Box::new(Exp::Conditional(cond, evar("n"), eelse)),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Cur(2)));
        assert_eq!(iter.next(), g(&OpCode::Stop));
        // lbl:2
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(0)));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntLe)));
        assert_eq!(iter.next(), g(&OpCode::Gotofalse(11)));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Goto(16)));
        // lbl:11
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntMinus)));
        // lbl:16
        // assert_eq!(iter.next(), g(&OpCode::Skip));
        assert_eq!(iter.next(), g(&OpCode::Return));
    }

    #[test]
    fn it_compiles_algebraic_datatypes() {
        // data list = nil | cons (int, list)
        //              ^     ^
        // id:          0     1

        // \s -> case s of nil () -> nil () | cons (c, t) -> t

        let case1 = Box::new(Exp::Constructor(0, Box::new(Exp::EmptyTuple)));

        let case2 = evar("t");

        let exp = Exp::Abstraction(
            pvar("s"),
            Box::new(Exp::Case(
                evar("s"),
                vec![
                    (0, Pat::EmptyPattern, case1),
                    (1, Pat::Pair(pvar_("c"), pvar_("t")), case2),
                ],
            )),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Cur(2)));
        assert_eq!(iter.next(), g(&OpCode::Stop));
        // lbl:2
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Switch(0, 6)));
        assert_eq!(iter.next(), g(&OpCode::Switch(1, 9)));
        // lbl:6
        assert_eq!(iter.next(), g(&OpCode::Clear));
        assert_eq!(iter.next(), g(&OpCode::Pack(0)));
        assert_eq!(iter.next(), g(&OpCode::Goto(11)));
        // lbl:9
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        // lbl:11
        assert_eq!(iter.next(), g(&OpCode::Return));
    }

    #[test]
    fn it_compiles_local_definition() {
        // let a = 5 in a * a

        let exp = Exp::Local(
            pvar("a"),
            Box::new(Exp::ConstInt(5)),
            Box::new(Exp::Sys(String::from("IntMul"), evar("a"), evar("a"))),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(5)));
        assert_eq!(iter.next(), g(&OpCode::Cons));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntMul)));
    }

    #[test]
    fn it_compiles_local_recursive_definition() {
        let recdef = {
            let cond = Box::new(Exp::Sys(
                String::from("IntEq"),
                evar("n"),
                Box::new(Exp::ConstInt(0)),
            ));

            // 1 - (even (n - 1)) // means not (even (n - 1))
            let eelse = Box::new(Exp::Sys(
                String::from("IntMinus"),
                Box::new(Exp::ConstInt(1)),
                Box::new(Exp::App(
                    evar("even"),
                    Box::new(Exp::Sys(
                        String::from("IntMinus"),
                        evar("n"),
                        Box::new(Exp::ConstInt(1)),
                    )),
                )),
            ));

            // \n -> if n == 0 then true else not (even (n - 1))
            Exp::Abstraction(
                pvar("n"),
                Box::new(Exp::Conditional(cond, Box::new(Exp::ConstInt(1)), eelse)),
            )
        };

        let exp = Exp::LocalRec(
            pvar("even"),
            Box::new(recdef),
            Box::new(Exp::App(evar("even"), Box::new(Exp::ConstInt(56)))),
        );

        let opcodes = Compiler::compile(&exp);
        let mut iter = opcodes.iter();

        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(56)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Rest(0)));
        assert_eq!(iter.next(), g(&OpCode::Call(7)));
        assert_eq!(iter.next(), g(&OpCode::App));
        assert_eq!(iter.next(), g(&OpCode::Stop));
        // lbl:7
        assert_eq!(iter.next(), g(&OpCode::Cur(9)));
        assert_eq!(iter.next(), g(&OpCode::Return));
        // lbl:9
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(0)));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntEq)));
        assert_eq!(iter.next(), g(&OpCode::Gotofalse(18)));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(1)));
        assert_eq!(iter.next(), g(&OpCode::Goto(32)));
        // lbl:18
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(1)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Push));
        assert_eq!(iter.next(), g(&OpCode::Acc(0)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::QuoteInt(1)));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntMinus)));
        assert_eq!(iter.next(), g(&OpCode::Swap));
        assert_eq!(iter.next(), g(&OpCode::Rest(1)));
        assert_eq!(iter.next(), g(&OpCode::Call(7)));
        assert_eq!(iter.next(), g(&OpCode::App));
        assert_eq!(iter.next(), g(&OpCode::Prim(PrimOp::IntMinus)));
        // lbl:32
        assert_eq!(iter.next(), g(&OpCode::Return));
        assert_eq!(iter.next(), None);
    }
}
