use crate::{
    err::{LogicErr, LogicRes},
    token::Span,
};
use gokart_core::{Ctor, Exp, Var};
use std::collections::HashMap;

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

    pub fn push(&mut self, s: &Span<'a>) -> LogicRes<'a, usize> {
        let idx = self.cnt;
        self.cnt += 1;
        self.items
            .entry(s.fragment())
            .and_modify(|x| x.push(idx))
            .or_insert(vec![idx]);
        Ok(idx)
    }

    pub fn pop(&mut self, s: &Span<'a>) -> LogicRes<'a, usize> {
        match self.items.get_mut(s.fragment()) {
            None => Err(LogicErr::new(*s, "todo")),
            Some(vals) => match vals.pop() {
                None => Err(LogicErr::new(*s, "todo")),
                Some(item) => Ok(item),
            },
        }
    }
}

#[derive(Debug)]
pub struct FuncTable<'a> {
    items: HashMap<&'a str, Exp>,
}

impl<'a> FuncTable<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn get(&self, s: &Span<'a>) -> LogicRes<'a, Exp> {
        let name = s.fragment();
        match self.items.get(name) {
            None => Err(LogicErr::new(*s, "todo")),
            Some(item) => Ok(item.clone()),
        }
    }

    pub fn add(&mut self, s: &Span<'a>, exp: Exp) -> LogicRes<'a, ()> {
        let name = s.fragment();
        match self.items.insert(name, exp) {
            None => Ok(()),
            Some(_) => todo!(),
        }
    }
}

pub struct Scope<'a> {
    pub vars: NameTable<'a>,
    pub ctors: NameTable<'a>,
    pub funcs: FuncTable<'a>,
}

impl<'a> Scope<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            vars: NameTable::new(),
            ctors: NameTable::new(),
            funcs: FuncTable::new(),
        }
    }

    #[inline]
    pub fn var(&self, s: &Span<'a>) -> LogicRes<'a, Var> {
        self.vars.get(s)
    }

    #[inline]
    pub fn ctor(&self, s: &Span<'a>) -> LogicRes<'a, Ctor> {
        self.ctors.get(s)
    }

    #[inline]
    pub fn func(&self, s: &Span<'a>) -> LogicRes<'a, Exp> {
        self.funcs.get(s)
    }
}
