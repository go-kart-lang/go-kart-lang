pub type Var = String;
pub type Con = u32;

#[derive(Debug)]
pub enum Exp {
    Variable(Var),
    ConstInt(i32),
    Sys(String, Box<Exp>, Box<Exp>),
    EmptyTuple,
    Pair(Box<Exp>, Box<Exp>),
    Constructor(Con, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    Abstraction(Pat, Box<Exp>),
    Conditional(Box<Exp>, Box<Exp>, Box<Exp>),
    Case(Box<Exp>, Vec<(Con, Pat, Box<Exp>)>),
    Local(Pat, Box<Exp>, Box<Exp>),
    LocalRec(Pat, Box<Exp>, Box<Exp>),
}

#[derive(Debug)]
pub enum Pat {
    Variable(Var),
    EmptyPattern,
    Pair(Box<Pat>, Box<Pat>),
    Layer(Var, Box<Pat>),
}

#[derive(Debug)]
pub struct TypeDef<'a> {
    type_name: &'a str,
    constructors: Vec<(&'a str, Vec<&'a str>)>,
}
