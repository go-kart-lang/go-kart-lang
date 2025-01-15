use gokart_core::{BinOp, Counter, Exp, Pat, Predef, Tag, Var, VarName};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ctx<'a> {
    var_cnt: Counter,
    tag_cnt: Counter,
    vars: HashMap<VarName<'a>, Var>,
    tags: HashMap<VarName<'a>, Tag>,
    funcs: HashMap<Var, Exp>,
    oprs: HashMap<VarName<'a>, BinOp>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn with_predef() -> Self {
        let tys = Predef::types(&mut Counter::default());
        let mut var_cnt = Counter::default();
        let mut funcs = HashMap::new();
        let mut vars = HashMap::new();

        for func in Predef::funcs(&tys).into_iter() {
            let idx = var_cnt.step();
            funcs.insert(idx, func.exp);
            vars.insert(func.name, idx);
        }

        let oprs = Predef::oprs(&tys)
            .into_iter()
            .map(|opr| (opr.name, opr.bin_op))
            .collect();

        Self {
            var_cnt,
            tag_cnt: Counter::default(),
            vars,
            tags: HashMap::new(),
            funcs,
            oprs,
        }
    }

    // because we check everything on verify step, so now we are
    // confident that all names/constructors/operations are defined

    #[inline]
    pub fn var(&self, name: VarName<'a>) -> Var {
        *self.vars.get(name).unwrap()
    }

    #[inline]
    pub fn push_var(&mut self, name: VarName<'a>) -> Option<Var> {
        self.vars.insert(name, self.var_cnt.step())
    }

    #[inline]
    pub fn pop_var(&mut self, name: VarName<'a>, prev: Option<Var>) {
        match prev {
            Some(x) => self.vars.insert(name, x),
            None => self.vars.remove(name),
        };
    }

    #[inline]
    pub fn push_vars(&mut self, names: &[VarName<'a>]) -> Vec<Option<Var>> {
        names.iter().map(|name| self.push_var(name)).collect()
    }

    #[inline]
    pub fn pop_vars(&mut self, names: &[VarName<'a>], prevs: Vec<Option<Var>>) {
        names
            .iter()
            .zip(prevs)
            .for_each(|(name, prev)| self.pop_var(name, prev))
    }

    #[inline]
    pub fn tag(&self, name: VarName<'a>) -> Tag {
        *self.tags.get(name).unwrap()
    }

    #[inline]
    pub fn add_tag(&mut self, name: VarName<'a>) -> Option<Tag> {
        self.tags.insert(name, self.tag_cnt.step())
    }

    #[inline]
    pub fn opr(&mut self, name: VarName<'a>) -> BinOp {
        *self.oprs.get(name).unwrap()
    }

    #[inline]
    pub fn wrap(self, body: Exp) -> Exp {
        let (pat, exp) =
            self.funcs
                .into_iter()
                .fold((Pat::Empty, Exp::Empty), |(pat, exp), (idx, func)| {
                    let new_pat = Pat::Pair(pat.ptr(), Pat::Var(idx).ptr());
                    let new_exp = Exp::Pair(exp.ptr(), func.ptr());
                    (new_pat, new_exp)
                });
        Exp::Let(pat, exp.ptr(), body.ptr())
    }
}
