#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    UnexpectedOpCode,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnexpectedEof => write!(f, "unexpected eof"),
            Self::UnexpectedOpCode => write!(f, "unexpected opcode"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Deserializable {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized;
}

pub trait Serializable {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write;
}

impl Serializable for u64 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserializable for u64 {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)
            .map_err(|_| Error::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serializable for i64 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserializable for i64 {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)
            .map_err(|_| Error::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serializable for usize {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        (*self as u64).serialize(w);
    }
}

impl Deserializable for usize {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        u64::deserialize(r).map(|x| x as usize)
    }
}

impl Serializable for u32 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserializable for u32 {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer)
            .map_err(|_| Error::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serializable for i32 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserializable for i32 {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer)
            .map_err(|_| Error::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serializable for crate::prim_op::PrimOp {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let tag = match self {
            crate::PrimOp::IntPlus => 1,
            crate::PrimOp::IntMul => 2,
            crate::PrimOp::IntMinus => 3,
            crate::PrimOp::IntDiv => 4,
            crate::PrimOp::IntLe => 5,
            crate::PrimOp::IntEq => 6,
            crate::PrimOp::Print => 99,
            _ => 666,
        };
        tag.serialize(w);
    }
}

impl Deserializable for crate::prim_op::PrimOp {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let tag = i32::deserialize(r)?;
        match tag {
            1 => Ok(crate::PrimOp::IntPlus),
            2 => Ok(crate::PrimOp::IntMul),
            3 => Ok(crate::PrimOp::IntMinus),
            4 => Ok(crate::PrimOp::IntDiv),
            5 => Ok(crate::PrimOp::IntLe),
            6 => Ok(crate::PrimOp::IntEq),
            99 => Ok(crate::PrimOp::Print),
            _ => Err(Error::UnexpectedOpCode),
        }
    }
}

impl Serializable for crate::OpCode {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        let tag = match self {
            crate::OpCode::Acc(_) => 1,
            crate::OpCode::Rest(_) => 2,
            crate::OpCode::QuoteInt(_) => 3,
            crate::OpCode::Push => 4,
            crate::OpCode::Swap => 5,
            crate::OpCode::Prim(_) => 6,
            crate::OpCode::Cur(_) => 7,
            crate::OpCode::Return => 8,
            crate::OpCode::Clear => 9,
            crate::OpCode::Cons => 10,
            crate::OpCode::App => 11,
            crate::OpCode::Pack(_) => 12,
            crate::OpCode::Skip => 13,
            crate::OpCode::Stop => 14,
            crate::OpCode::Call(_) => 15,
            crate::OpCode::GotoFalse(_) => 16,
            crate::OpCode::Switch(_, _) => 17,
            crate::OpCode::Goto(_) => 18,
            crate::OpCode::Read => 19
        };
        tag.serialize(w);

        match self {
            crate::OpCode::Acc(l) => l.serialize(w),
            crate::OpCode::Rest(l) => l.serialize(w),
            crate::OpCode::QuoteInt(l) => l.serialize(w),
            crate::OpCode::Prim(prim_op) => prim_op.serialize(w),
            crate::OpCode::Cur(l) => l.serialize(w),
            crate::OpCode::Pack(l) => l.serialize(w),
            crate::OpCode::Call(l) => l.serialize(w),
            crate::OpCode::GotoFalse(l) => l.serialize(w),
            crate::OpCode::Switch(a, b) => {
                a.serialize(w);
                b.serialize(w);
            }
            crate::OpCode::Goto(l) => l.serialize(w),
            crate::OpCode::Push => (),
            crate::OpCode::Swap => (),
            crate::OpCode::Return => (),
            crate::OpCode::Clear => (),
            crate::OpCode::Cons => (),
            crate::OpCode::App => (),
            crate::OpCode::Skip => (),
            crate::OpCode::Stop => (),
            crate::OpCode::Read => (),
        };
    }
}

impl Deserializable for crate::OpCode {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let tag = i32::deserialize(r)?;
        match tag {
            1 => u32::deserialize(r).map(crate::OpCode::Acc),
            2 => u32::deserialize(r).map(crate::OpCode::Rest),
            3 => i64::deserialize(r).map(crate::OpCode::QuoteInt),
            4 => Ok(crate::OpCode::Push),
            5 => Ok(crate::OpCode::Swap),
            6 => crate::PrimOp::deserialize(r).map(crate::OpCode::Prim),
            7 => usize::deserialize(r).map(crate::OpCode::Cur),
            8 => Ok(crate::OpCode::Return),
            9 => Ok(crate::OpCode::Clear),
            10 => Ok(crate::OpCode::Cons),
            11 => Ok(crate::OpCode::App),
            12 => usize::deserialize(r).map(crate::OpCode::Pack),
            13 => Ok(crate::OpCode::Skip),
            14 => Ok(crate::OpCode::Stop),
            15 => usize::deserialize(r).map(crate::OpCode::Call),
            16 => usize::deserialize(r).map(crate::OpCode::GotoFalse),
            17 => {
                let a = usize::deserialize(r)?;
                let b = usize::deserialize(r)?;
                Ok(crate::OpCode::Switch(a, b))
            }
            18 => usize::deserialize(r).map(crate::OpCode::Goto),
            19 => Ok(crate::OpCode::Read),
            _ => Err(Error::UnexpectedOpCode),
        }
    }
}

impl Serializable for crate::Code {
    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        for c in &self.data {
            c.serialize(w);
        }
    }
}

impl Deserializable for crate::Code {
    fn deserialize<R>(r: &mut R) -> Result<Self>
    where
        R: std::io::Read,
        Self: std::marker::Sized,
    {
        let mut codes = vec![];

        loop {
            match crate::OpCode::deserialize(r) {
                Ok(x) => codes.push(x),
                Err(Error::UnexpectedOpCode) => return Err(Error::UnexpectedOpCode),
                Err(Error::UnexpectedEof) => return Ok(crate::Code::from(codes)),
            }
        }
    }
}
