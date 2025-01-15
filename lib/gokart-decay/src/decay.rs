use crate::ctx::Ctx;
use gokart_core::{
    Abs, App, AsTpl, Ast, Case, ConTerm, Cond, Def, EmptyTerm, EmptyTpl, Exp, Let, Letrec, Lit,
    Name, NullOp, Opr, PairTerm, PairTpl, Pat, Term, Tpl, TypeDef, VarName,
};

trait Decay<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp;
}

impl<'a> Decay<'a> for EmptyTerm<'a> {
    fn decay(&self, _ctx: &mut Ctx<'a>) -> Exp {
        Exp::Empty
    }
}

impl<'a> Decay<'a> for PairTerm<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        Exp::Pair(self.left.decay(ctx).ptr(), self.right.decay(ctx).ptr())
    }
}

impl<'a> Decay<'a> for Name<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let idx = ctx.var(self);
        Exp::Var(idx)
    }
}

impl<'a> Decay<'a> for Lit<'a> {
    fn decay(&self, _ctx: &mut Ctx<'a>) -> Exp {
        let null_op = match self {
            Lit::Int(lit) => NullOp::IntLit(lit.val),
            Lit::Double(lit) => NullOp::DoubleLit(lit.val),
            Lit::Str(lit) => NullOp::StrLit(String::from(lit.val)),
        };
        Exp::Sys0(null_op)
    }
}

impl<'a> Decay<'a> for ConTerm<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let tag = ctx.tag(&self.name);
        Exp::Con(tag, self.body.decay(ctx).ptr())
    }
}

impl<'a> Decay<'a> for Opr<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let bin_op = ctx.opr(&self.name);
        Exp::Sys2(
            bin_op,
            self.left.decay(ctx).ptr(),
            self.right.decay(ctx).ptr(),
        )
    }
}

impl<'a> Decay<'a> for App<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        Exp::App(self.head.decay(ctx).ptr(), self.body.decay(ctx).ptr())
    }
}

impl<'a> Decay<'a> for Cond<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        Exp::Cond(
            self.cond.decay(ctx).ptr(),
            self.left.decay(ctx).ptr(),
            self.right.decay(ctx).ptr(),
        )
    }
}

impl<'a> Decay<'a> for Abs<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let prev = ctx.push_var(&self.arg);
        let res = Exp::Abs(self.arg.as_pat(ctx), self.body.decay(ctx).ptr());
        ctx.pop_var(&self.arg, prev);
        res
    }
}

impl<'a> Decay<'a> for Case<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let body = self.cond.decay(ctx);
        let branches = self
            .branches
            .iter()
            .map(|branch| {
                let names = branch.tpl.get_names();
                let prevs = ctx.push_vars(&names);

                let tag = ctx.tag(&branch.con);
                let pat = branch.tpl.as_pat(ctx);
                let exp = branch.body.decay(ctx);

                ctx.pop_vars(&names, prevs);
                (tag, pat, exp)
            })
            .collect::<Vec<_>>();

        Exp::Case(body.ptr(), branches)
    }
}

impl<'a> Decay<'a> for Let<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let exp = self.term.decay(ctx);

        let names = self.tpl.get_names();
        let prevs = ctx.push_vars(&names);

        let pat = self.tpl.as_pat(ctx);
        let body = self.body.decay(ctx);
        let res = Exp::Let(pat, exp.ptr(), body.ptr());

        ctx.pop_vars(&names, prevs);
        res
    }
}

impl<'a> Decay<'a> for Letrec<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        let names = self.tpl.get_names();
        let prevs = ctx.push_vars(&names);

        let pat = self.tpl.as_pat(ctx);
        let exp = self.term.decay(ctx);
        let body = self.body.decay(ctx);
        let res = Exp::Letrec(pat, exp.ptr(), body.ptr());

        ctx.pop_vars(&names, prevs);
        res
    }
}

impl<'a> Decay<'a> for Term<'a> {
    fn decay(&self, ctx: &mut Ctx<'a>) -> Exp {
        match self {
            Term::Empty(term) => term.decay(ctx),
            Term::Pair(term) => term.decay(ctx),
            Term::Var(term) => term.decay(ctx),
            Term::Lit(term) => term.decay(ctx),
            Term::Con(term) => term.decay(ctx),
            Term::Opr(term) => term.decay(ctx),
            Term::App(term) => term.decay(ctx),
            Term::Cond(term) => term.decay(ctx),
            Term::Abs(term) => term.decay(ctx),
            Term::Case(term) => term.decay(ctx),
            Term::Let(term) => term.decay(ctx),
            Term::Letrec(term) => term.decay(ctx),
        }
    }
}

trait AsPat<'a> {
    fn as_pat(&self, ctx: &mut Ctx<'a>) -> Pat;
}

impl<'a> AsPat<'a> for EmptyTpl<'a> {
    fn as_pat(&self, _ctx: &mut Ctx<'a>) -> Pat {
        Pat::Empty
    }
}

impl<'a> AsPat<'a> for Name<'a> {
    fn as_pat(&self, ctx: &mut Ctx<'a>) -> Pat {
        let idx = ctx.var(self);
        Pat::Var(idx)
    }
}

impl<'a> AsPat<'a> for PairTpl<'a> {
    fn as_pat(&self, ctx: &mut Ctx<'a>) -> Pat {
        Pat::Pair(self.left.as_pat(ctx).ptr(), self.right.as_pat(ctx).ptr())
    }
}

impl<'a> AsPat<'a> for AsTpl<'a> {
    fn as_pat(&self, ctx: &mut Ctx<'a>) -> Pat {
        let idx = ctx.var(&self.name);
        Pat::Layer(idx, self.tpl.as_pat(ctx).ptr())
    }
}

impl<'a> AsPat<'a> for Tpl<'a> {
    fn as_pat(&self, ctx: &mut Ctx<'a>) -> Pat {
        match self {
            Tpl::Empty(tpl) => tpl.as_pat(ctx),
            Tpl::Var(tpl) => tpl.as_pat(ctx),
            Tpl::Pair(tpl) => tpl.as_pat(ctx),
            Tpl::As(tpl) => tpl.as_pat(ctx),
        }
    }
}

trait GetNames<'a> {
    fn get_names(&self) -> Vec<VarName<'a>>;
}

impl<'a> GetNames<'a> for Tpl<'a> {
    fn get_names(&self) -> Vec<VarName<'a>> {
        fn go<'b>(tpl: &Tpl<'b>, names: &mut Vec<VarName<'b>>) {
            match tpl {
                Tpl::Empty(_) => (),
                Tpl::Var(name) => names.push(name.val),
                Tpl::Pair(tpl) => {
                    go(&tpl.left, names);
                    go(&tpl.right, names);
                }
                Tpl::As(tpl) => {
                    names.push(tpl.name.val);
                    go(&tpl.tpl, names);
                }
            }
        }

        let mut res = Vec::new();
        go(self, &mut res);
        res
    }
}

trait Apply<'a> {
    fn apply(&self, ctx: &mut Ctx<'a>);
}

impl<'a> Apply<'a> for TypeDef<'a> {
    fn apply(&self, ctx: &mut Ctx<'a>) {
        for con in self.cons.iter() {
            ctx.add_tag(&con.name);
        }
    }
}

impl<'a> Apply<'a> for Def<'a> {
    fn apply(&self, ctx: &mut Ctx<'a>) {
        match self {
            Def::TypeDef(type_def) => type_def.apply(ctx),
        }
    }
}

pub fn decay(ast: &Ast) -> Exp {
    let mut ctx = Ctx::with_predef();

    for def in ast.defs.iter() {
        def.apply(&mut ctx);
    }

    let res = ast.body.decay(&mut ctx);
    ctx.wrap(res)
}
