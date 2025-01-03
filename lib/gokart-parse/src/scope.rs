use crate::{
    ast::{Tpl, TplNode},
    err::{LogicErr, LogicRes},
    token::Span,
};
use gokart_core::{Ctor, Var};
use std::{
    collections::{hash_set, HashMap, HashSet},
    ops::Deref,
};

#[derive(Debug)]
pub struct NameTable<'a> {
    items: HashMap<&'a str, Vec<usize>>,
    cnt: usize,
}

impl<'a> NameTable<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            cnt: 0,
        }
    }

    pub fn get(&self, s: &Span<'a>) -> LogicRes<'a, usize> {
        let name = s.fragment();
        match self.items.get(name) {
            None => Err(LogicErr::new(*s, "todo")),
            Some(vals) => match vals.last() {
                None => Err(LogicErr::new(*s, "todo")),
                Some(item) => Ok(*item),
            },
        }
    }

    pub fn push(&mut self, s: &Span<'a>) -> LogicRes<'a, ()> {
        let idx = self.cnt;
        self.cnt += 1;
        self.items
            .entry(s.fragment())
            .and_modify(|x| x.push(idx))
            .or_insert(vec![idx]);
        Ok(())
    }

    pub fn pop(&mut self, s: &Span<'a>) -> LogicRes<'a, ()> {
        match self.items.get_mut(s.fragment()) {
            None => Err(LogicErr::new(*s, "todo")),
            Some(vals) => match vals.pop() {
                None => Err(LogicErr::new(*s, "todo")),
                Some(_) => Ok(()),
            },
        }
    }
}

#[derive(Debug)]
pub struct Names<'a> {
    items: HashSet<Span<'a>>,
}

impl<'a> Names<'a> {
    pub fn new() -> Self {
        Names {
            items: HashSet::new(),
        }
    }

    fn add(&mut self, item: &Span<'a>) -> LogicRes<'a, ()> {
        match self.items.insert(*item) {
            true => Ok(()),
            false => Err(LogicErr::new(*item, "todo")),
        }
    }

    pub fn collect(mut self, tpl: &Tpl<'a>) -> LogicRes<'a, Self> {
        match tpl.deref() {
            TplNode::Var(name) => {
                self.add(&name.span)?;
                Ok(self)
            }
            TplNode::Empty => Ok(self),
            TplNode::Seq(tpls) => tpls.iter().fold(Ok(self), |acc, tpl| acc?.collect(tpl)),
            TplNode::As(var, tpl) => {
                self.add(&var.span)?;
                self.collect(tpl)
            }
        }
    }

    #[inline]
    pub fn iter<'b>(&'b self) -> hash_set::Iter<'b, Span<'a>> {
        self.items.iter()
    }

    #[inline]
    pub fn with_scope<F, T>(&self, sc: &mut Scope<'a>, f: F) -> LogicRes<'a, T>
    where
        F: Fn(&mut Scope<'a>) -> LogicRes<'a, T>,
    {
        sc.push_vars(self)?;
        let res = f(sc);
        sc.pop_vars(self)?;
        res
    }
}

pub struct Scope<'a> {
    vars: NameTable<'a>,
    ctors: NameTable<'a>,
}

impl<'a> Scope<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            vars: NameTable::new(),
            ctors: NameTable::new(),
        }
    }

    #[inline]
    pub fn var(&self, s: &Span<'a>) -> LogicRes<'a, Var> {
        self.vars.get(s)
    }

    pub fn push_vars(&mut self, names: &Names<'a>) -> LogicRes<'a, ()> {
        names.iter().fold(Ok(()), |acc, name| {
            acc?;
            self.vars.push(name)
        })
    }

    pub fn pop_vars(&mut self, names: &Names<'a>) -> LogicRes<'a, ()> {
        names.iter().fold(Ok(()), |acc, name| {
            acc?;
            self.vars.pop(name)
        })
    }

    #[inline]
    pub fn ctor(&self, s: &Span<'a>) -> LogicRes<'a, Ctor> {
        self.ctors.get(s)
    }
}
