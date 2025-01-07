#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimOp {
    IntPlus,
    IntMul,
    IntMinus,
    IntDiv,
    IntLe,
    // IntLeq,
    IntEq,
    // IntNeq,
    // IntGe,
    // IntGeq,
    Print,
}

impl<'a> TryFrom<&'a str> for PrimOp {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        use PrimOp::*;

        match value {
            "+" => Ok(IntPlus),
            "*" => Ok(IntMul),
            "-" => Ok(IntMinus),
            "/" => Ok(IntDiv),
            "<" => Ok(IntLe),
            // "<=" => Ok(IntLeq),
            "==" => Ok(IntEq),
            // "!=" => Ok(IntNeq),
            // ">" => Ok(IntGe),
            // ">=" => Ok(IntGeq),
            "print" => Ok(Print),
            _ => Err(format!("Unknown PrimOp kind {}", value)), // todo
        }
    }
}
