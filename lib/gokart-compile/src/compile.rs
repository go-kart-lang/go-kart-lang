use std::collections::VecDeque;

use crate::ctx::{Ctx, DLabel, Env, EnvUnit, VOpCode};
use gokart_core::{Exp, ExpPtr, Label, OpCode, Pat, Var};

trait Compile<'a> {
    fn compile(self, ctx: &mut Ctx<'a>, env: Env<'a>);
}

// aka T-rule
impl<'a> Compile<'a> for (&'a ExpPtr, &'a ExpPtr) {
    fn compile(self, ctx: &mut Ctx<'a>, env: Env<'a>) {
        ctx.code.push_back(VOpCode::Push);
        self.0.compile(ctx, env.clone());
        ctx.code.push_back(VOpCode::Swap);
        self.1.compile(ctx, env);
    }
}

// aka E-rule
impl<'a> Compile<'a> for Var {
    fn compile(self, ctx: &mut Ctx<'a>, env: Env<'a>) {
        let mut lvl = 0;
        let mut iter = env.iter();

        loop {
            match iter.next() {
                None => unreachable!(
                    "The correctness of the pattern is checked at the verification stage"
                ),
                Some(EnvUnit::Con(pat)) => {
                    if let Some(mut code) = compile_pattern(self, pat) {
                        code.push_front(VOpCode::Acc(lvl));
                        ctx.code.extend(code);
                        break;
                    } else {
                        lvl += 1;
                    }
                }
                Some(EnvUnit::Lab(pat, lbl)) => {
                    if let Some(mut code) = compile_pattern(self, pat) {
                        code.push_front(VOpCode::Call(*lbl));
                        code.push_front(VOpCode::Rest(lvl));
                        ctx.code.extend(code);
                        break;
                    }
                }
            }
        }
    }
}

fn compile_pattern(var: Var, pat: &Pat) -> Option<VecDeque<VOpCode>> {
    match pat {
        Pat::Empty => None,
        Pat::Var(x) => (var == *x).then_some(VecDeque::new()),
        Pat::Pair(left, right) => {
            if let Some(mut code) = compile_pattern(var, left) {
                code.push_front(VOpCode::Rest(1));
                Some(code)
            } else if let Some(mut code) = compile_pattern(var, right) {
                code.push_front(VOpCode::Acc(0));
                Some(code)
            } else {
                None
            }
        }
        Pat::Layer(x, pat) => (var == *x)
            .then_some(VecDeque::new())
            .or(compile_pattern(var, pat)),
    }
}

impl<'a> Compile<'a> for &'a Exp {
    fn compile(self, ctx: &mut Ctx<'a>, env: Env<'a>) {
        match self {
            Exp::Empty => {
                ctx.code.push_back(VOpCode::Clear);
            }
            Exp::Var(var) => var.compile(ctx, env),
            Exp::Sys0(op) => {
                ctx.code.push_back(VOpCode::Sys0(op.clone()));
            }
            Exp::Sys1(op, exp) => {
                exp.compile(ctx, env);
                ctx.code.push_back(VOpCode::Sys1(*op));
            }
            Exp::Sys2(op, left, right) => {
                (left, right).compile(ctx, env);
                ctx.code.push_back(VOpCode::Sys2(*op));
            }
            Exp::Pair(left, right) => {
                (left, right).compile(ctx, env);
                ctx.code.push_back(VOpCode::Cons);
            }
            Exp::Con(tag, body) => {
                body.compile(ctx, env);
                ctx.code.push_back(VOpCode::Pack(*tag));
            }
            Exp::App(head, body) => {
                (body, head).compile(ctx, env);
                ctx.code.push_back(VOpCode::App);
            }
            Exp::Abs(pat, exp) => {
                let lbl = DLabel::Defer(ctx.queue.len());
                let new_env = env.push_front(EnvUnit::Con(pat));
                ctx.queue.push_back((exp, new_env));
                ctx.code.push_back(VOpCode::Cur(lbl));
            }
            Exp::Cond(cond, left, right) => {
                ctx.code.push_back(VOpCode::Push);
                cond.compile(ctx, env.clone());

                let gtf_idx = ctx.code.push_dummy();
                left.compile(ctx, env.clone());

                let gt_idx = ctx.code.push_dummy();
                ctx.code[gtf_idx] = VOpCode::GotoFalse(ctx.code.cur_label());
                right.compile(ctx, env);
                ctx.code[gt_idx] = VOpCode::Goto(ctx.code.cur_label())
            }
            Exp::Case(cond, branches) => {
                ctx.code.push_back(VOpCode::Push);
                cond.compile(ctx, env.clone());

                let sw_idx = ctx.code.push_dummies(branches.len());
                let mut gt_idxs = Vec::new();

                for (idx, (tag, pat, exp)) in branches.iter().enumerate() {
                    ctx.code[sw_idx + idx] = VOpCode::Switch(*tag, ctx.code.cur_label());
                    let new_env = env.push_front(EnvUnit::Con(pat));
                    exp.compile(ctx, new_env);
                    if idx != branches.len() - 1 {
                        gt_idxs.push(ctx.code.push_dummy());
                    }
                }

                let skip_lbl = ctx.code.cur_label();
                for idx in gt_idxs {
                    ctx.code[idx] = VOpCode::Goto(skip_lbl);
                }
            }
            Exp::Let(pat, exp, body) => {
                ctx.code.push_back(VOpCode::Push);
                exp.compile(ctx, env.clone());
                ctx.code.push_back(VOpCode::Cons);
                body.compile(ctx, env.push_front(EnvUnit::Con(pat)));
            }
            Exp::Letrec(pat, exp, body) => {
                let lbl = DLabel::Defer(ctx.queue.len());
                let new_env = env.push_front(EnvUnit::Lab(pat, lbl));
                ctx.queue.push_back((exp, new_env.clone()));
                body.compile(ctx, new_env);
            }
        }
    }
}

fn make_labels(ctx: &mut Ctx) -> Vec<Label> {
    let mut res = Vec::new();
    let mut idx = 0;

    while idx < ctx.queue.len() {
        let (exp, env) = &ctx.queue[idx];
        idx += 1;

        res.push(ctx.code.len() as u64);
        exp.compile(ctx, env.clone());
        ctx.code.push_back(VOpCode::Return);
    }

    res
}

pub fn compile(exp: &Exp) -> Vec<OpCode> {
    let mut ctx = Ctx::new();
    let env = Env::new();

    exp.compile(&mut ctx, env);
    ctx.code.push_back(VOpCode::Stop);

    let labels = make_labels(&mut ctx);
    ctx.code.transform(&labels)
}

#[cfg(test)]
mod tests {
    use crate::compile;
    use gokart_core::{BinOp, Exp, ExpPtr, GOpCode, Int, NullOp, Pat, PatPtr, Var};
    use GOpCode::*;

    #[inline]
    fn evar_(var: Var) -> Exp {
        Exp::Var(var)
    }

    #[inline]
    fn evar(var: Var) -> ExpPtr {
        evar_(var).ptr()
    }

    #[inline]
    fn pvar_(var: Var) -> Pat {
        Pat::Var(var)
    }

    #[inline]
    fn pvar(var: Var) -> PatPtr {
        pvar_(var).ptr()
    }

    #[inline]
    fn eint(val: Int) -> ExpPtr {
        Exp::Sys0(NullOp::IntLit(val)).ptr()
    }

    #[test]
    fn ok_abstraction_1() {
        // \x -> 42 + x
        let exp = Exp::Abs(pvar_(1), Exp::Sys2(BinOp::IntPlus, eint(42), evar(1)).ptr());

        assert_eq!(
            compile(&exp),
            [
                Cur(2),
                Stop,
                // lbl:2
                Push,
                Sys0(NullOp::IntLit(42)),
                Swap,
                Acc(0),
                Sys2(BinOp::IntPlus),
                Return
            ]
        );
    }

    #[test]
    fn ok_abstraction_2() {
        // \f -> \x -> f (f x)
        let exp = Exp::Abs(
            pvar_(1),
            Exp::Abs(
                pvar_(2),
                Exp::App(evar(1), Exp::App(evar(1), evar(2)).ptr()).ptr(),
            )
            .ptr(),
        );

        assert_eq!(
            compile(&exp),
            [
                Cur(2),
                Stop,
                // lbl:2
                Cur(4),
                Return,
                // lbl:4
                Push,
                Push,
                Acc(0),
                Swap,
                Acc(1),
                App,
                Swap,
                Acc(1),
                App,
                Return,
            ]
        )
    }

    #[test]
    fn ok_abstraction_3() {
        // \(f, x) -> f (f x)
        let exp = Exp::Abs(
            Pat::Pair(pvar(1), pvar(2)),
            Exp::App(evar(1), Exp::App(evar(1), evar(2)).ptr()).ptr(),
        );

        assert_eq!(
            compile(&exp),
            [
                Cur(2),
                Stop,
                // lbl:2
                Push,
                Push,
                Acc(0),
                Acc(0),
                Swap,
                Acc(0),
                Rest(1),
                App,
                Swap,
                Acc(0),
                Rest(1),
                App,
                Return,
            ]
        );
    }

    #[test]
    fn ok_alternative() {
        // \n -> if n < 0 then n else -n
        let cond = Exp::Sys2(BinOp::IntLt, evar(1), eint(0)).ptr();
        let on_else = Exp::Sys2(BinOp::IntMinus, eint(0), evar(1)).ptr();
        let exp = Exp::Abs(pvar_(1), Exp::Cond(cond, evar(1), on_else).ptr());

        assert_eq!(
            compile(&exp),
            [
                Cur(2),
                Stop,
                // lbl:2
                Push,
                Push,
                Acc(0),
                Swap,
                Sys0(NullOp::IntLit(0)),
                Sys2(BinOp::IntLt),
                GotoFalse(11),
                Acc(0),
                Goto(16),
                // lbl:11
                Push,
                Sys0(NullOp::IntLit(0)),
                Swap,
                Acc(0),
                Sys2(BinOp::IntMinus),
                // lbl:16
                Return,
            ]
        );
    }

    #[test]
    fn ok_adt() {
        // data List = Nil | Cons (Int, List)
        //              ^     ^
        // id:          0     1

        // \s -> case s of Nil () -> Nil () | Cons (c, t) -> t
        let case1 = Exp::Con(0, Exp::Empty.ptr());
        let case2 = evar_(3);
        let exp = Exp::Abs(
            pvar_(1),
            Exp::Case(
                evar(1),
                vec![
                    (0, Pat::Empty, case1),
                    (1, Pat::Pair(pvar(2), pvar(3)), case2),
                ],
            )
            .ptr(),
        );

        assert_eq!(
            compile(&exp),
            [
                Cur(2),
                Stop,
                // lbl:2
                Push,
                Acc(0),
                Switch(0, 6),
                Switch(1, 9),
                // lbl:6
                Clear,
                Pack(0),
                Goto(11),
                // lbl:9
                Acc(0),
                Acc(0),
                // lbl:11
                Return,
            ]
        )
    }

    #[test]
    fn ok_local_def() {
        // let a = 5 in a * a
        let exp = Exp::Let(
            pvar_(1),
            eint(5),
            Exp::Sys2(BinOp::IntMul, evar(1), evar(1)).ptr(),
        );

        assert_eq!(
            compile(&exp),
            [
                Push,
                Sys0(NullOp::IntLit(5)),
                Cons,
                Push,
                Acc(0),
                Swap,
                Acc(0),
                Sys2(BinOp::IntMul),
                Stop
            ]
        );
    }

    #[test]
    fn ok_local_rec_def() {
        let cond = Exp::Sys2(BinOp::IntEq, evar(1), eint(0)).ptr();
        // 1 - (even (n - 1)) == not (even (n - 1))
        let on_else = Exp::Sys2(
            BinOp::IntMinus,
            eint(1),
            Exp::App(
                evar(2),
                Box::new(Exp::Sys2(BinOp::IntMinus, evar(1), eint(1))),
            )
            .ptr(),
        )
        .ptr();
        // \n -> if n == 0 then true else not (even (n - 1))
        let recdef = Exp::Abs(pvar_(1), Exp::Cond(cond, eint(1), on_else).ptr()).ptr();
        let exp = Exp::Letrec(pvar_(2), recdef, Exp::App(evar(2), eint(56)).ptr());

        assert_eq!(
            compile(&exp),
            [
                Push,
                Sys0(NullOp::IntLit(56)),
                Swap,
                Rest(0),
                Call(7),
                App,
                Stop,
                // lbl:7
                Cur(9),
                Return,
                // lbl:9
                Push,
                Push,
                Acc(0),
                Swap,
                Sys0(NullOp::IntLit(0)),
                Sys2(BinOp::IntEq),
                GotoFalse(18),
                Sys0(NullOp::IntLit(1)),
                Goto(32),
                // lbl:18
                Push,
                Sys0(NullOp::IntLit(1)),
                Swap,
                Push,
                Push,
                Acc(0),
                Swap,
                Sys0(NullOp::IntLit(1)),
                Sys2(BinOp::IntMinus),
                Swap,
                Rest(1),
                Call(7),
                App,
                Sys2(BinOp::IntMinus),
                // lbl:32
                Return,
            ]
        );
    }
}
