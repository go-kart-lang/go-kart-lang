use crate::err::{LogicErr, LogicRes};
use gokart_core::{Name, Span, Tag, Tpl, TplNode, Var};
use num_traits::NumAssign;
use std::{
    collections::{hash_map, HashMap},
    ops::Deref,
};

#[derive(Debug)]
pub struct NameTable<'a, T> {
    items: HashMap<&'a str, Vec<T>>,
    cnt: T,
}

impl<'a, T> NameTable<'a, T>
where
    T: NumAssign + Copy,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            cnt: T::zero(),
        }
    }

    pub fn get(&self, s: &Span<'a>) -> LogicRes<'a, T> {
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
        self.cnt += T::one();
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
    items: HashMap<&'a str, Span<'a>>,
}

impl<'a> Names<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    #[inline]
    pub fn from(params: &Vec<Name<'a>>) -> LogicRes<'a, Self> {
        params.iter().fold(Ok(Self::new()), |acc, param| {
            let mut names = acc?;
            let _ = names.add(&param.span);
            Ok(names)
        })
    }

    #[inline]
    fn add(&mut self, item: &Span<'a>) -> LogicRes<'a, ()> {
        match self.items.insert(item.fragment(), *item) {
            None => Ok(()),
            Some(_) => Err(LogicErr::new(*item, "todo")),
        }
    }

    pub fn make(mut self, tpl: &Tpl<'a>) -> LogicRes<'a, Self> {
        match tpl.deref() {
            TplNode::Var(name) => {
                self.add(&name.span)?;
                Ok(self)
            }
            TplNode::Empty => Ok(self),
            TplNode::Seq(tpls) => tpls.iter().fold(Ok(self), |acc, tpl| acc?.make(tpl)),
            TplNode::As(var, tpl) => {
                self.add(&var.span)?;
                self.make(tpl)
            }
        }
    }

    #[inline]
    pub fn iter<'b>(&'b self) -> hash_map::Values<'b, &'a str, Span<'a>> {
        self.items.values()
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
    vars: NameTable<'a, Var>,
    tags: NameTable<'a, Tag>,
}

impl<'a> Scope<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            vars: NameTable::new(),
            tags: NameTable::new(),
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
    pub fn tag(&self, s: &Span<'a>) -> LogicRes<'a, Tag> {
        self.tags.get(s)
    }

    #[inline]
    pub fn add_tag(&mut self, s: &Span<'a>) -> LogicRes<'a, ()> {
        self.tags.push(s)
    }
}
