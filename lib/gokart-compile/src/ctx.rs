use gokart_core::{Exp, GOpCode, Label, OpCode, Pat};
use rpds::List as PList;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

pub type LabelIdx = usize;

#[derive(Debug, Clone, Copy)]
pub enum DLabel {
    Label(Label),
    Defer(LabelIdx),
}

impl DLabel {
    #[inline]
    pub fn transform(self, labels: &[Label]) -> Label {
        match self {
            DLabel::Label(label) => label,
            DLabel::Defer(idx) => labels[idx],
        }
    }
}

pub type VOpCode = GOpCode<DLabel>;

trait Transform {
    fn transform(self, labels: &[Label]) -> OpCode;
}

impl Transform for VOpCode {
    #[inline]
    fn transform(self, labels: &[Label]) -> OpCode {
        match self {
            GOpCode::Acc(n) => OpCode::Acc(n),
            GOpCode::Rest(n) => OpCode::Rest(n),
            GOpCode::Push => OpCode::Push,
            GOpCode::Swap => OpCode::Swap,
            GOpCode::Sys0(op) => OpCode::Sys0(op),
            GOpCode::Sys1(op) => OpCode::Sys1(op),
            GOpCode::Sys2(op) => OpCode::Sys2(op),
            GOpCode::Cur(dl) => OpCode::Cur(dl.transform(labels)),
            GOpCode::Return => OpCode::Return,
            GOpCode::Clear => OpCode::Clear,
            GOpCode::Cons => OpCode::Cons,
            GOpCode::App => OpCode::App,
            GOpCode::Pack(tag) => OpCode::Pack(tag),
            GOpCode::Skip => OpCode::Skip,
            GOpCode::Stop => OpCode::Stop,
            GOpCode::Call(dl) => OpCode::Call(dl.transform(labels)),
            GOpCode::GotoFalse(dl) => OpCode::GotoFalse(dl.transform(labels)),
            GOpCode::Switch(tag, dl) => OpCode::Switch(tag, dl.transform(labels)),
            GOpCode::Goto(dl) => OpCode::Goto(dl.transform(labels)),
        }
    }
}

#[derive(Debug)]
pub struct Code {
    items: VecDeque<VOpCode>,
}

impl Code {
    #[inline]
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    #[inline]
    pub fn push_dummy(&mut self) -> usize {
        let res = self.len();
        self.items.push_back(VOpCode::Stop);
        res
    }

    #[inline]
    pub fn push_dummies(&mut self, n: usize) -> usize {
        let res = self.len();
        for _ in 0..n {
            self.items.push_back(VOpCode::Stop);
        }
        res
    }

    #[inline]
    pub fn cur_label(&self) -> DLabel {
        DLabel::Label(self.len())
    }

    #[inline]
    pub fn transform(self, labels: &[Label]) -> Vec<OpCode> {
        self.items
            .into_iter()
            .map(|r| r.transform(labels))
            .collect()
    }
}

impl Deref for Code {
    type Target = VecDeque<VOpCode>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Code {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[derive(Debug)]
pub enum EnvUnit<'a> {
    Con(&'a Pat),
    Lab(&'a Pat, DLabel),
}

pub type Env<'a> = PList<EnvUnit<'a>>;

#[derive(Debug)]
pub struct Ctx<'a> {
    pub code: Code,
    pub queue: VecDeque<(&'a Exp, Env<'a>)>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            code: Code::new(),
            queue: VecDeque::new(),
        }
    }
}
