use crate::state::State;
use gokart_core::{Tag, Tpl, Var, VarName};
use rpds::HashTrieMap as PHashMap;

#[derive(Debug, Clone)]
struct NameTable<'a> {
    items: PHashMap<VarName<'a>, usize>,
}

impl<'a> NameTable<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            items: PHashMap::new(),
        }
    }

    #[inline]
    pub fn get(&self, name: VarName<'a>) -> usize {
        // because we check everything on verify step,
        // so now we are confident that all names are defined
        match self.items.get(name) {
            Some(v) => *v,
            None => panic!("Unable to find {}", name),
        }
    }

    #[inline]
    pub fn add(&self, name: VarName<'a>, idx: usize) -> Self {
        Self {
            items: self.items.insert(name, idx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ctx<'a> {
    vars: NameTable<'a>,
    tags: NameTable<'a>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            vars: NameTable::new(),
            tags: NameTable::new(),
        }
    }

    #[inline]
    pub fn var(&self, name: VarName<'a>) -> Var {
        self.vars.get(name)
    }

    #[inline]
    pub fn add_var(&self, name: VarName<'a>, st: &mut State) -> Self {
        Self {
            vars: self.vars.add(name, st.next_var()),
            tags: self.tags.clone(),
        }
    }

    pub fn add_tpl(&self, tpl: &Tpl<'a>, st: &mut State) -> Self {
        match tpl {
            Tpl::Empty(_) => self.clone(),
            Tpl::Var(name) => self.add_var(name, st),
            Tpl::Pair(tpl) => {
                let x = self.add_tpl(&tpl.left, st);
                x.add_tpl(&tpl.right, st)
            }
            Tpl::As(tpl) => {
                let x = self.add_var(&tpl.name, st);
                x.add_tpl(&tpl.tpl, st)
            }
        }
    }

    #[inline]
    pub fn tag(&self, name: VarName<'a>) -> Tag {
        self.tags.get(name)
    }

    #[inline]
    pub fn add_tag(&self, name: VarName<'a>, st: &mut State) -> Self {
        Self {
            vars: self.vars.clone(),
            tags: self.tags.add(name, st.next_tag()),
        }
    }
}
