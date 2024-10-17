type Var = String;
type Con = String;

enum Exp {
    Variable(Var),
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

enum Pat {
    Variable(Var),
    EmptyPattern,
    Pair(Box<Pat>, Box<Pat>),
    Layer(Var, Box<Pat>),
}

fn main() {}
