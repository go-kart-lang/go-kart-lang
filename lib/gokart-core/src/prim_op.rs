#[derive(Clone, Copy, Debug)]
pub enum PrimOp {
    IntPlus,
    IntMul,
    IntMinus,
    IntDiv,
    IntLe,
    IntEq,
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
            "==" => Ok(IntEq),
            _ => Err(format!("Unknown PrimOp kind {}", value)), // todo
        }
    }
}
