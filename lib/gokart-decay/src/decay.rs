use core::panic;

use crate::{ctx::Ctx, state::State};
use gokart_core::{
    Abs, App, AsTpl, Ast, BinOp, Case, ConTerm, Cond, Def, EmptyTerm, EmptyTpl, Exp, Let, Letrec,
    Lit, Name, NullOp, Opr, PairTerm, PairTpl, Pat, Predef, Term, Tpl, TypeDef,
};

trait Decay<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp;
}

impl<'a> Decay<'a> for EmptyTerm<'a> {
    fn decay(&self, _ctx: &Ctx<'a>, _st: &mut State) -> Exp {
        Exp::Empty
    }
}

impl<'a> Decay<'a> for PairTerm<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        Exp::Pair(
            self.left.decay(ctx, st).ptr(),
            self.right.decay(ctx, st).ptr(),
        )
    }
}

impl<'a> Decay<'a> for Name<'a> {
    fn decay(&self, ctx: &Ctx<'a>, _st: &mut State) -> Exp {
        let idx = ctx.var(self);
        Exp::Var(idx)
    }
}

impl<'a> Decay<'a> for Lit<'a> {
    fn decay(&self, _ctx: &Ctx<'a>, _st: &mut State) -> Exp {
        let null_op = match self {
            Lit::Int(lit) => NullOp::IntLit(lit.val),
            Lit::Double(lit) => NullOp::DoubleLit(lit.val),
            Lit::Str(lit) => NullOp::StrLit(String::from(lit.val)),
        };
        Exp::Sys0(null_op)
    }
}

impl<'a> Decay<'a> for ConTerm<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        let tag = ctx.tag(&self.name);
        Exp::Con(tag, self.body.decay(ctx, st).ptr())
    }
}

impl<'a> Decay<'a> for Opr<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        // todo: use (type) hints
        // or (maybe) determine binop kind on verify step
        // and use Option<BinOp> field in Opr

        let raw = self.name.val;
        let bin_op = match raw {
            "+" => BinOp::IntPlus,
            "*" => BinOp::IntMul,
            "-" => BinOp::IntMinus,
            "/" => BinOp::IntDiv,
            "<" => BinOp::IntLt,
            "<=" => BinOp::IntLe,
            "==" => BinOp::IntEq,
            "!=" => BinOp::IntNe,
            ">" => BinOp::IntGt,
            ">=" => BinOp::IntGe,
            _ => panic!("Unknown binary operation: {raw}"),
        };

        Exp::Sys2(
            bin_op,
            self.left.decay(ctx, st).ptr(),
            self.right.decay(ctx, st).ptr(),
        )
    }
}

impl<'a> Decay<'a> for App<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        Exp::App(
            self.head.decay(ctx, st).ptr(),
            self.body.decay(ctx, st).ptr(),
        )
    }
}

impl<'a> Decay<'a> for Cond<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        Exp::Cond(
            self.cond.decay(ctx, st).ptr(),
            self.left.decay(ctx, st).ptr(),
            self.right.decay(ctx, st).ptr(),
        )
    }
}

impl<'a> Decay<'a> for Abs<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        let ctx_ = ctx.add_var(&self.arg, st);
        Exp::Abs(self.arg.as_pat(&ctx_, st), self.body.decay(&ctx_, st).ptr())
    }
}

impl<'a> Decay<'a> for Case<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        let body = self.cond.decay(ctx, st);
        let branches = self
            .branches
            .iter()
            .map(|branch| {
                let ctx_ = ctx.add_tpl(&branch.tpl, st);
                let tag = ctx_.tag(&branch.con);
                let pat = branch.tpl.as_pat(&ctx_, st);
                let exp = branch.body.decay(&ctx_, st);

                (tag, pat, exp)
            })
            .collect::<Vec<_>>();

        Exp::Case(body.ptr(), branches)
    }
}

impl<'a> Decay<'a> for Let<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        let ctx_ = ctx.add_tpl(&self.tpl, st);
        let pat = self.tpl.as_pat(&ctx_, st);
        let exp = self.term.decay(ctx, st);
        let body = self.body.decay(&ctx_, st);
        Exp::Let(pat, exp.ptr(), body.ptr())
    }
}

impl<'a> Decay<'a> for Letrec<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        let ctx_ = ctx.add_tpl(&self.tpl, st);
        let pat = self.tpl.as_pat(&ctx_, st);
        let exp = self.term.decay(&ctx_, st);
        let body = self.body.decay(&ctx_, st);
        Exp::Letrec(pat, exp.ptr(), body.ptr())
    }
}

impl<'a, 'b> Decay<'a> for Predef<'a, 'b> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        let (ctx_, pat, exp) = self.items.iter().fold(
            (ctx.clone(), Pat::Empty, Exp::Empty),
            |(ctx_, pat, exp), (name, newfunc)| {
                let new_ctx = ctx_.add_var(name, st);
                let new_pat = Pat::Pair(pat.ptr(), Pat::Var(new_ctx.var(name)).ptr());

                let new_exp = Exp::Pair(exp.ptr(), newfunc.clone().ptr());

                (new_ctx, new_pat, new_exp)
            },
        );

        Exp::Let(pat, exp.ptr(), self.body.decay(&ctx_, st).ptr())
    }
}

impl<'a> Decay<'a> for Term<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> Exp {
        match self {
            Term::Empty(term) => term.decay(ctx, st),
            Term::Pair(term) => term.decay(ctx, st),
            Term::Var(term) => term.decay(ctx, st),
            Term::Lit(term) => term.decay(ctx, st),
            Term::Con(term) => term.decay(ctx, st),
            Term::Opr(term) => term.decay(ctx, st),
            Term::App(term) => term.decay(ctx, st),
            Term::Cond(term) => term.decay(ctx, st),
            Term::Abs(term) => term.decay(ctx, st),
            Term::Case(term) => term.decay(ctx, st),
            Term::Let(term) => term.decay(ctx, st),
            Term::Letrec(term) => term.decay(ctx, st),
        }
    }
}

trait AsPat<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> Pat;
}

impl<'a> AsPat<'a> for EmptyTpl<'a> {
    fn as_pat(&self, _ctx: &Ctx<'a>, _st: &mut State) -> Pat {
        Pat::Empty
    }
}

impl<'a> AsPat<'a> for Name<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, _st: &mut State) -> Pat {
        let idx = ctx.var(self);
        Pat::Var(idx)
    }
}

impl<'a> AsPat<'a> for PairTpl<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> Pat {
        Pat::Pair(
            self.left.as_pat(ctx, st).ptr(),
            self.right.as_pat(ctx, st).ptr(),
        )
    }
}

impl<'a> AsPat<'a> for AsTpl<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> Pat {
        let idx = ctx.var(&self.name);
        Pat::Layer(idx, self.tpl.as_pat(ctx, st).ptr())
    }
}

impl<'a> AsPat<'a> for Tpl<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> Pat {
        match self {
            Tpl::Empty(tpl) => tpl.as_pat(ctx, st),
            Tpl::Var(tpl) => tpl.as_pat(ctx, st),
            Tpl::Pair(tpl) => tpl.as_pat(ctx, st),
            Tpl::As(tpl) => tpl.as_pat(ctx, st),
        }
    }
}

trait Apply<'a> {
    fn apply(&self, ctx: Ctx<'a>, st: &mut State) -> Ctx<'a>;
}

impl<'a> Apply<'a> for TypeDef<'a> {
    fn apply(&self, ctx: Ctx<'a>, st: &mut State) -> Ctx<'a> {
        self.cons
            .iter()
            .fold(ctx, |ctx_, con| ctx_.add_tag(&con.name, st))
    }
}

impl<'a> Apply<'a> for Def<'a> {
    fn apply(&self, ctx: Ctx<'a>, st: &mut State) -> Ctx<'a> {
        match self {
            Def::TypeDef(type_def) => type_def.apply(ctx, st),
        }
    }
}

pub fn decay(ast: &Ast) -> Exp {
    let mut st = State::new();
    let mut ctx = Ctx::new();

    let Ast { defs, body, .. } = ast;
    for def in defs.iter() {
        ctx = def.apply(ctx, &mut st);
    }
    Predef::new(body).decay(&ctx, &mut st)
}
