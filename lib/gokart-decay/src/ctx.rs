use crate::{
    err::{DecayErr, DecayRes},
    state::State,
};
use gokart_core::{Loc, LocExt, Name, Tag, Tpl, Var, VarName};
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
    pub fn get(&self, name: &Name<'a>) -> DecayRes<'a, Var> {
        match self.items.get(name.val) {
            Some(var) => Ok(*var),
            None => Err(DecayErr::UnknownName(name.loc.into_span(), name.val.into())),
        }
    }

    #[inline]
    pub fn add(&self, name: &Name<'a>, idx: usize) -> Self {
        Self {
            items: self.items.insert(name.val, idx),
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
    pub fn init_with<I>(st: &mut State, items: I) -> Self
    where
        I: IntoIterator<Item = VarName<'a>>,
    {
        items.into_iter().fold(Self::new(), |acc, x| {
            // we use fake Loc because we can guarantee
            // that such name will always exist
            acc.push_var(&Name::new(x, Loc::new("")), st)
        })
    }

    #[inline]
    pub fn var(&self, name: &Name<'a>) -> DecayRes<'a, Var> {
        self.vars.get(name)
    }

    #[inline]
    pub fn push_var(&self, name: &Name<'a>, st: &mut State) -> Self {
        Self {
            vars: self.vars.add(name, st.next_var()),
            tags: self.tags.clone(),
        }
    }

    pub fn push_tpl(&self, tpl: &Tpl<'a>, st: &mut State) -> Self {
        match tpl {
            Tpl::Empty(_) => self.clone(),
            Tpl::Var(name) => self.push_var(name, st),
            Tpl::Pair(tpl) => {
                let x = self.push_tpl(&tpl.left, st);
                x.push_tpl(&tpl.right, st)
            }
            Tpl::As(tpl) => {
                let x = self.push_var(&tpl.name, st);
                x.push_tpl(&tpl.tpl, st)
            }
        }
    }

    #[inline]
    pub fn tag(&self, name: &Name<'a>) -> DecayRes<'a, Tag> {
        self.tags.get(name)
    }
}

// #[derive(Debug)]
// pub struct Names<'a> {
//     items: HashMap<&'a str, Span<'a>>,
// }

// impl<'a> Names<'a> {
//     #[inline]
//     pub fn new() -> Self {
//         Self {
//             items: HashMap::new(),
//         }
//     }

//     #[inline]
//     pub fn from(params: &Vec<Name<'a>>) -> LogicRes<'a, Self> {
//         params.iter().fold(Ok(Self::new()), |acc, param| {
//             let mut names = acc?;
//             let _ = names.add(&param.span);
//             Ok(names)
//         })
//     }

//     #[inline]
//     fn add(&mut self, item: &Span<'a>) -> LogicRes<'a, ()> {
//         match self.items.insert(item.fragment(), *item) {
//             None => Ok(()),
//             Some(_) => Err(LogicErr::new(*item, "todo")),
//         }
//     }

//     pub fn make(mut self, tpl: &Tpl<'a>) -> LogicRes<'a, Self> {
//         match tpl.deref() {
//             TplNode::Var(name) => {
//                 self.add(&name.span)?;
//                 Ok(self)
//             }
//             TplNode::Empty => Ok(self),
//             TplNode::Seq(tpls) => tpls.iter().fold(Ok(self), |acc, tpl| acc?.make(tpl)),
//             TplNode::As(var, tpl) => {
//                 self.add(&var.span)?;
//                 self.make(tpl)
//             }
//         }
//     }

//     #[inline]
//     pub fn iter<'b>(&'b self) -> hash_map::Values<'b, &'a str, Span<'a>> {
//         self.items.values()
//     }

//     #[inline]
//     pub fn with_scope<F, T>(&self, sc: &mut Scope<'a>, f: F) -> LogicRes<'a, T>
//     where
//         F: Fn(&mut Scope<'a>) -> LogicRes<'a, T>,
//     {
//         sc.push_vars(self)?;
//         let res = f(sc);
//         sc.pop_vars(self)?;
//         res
//     }
// }
