use gokart_core::{BinOp, Double, GOpCode, Int, Label, NullOp, OpCode, Str, Tag, UnOp};
use std::{io, marker::Sized};
use thiserror::Error;

// todo: better error messages
#[derive(Debug, Error)]
pub enum SerdeErr {
    #[error("Unexpected eof")]
    UnexpectedEof,
    #[error("Unexpected OpCode")]
    UnexpectedOpCode,
    #[error("Invalid UTF-8")]
    InvalidUtf8,
}

pub type SerdeRes<T> = Result<T, SerdeErr>;

pub trait Deserialize {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized;
}

pub trait Serialize {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write;
}

impl Serialize for u64 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserialize for u64 {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)
            .map_err(|_| SerdeErr::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serialize for i64 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserialize for i64 {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)
            .map_err(|_| SerdeErr::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serialize for usize {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        (*self as u64).serialize(w);
    }
}

impl Deserialize for usize {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        u64::deserialize(r).map(|x| x as usize)
    }
}

impl Serialize for u32 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserialize for u32 {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer)
            .map_err(|_| SerdeErr::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serialize for i32 {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserialize for i32 {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let mut buffer = [0u8; 4];
        r.read_exact(&mut buffer)
            .map_err(|_| SerdeErr::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serialize for Str {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        self.len().serialize(w);
        let _ = w.write(self.as_bytes());
    }
}

impl Deserialize for Str {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let cap = usize::deserialize(r)?;
        let mut buffer = Vec::with_capacity(cap);
        r.read_exact(&mut buffer)
            .map_err(|_| SerdeErr::UnexpectedEof)?;
        Self::from_utf8(buffer).map_err(|_| SerdeErr::InvalidUtf8)
    }
}

impl Serialize for Double {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let _ = w.write(&self.to_le_bytes());
    }
}

impl Deserialize for Double {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer)
            .map_err(|_| SerdeErr::UnexpectedEof)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Serialize for NullOp {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        use NullOp::*;
        let tag: i32 = match self {
            IntLit(_) => 1,
            DoubleLit(_) => 2,
            StrLit(_) => 3,
        };
        tag.serialize(w);

        match self {
            IntLit(val) => val.serialize(w),
            DoubleLit(val) => val.serialize(w),
            StrLit(val) => val.serialize(w),
        }
    }
}

impl Deserialize for NullOp {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        use NullOp::*;
        let tag = i32::deserialize(r)?;
        match tag {
            1 => Int::deserialize(r).map(IntLit),
            2 => Double::deserialize(r).map(DoubleLit),
            3 => Str::deserialize(r).map(StrLit),
            _ => Err(SerdeErr::UnexpectedOpCode),
        }
    }
}

impl Serialize for UnOp {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        use UnOp::*;
        let tag: i32 = match self {
            Print => 1,
            Read => 2,
            Int2Str => 3,
            Str2Int => 4,
            Double2Str => 5,
            Str2Double => 6,
            Double2Int => 7,
            Int2Double => 8,
            VectorIntZeros => 9,
        };
        tag.serialize(w);
    }
}

impl Deserialize for UnOp {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        use UnOp::*;
        let tag = i32::deserialize(r)?;
        match tag {
            1 => Ok(Print),
            2 => Ok(Read),
            3 => Ok(Int2Str),
            4 => Ok(Str2Int),
            5 => Ok(Double2Str),
            6 => Ok(Str2Double),
            7 => Ok(Double2Int),
            8 => Ok(Int2Double),
            9 => Ok(VectorIntZeros),
            _ => Err(SerdeErr::UnexpectedOpCode),
        }
    }
}

impl Serialize for BinOp {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        use BinOp::*;
        let tag: i32 = match self {
            IntPlus => 1,
            IntMul => 2,
            IntMinus => 3,
            IntDiv => 4,
            IntLt => 5,
            IntLe => 6,
            IntEq => 7,
            IntNe => 8,
            IntGt => 9,
            IntGe => 10,
            DoublePlus => 11,
            DoubleMul => 12,
            DoubleMinus => 13,
            DoubleDiv => 14,
            DoubleLt => 15,
            DoubleLe => 16,
            DoubleEq => 17,
            DoubleNe => 18,
            DoubleGt => 19,
            DoubleGe => 20,
            StrPlus => 21,
            StrEq => 22,
            StrNe => 23,
            VectorIntGet => 24,
            VectorIntUpdate => 25,
        };
        tag.serialize(w);
    }
}

impl Deserialize for BinOp {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        use BinOp::*;
        let tag = i32::deserialize(r)?;
        match tag {
            1 => Ok(IntPlus),
            2 => Ok(IntMul),
            3 => Ok(IntMinus),
            4 => Ok(IntDiv),
            5 => Ok(IntLt),
            6 => Ok(IntLe),
            7 => Ok(IntEq),
            8 => Ok(IntNe),
            9 => Ok(IntGt),
            10 => Ok(IntGe),
            11 => Ok(DoublePlus),
            12 => Ok(DoubleMul),
            13 => Ok(DoubleMinus),
            14 => Ok(DoubleDiv),
            15 => Ok(DoubleLt),
            16 => Ok(DoubleLe),
            17 => Ok(DoubleEq),
            18 => Ok(DoubleNe),
            19 => Ok(DoubleGt),
            20 => Ok(DoubleGe),
            21 => Ok(StrPlus),
            22 => Ok(StrEq),
            23 => Ok(StrNe),
            24 => Ok(VectorIntGet),
            25 => Ok(VectorIntUpdate),
            _ => Err(SerdeErr::UnexpectedOpCode),
        }
    }
}

impl Serialize for OpCode {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        use GOpCode::*;
        let tag: i32 = match self {
            Acc(_) => 1,
            Rest(_) => 2,
            Push => 3,
            Swap => 4,
            Sys0(_) => 5,
            Sys1(_) => 6,
            Sys2(_) => 7,
            Cur(_) => 8,
            Return => 9,
            Clear => 10,
            Cons => 11,
            App => 12,
            Pack(_) => 13,
            Skip => 14,
            Stop => 15,
            Call(_) => 16,
            GotoFalse(_) => 17,
            Switch(_, _) => 18,
            Goto(_) => 19,
        };
        tag.serialize(w);

        match self {
            Acc(n) => n.serialize(w),
            Rest(n) => n.serialize(w),
            Push => (),
            Swap => (),
            Sys0(op) => op.serialize(w),
            Sys1(op) => op.serialize(w),
            Sys2(op) => op.serialize(w),
            Cur(l) => l.serialize(w),
            Return => (),
            Clear => (),
            Cons => (),
            App => (),
            Pack(t) => t.serialize(w),
            Skip => (),
            Stop => (),
            Call(l) => l.serialize(w),
            GotoFalse(l) => l.serialize(w),
            Switch(t, l) => {
                t.serialize(w);
                l.serialize(w);
            }
            Goto(l) => l.serialize(w),
        };
    }
}

impl Deserialize for crate::OpCode {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        use GOpCode::*;
        let tag = i32::deserialize(r)?;
        match tag {
            1 => u32::deserialize(r).map(Acc),
            2 => u32::deserialize(r).map(Rest),
            3 => Ok(Push),
            4 => Ok(Swap),
            5 => NullOp::deserialize(r).map(Sys0),
            6 => UnOp::deserialize(r).map(Sys1),
            7 => BinOp::deserialize(r).map(Sys2),
            8 => Label::deserialize(r).map(Cur),
            9 => Ok(Return),
            10 => Ok(Clear),
            11 => Ok(Cons),
            12 => Ok(App),
            13 => Label::deserialize(r).map(Pack),
            14 => Ok(Skip),
            15 => Ok(Stop),
            16 => Label::deserialize(r).map(Call),
            17 => Label::deserialize(r).map(GotoFalse),
            18 => {
                let t = Tag::deserialize(r)?;
                let l = Label::deserialize(r)?;
                Ok(Switch(t, l))
            }
            19 => Label::deserialize(r).map(Goto),
            _ => Err(SerdeErr::UnexpectedOpCode),
        }
    }
}

impl Serialize for Vec<OpCode> {
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        for x in self.iter() {
            x.serialize(w);
        }
    }
}

impl Deserialize for Vec<OpCode> {
    fn deserialize<R>(r: &mut R) -> SerdeRes<Self>
    where
        R: io::Read,
        Self: Sized,
    {
        let mut code = Vec::new();

        loop {
            match OpCode::deserialize(r) {
                Ok(x) => code.push(x),
                Err(SerdeErr::UnexpectedEof) => return Ok(code),
                Err(e) => return Err(e),
            }
        }
    }
}
