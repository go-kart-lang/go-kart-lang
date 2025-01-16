use crate::{BinOp, Exp, Pat, Type, TypeIdx, UnOp};
use derive_new::new;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Predef {}

#[derive(Debug, new)]
pub struct PredefFunc {
    pub name: &'static str,
    pub exp: Exp,
    pub ty: Type,
}

#[derive(Debug, new)]
pub struct PredefOpr {
    pub name: &'static str,
    pub bin_op: BinOp,
    pub left_ty: Type,
    pub right_ty: Type,
    pub res_ty: Type,
}

impl Predef {
    fn unit() -> &'static str {
        "Unit"
    }

    fn int() -> &'static str {
        "Int"
    }

    fn double() -> &'static str {
        "Double"
    }

    fn str() -> &'static str {
        "Str"
    }

    fn vi() -> &'static str {
        "VectorInt"
    }

    pub fn types<'a>(ty_cnt: &mut Counter) -> HashMap<&'a str, TypeIdx> {
        HashMap::from_iter([
            (Predef::unit(), ty_cnt.step()),
            (Predef::int(), ty_cnt.step()),
            (Predef::double(), ty_cnt.step()),
            (Predef::str(), ty_cnt.step()),
            (Predef::vi(), ty_cnt.step()),
        ])
    }

    pub fn funcs(tys: &HashMap<&str, TypeIdx>) -> Vec<PredefFunc> {
        use BinOp::*;
        use UnOp::*;

        let get_prim = |name| Type::Prim(*tys.get(name).unwrap());
        let unit_ty = get_prim(Predef::unit());
        let int_ty = get_prim(Predef::int());
        let double_ty = get_prim(Predef::double());
        let str_ty = get_prim(Predef::str());
        let vi_ty = get_prim(Predef::vi());

        Vec::from_iter([
            un_func("print", Print, &str_ty, &unit_ty),
            un_func("read", Read, &unit_ty, &str_ty),
            un_func("i2s", Int2Str, &int_ty, &str_ty),
            un_func("s2i", Str2Int, &str_ty, &int_ty),
            un_func("d2s", Double2Str, &double_ty, &str_ty),
            un_func("s2d", Str2Double, &str_ty, &double_ty),
            un_func("d2i", Double2Int, &double_ty, &int_ty),
            un_func("i2d", Int2Double, &int_ty, &double_ty),
            un_func("viLen", VectorIntLength, &vi_ty, &int_ty),
            un_func("viFillRandom", VectorIntFillRandom, &int_ty, &vi_ty),
            bin_func("viFill", VectorIntFill, &int_ty, &int_ty, &vi_ty),
            bin_func("viGet", VectorIntGet, &vi_ty, &int_ty, &int_ty),
            tern_func(
                "viUpdate",
                VectorIntUpdate,
                &vi_ty,
                &int_ty,
                &int_ty,
                &vi_ty,
            ),
            tern_func(
                "viUpdateMut",
                VectorIntUpdateMut,
                &vi_ty,
                &int_ty,
                &int_ty,
                &unit_ty,
            ),
        ])
    }

    pub fn oprs(tys: &HashMap<&str, TypeIdx>) -> Vec<PredefOpr> {
        use BinOp::*;

        let get_prim = |name| Type::Prim(*tys.get(name).unwrap());
        let int_ty = get_prim(Predef::int());
        let double_ty = get_prim(Predef::double());
        let str_ty = get_prim(Predef::str());

        Vec::from_iter([
            opr("+", IntPlus, &int_ty, &int_ty, &int_ty),
            opr("*", IntMul, &int_ty, &int_ty, &int_ty),
            opr("-", IntMinus, &int_ty, &int_ty, &int_ty),
            opr("/", IntDiv, &int_ty, &int_ty, &int_ty),
            opr("<", IntLt, &int_ty, &int_ty, &int_ty),
            opr("<=", IntLe, &int_ty, &int_ty, &int_ty),
            opr("==", IntEq, &int_ty, &int_ty, &int_ty),
            opr("!=", IntNe, &int_ty, &int_ty, &int_ty),
            opr(">", IntGt, &int_ty, &int_ty, &int_ty),
            opr(">=", IntGe, &int_ty, &int_ty, &int_ty),
            opr("+%", DoublePlus, &double_ty, &double_ty, &double_ty),
            opr("*%", DoubleMul, &double_ty, &double_ty, &double_ty),
            opr("-%", DoubleMinus, &double_ty, &double_ty, &double_ty),
            opr("/%", DoubleDiv, &double_ty, &double_ty, &double_ty),
            opr("<%", DoubleLt, &double_ty, &double_ty, &double_ty),
            opr("<=%", DoubleLe, &double_ty, &double_ty, &double_ty),
            opr("==%", DoubleEq, &double_ty, &double_ty, &double_ty),
            opr("!=%", DoubleNe, &double_ty, &double_ty, &double_ty),
            opr(">%", DoubleGt, &double_ty, &double_ty, &double_ty),
            opr(">=%", DoubleGe, &double_ty, &double_ty, &double_ty),
            opr("++", StrPlus, &str_ty, &str_ty, &str_ty),
            opr("=&=", StrEq, &str_ty, &str_ty, &int_ty),
            opr("!&=", StrNe, &str_ty, &str_ty, &int_ty),
        ])
    }
}

#[inline]
fn un_func(name: &'static str, un_op: UnOp, ty1: &Type, ty2: &Type) -> PredefFunc {
    PredefFunc::new(name, un_op.as_exp(), Type::func(ty1.clone(), ty2.clone()))
}

#[inline]
fn bin_func(name: &'static str, bin_op: BinOp, ty1: &Type, ty2: &Type, ty3: &Type) -> PredefFunc {
    PredefFunc::new(
        name,
        bin_op.as_exp(),
        Type::func(ty1.clone(), Type::func(ty2.clone(), ty3.clone())),
    )
}

#[inline]
fn tern_func(
    name: &'static str,
    bin_op: BinOp,
    ty1: &Type,
    ty2: &Type,
    ty3: &Type,
    ty4: &Type,
) -> PredefFunc {
    PredefFunc::new(
        name,
        bin_op.as_exp(),
        Type::func(
            ty1.clone(),
            Type::func(Type::pair(ty2.clone(), ty3.clone()), ty4.clone()),
        ),
    )
}

#[inline]
fn opr(
    name: &'static str,
    bin_op: BinOp,
    left_ty: &Type,
    right_ty: &Type,
    res_ty: &Type,
) -> PredefOpr {
    PredefOpr::new(
        name,
        bin_op,
        left_ty.clone(),
        right_ty.clone(),
        res_ty.clone(),
    )
}

trait AsExp {
    fn as_exp(&self) -> Exp;
}

impl AsExp for UnOp {
    #[inline]
    fn as_exp(&self) -> Exp {
        Exp::Abs(Pat::Var(0), Exp::Sys1(*self, Exp::Var(0).ptr()).ptr())
    }
}

impl AsExp for BinOp {
    #[inline]
    fn as_exp(&self) -> Exp {
        Exp::Abs(
            Pat::Var(0),
            Exp::Abs(
                Pat::Var(1),
                Exp::Sys2(*self, Exp::Var(0).ptr(), Exp::Var(1).ptr()).ptr(),
            )
            .ptr(),
        )
    }
}

#[derive(Debug, Default)]
pub struct Counter {
    val: usize,
}

impl Counter {
    #[inline]
    pub fn step(&mut self) -> usize {
        self.val += 1;
        self.val
    }
}
