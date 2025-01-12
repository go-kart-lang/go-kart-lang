use crate::{
    err::{ParseErr, ParseRes},
    lex::token,
    token::{Token, TokenKind},
};
use gokart_core::{
    Abs, App, AsTpl, Ast, Branch, Case, Con, ConTerm, Cond, Def, DoubleLit, EmptyTerm, EmptyTpl,
    IntLit, Let, Letrec, Lit, Loc, LocExt, Name, Opr, PairTerm, PairTpl, StrLit, Term, Tpl,
    TypeDef,
};
use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{eof, map},
    multi::{many0, many1, separated_list0},
    sequence::tuple,
    IResult, InputTake, Offset, Parser,
};

fn with_loc<'a, O, E, P>(mut p: P) -> impl FnMut(Loc<'a>) -> IResult<Loc<'a>, (Loc<'a>, O), E>
where
    P: Parser<Loc<'a>, O, E>,
{
    move |i: Loc<'a>| {
        let (rem, res) = p.parse(i)?;
        let loc = i.take(i.offset(&rem));
        Ok((rem, (loc, res)))
    }
}

#[inline]
fn pair_loc<'a>(i: Loc<'a>, first: Loc<'a>, second: Loc<'a>) -> Loc<'a> {
    let (tail, _) = i.take_split(i.offset(&first));
    tail.take(tail.offset(&second) + second.len())
}

#[inline]
fn expect(kind: TokenKind) -> impl Fn(Loc) -> ParseRes<Token> {
    move |i: Loc| match token(i)? {
        (r, tok) if tok.kind == kind => Ok((r, tok)),
        (_, tok) => ParseErr::UnexpectedToken(tok.loc.into_span(), kind, tok.kind).err(),
    }
}

#[inline]
fn name(kind: TokenKind) -> impl Fn(Loc) -> ParseRes<Name> {
    move |i: Loc| map(expect(kind), |tok| Name::new(tok.loc.val(), tok.loc))(i)
}

#[inline]
fn ident(i: Loc) -> ParseRes<Name> {
    name(TokenKind::Ident)(i)
}

#[inline]
fn udent(i: Loc) -> ParseRes<Name> {
    name(TokenKind::Udent)(i)
}

#[inline]
fn opr_name(i: Loc) -> ParseRes<Name> {
    name(TokenKind::Opr)(i)
}

fn seq_term(i: Loc) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::LParen),
        separated_list0(expect(TokenKind::Comma), term),
        expect(TokenKind::RParen),
    ));

    map(with_loc(res), |(loc, (_, terms, _))| {
        let mut it = terms.into_iter();
        match it.next() {
            Some(init) => it.fold(init, |acc, x| {
                let (first, second) = (acc.loc(), x.loc());
                Term::Pair(PairTerm::new(
                    acc.ptr(),
                    x.ptr(),
                    pair_loc(i, first, second),
                ))
            }),
            None => Term::Empty(EmptyTerm::new(loc)),
        }
    })(i)
}

fn con_term(i: Loc) -> ParseRes<Term> {
    let res = tuple((udent, seq_term));

    map(with_loc(res), |(loc, (name, body))| {
        Term::Con(ConTerm::new(name, body.ptr(), loc))
    })(i)
}

fn int_lit(i: Loc) -> ParseRes<Lit> {
    let (rem, tok) = expect(TokenKind::Int)(i)?;

    match tok.loc.val().parse::<i64>() {
        Ok(x) => Ok((rem, Lit::Int(IntLit::new(x, tok.loc)))),
        Err(e) => ParseErr::BadIntLiteral(tok.loc.into_span(), e).failure(),
    }
}

fn double_lit(i: Loc) -> ParseRes<Lit> {
    let (rem, tok) = expect(TokenKind::Double)(i)?;

    match tok.loc.val().parse::<f64>() {
        Ok(x) => Ok((rem, Lit::Double(DoubleLit::new(x, tok.loc)))),
        Err(e) => ParseErr::BadDoubleLiteral(tok.loc.into_span(), e).failure(),
    }
}

fn str_lit(i: Loc) -> ParseRes<Lit> {
    let res = expect(TokenKind::Str);

    map(res, |tok| {
        let val = tok.loc.val();
        Lit::Str(StrLit::new(&val[1..val.len() - 1], tok.loc))
    })(i)
}

fn lit(i: Loc) -> ParseRes<Lit> {
    alt((int_lit, double_lit, str_lit))(i)
}

fn at_term(i: Loc) -> ParseRes<Term> {
    alt((
        map(lit, Term::Lit),
        map(ident, Term::Var),
        con_term,
        seq_term,
    ))(i)
}

fn abs(i: Loc) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::Backslash),
        many1(ident),
        expect(TokenKind::Arrow),
        term,
    ));

    map(res, |(_, params, _, body)| {
        let second = body.loc();
        params.into_iter().rfold(body, |acc, p| {
            let loc = pair_loc(i, p.loc, second);
            Term::Abs(Abs::new(p, acc.ptr(), loc))
        })
    })(i)
}

fn app(i: Loc) -> ParseRes<Term> {
    let res = tuple((at_term, many1(at_term)));

    map(res, |(head, children)| {
        children.into_iter().fold(head, |acc, x| {
            let loc = pair_loc(i, acc.loc(), x.loc());
            Term::App(App::new(acc.ptr(), x.ptr(), loc))
        })
    })(i)
}

fn app_term(i: Loc) -> ParseRes<Term> {
    alt((app, at_term))(i)
}

fn opr(i: Loc) -> ParseRes<Term> {
    let res = tuple((app_term, opr_name, infix_term));

    map(with_loc(res), |(loc, (left, name, right))| {
        Term::Opr(Opr::new(left.ptr(), name, right.ptr(), loc))
    })(i)
}

fn let_part(i: Loc) -> ParseRes<(Tpl, Term)> {
    let res = tuple((
        tpl,
        expect(TokenKind::Assign),
        term,
        expect(TokenKind::Semicolon),
    ));

    map(res, |(tpl, _, body, _)| (tpl, body))(i)
}

#[derive(Debug)]
enum LetKind {
    NonRec,
    Rec,
}

fn let_kind(i: Loc) -> ParseRes<LetKind> {
    match token(i)? {
        (r, tok) if tok.kind == TokenKind::Let => Ok((r, LetKind::NonRec)),
        (r, tok) if tok.kind == TokenKind::Letrec => Ok((r, LetKind::Rec)),
        (_, tok) => ParseErr::UnexpectedToken(tok.loc.into_span(), TokenKind::Let, tok.kind).err(),
    }
}

fn let_term(i: Loc) -> ParseRes<Term> {
    let res = tuple((let_kind, many1(let_part), expect(TokenKind::In), term));

    map(with_loc(res), |(loc, (kind, parts, _, body))| {
        let (tpls, terms): (Vec<_>, Vec<_>) = parts.into_iter().unzip();

        let tpl = tpls
            .into_iter()
            .reduce(|a, b| {
                let loc = b.loc();
                Tpl::Pair(PairTpl::new(a.ptr(), b.ptr(), loc))
            })
            .unwrap(); // because we always have at least one let_part
        let term = terms
            .into_iter()
            .reduce(|a, b| {
                let loc = b.loc();
                Term::Pair(PairTerm::new(a.ptr(), b.ptr(), loc))
            })
            .unwrap(); // because we always have at least one let_part

        match kind {
            LetKind::NonRec => Term::Let(Let::new(tpl, term.ptr(), body.ptr(), loc)),
            LetKind::Rec => Term::Letrec(Letrec::new(tpl, term.ptr(), body.ptr(), loc)),
        }
    })(i)
}

fn branch(i: Loc) -> ParseRes<Branch> {
    let res = tuple((
        expect(TokenKind::Pipe),
        udent,
        tpl,
        expect(TokenKind::Arrow),
        term,
        expect(TokenKind::Semicolon),
    ));

    map(with_loc(res), |(loc, (_, con, tpl, _, term, _))| {
        Branch::new(con, tpl, term, loc)
    })(i)
}

fn case(i: Loc) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::Case),
        term,
        expect(TokenKind::Of),
        many1(branch),
    ));

    map(with_loc(res), |(loc, (_, cond, _, branches))| {
        Term::Case(Case::new(cond.ptr(), branches, loc))
    })(i)
}

fn cond(i: Loc) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::If),
        term,
        expect(TokenKind::Then),
        term,
        expect(TokenKind::Else),
        term,
    ));

    map(with_loc(res), |(loc, (_, cond, _, left, _, right))| {
        Term::Cond(Cond::new(cond.ptr(), left.ptr(), right.ptr(), loc))
    })(i)
}

fn infix_term(i: Loc) -> ParseRes<Term> {
    alt((opr, app_term, abs))(i)
}

fn term(i: Loc) -> ParseRes<Term> {
    alt((infix_term, cond, case, let_term))(i)
}

fn param(i: Loc) -> ParseRes<Tpl> {
    map(ident, Tpl::Var)(i)
}

fn seq_tpl(i: Loc) -> ParseRes<Tpl> {
    let res = tuple((
        expect(TokenKind::LParen),
        separated_list0(expect(TokenKind::Comma), tpl),
        expect(TokenKind::RParen),
    ));

    map(with_loc(res), |(loc, (_, tpls, _))| {
        let mut it = tpls.into_iter();
        match it.next() {
            Some(init) => it.fold(init, |acc, x| {
                let (first, second) = (acc.loc(), x.loc());
                Tpl::Pair(PairTpl::new(acc.ptr(), x.ptr(), pair_loc(i, first, second)))
            }),
            None => Tpl::Empty(EmptyTpl::new(loc)),
        }
    })(i)
}

fn at_tpl(i: Loc) -> ParseRes<Tpl> {
    alt((param, seq_tpl))(i)
}

fn as_tpl(i: Loc) -> ParseRes<Tpl> {
    let res = tuple((ident, expect(TokenKind::As), at_tpl));

    map(with_loc(res), |(loc, (name, _, tpl))| {
        Tpl::As(AsTpl::new(name, tpl.ptr(), loc))
    })(i)
}

fn tpl(i: Loc) -> ParseRes<Tpl> {
    alt((as_tpl, at_tpl))(i)
}

fn con(i: Loc) -> ParseRes<Con> {
    let res = tuple((udent, many0(udent)));

    map(with_loc(res), |(loc, (name, params))| {
        Con::new(name, params, loc)
    })(i)
}

fn type_def(i: Loc) -> ParseRes<TypeDef> {
    let res = tuple((
        expect(TokenKind::Data),
        udent,
        expect(TokenKind::Assign),
        separated_list0(expect(TokenKind::Pipe), con),
    ));
    map(with_loc(res), |(loc, (_, name, _, cons))| {
        TypeDef::new(name, cons, loc)
    })(i)
}

fn def(i: Loc) -> ParseRes<Def> {
    alt((map(type_def, Def::TypeDef),))(i)
}

fn ast(i: Loc) -> ParseRes<Ast> {
    let (i, _) = multispace0(i)?;
    let res = tuple((many0(def), term));

    let (rem, ast) = map(with_loc(res), |(loc, (defs, body))| {
        Ast::new(defs, body, loc)
    })(i)?;

    let (rem, _) = tuple((multispace0, eof))(rem)?;
    Ok((rem, ast))
}

pub fn parse(s: &str) -> Result<Ast<'_>, ParseErr> {
    match ast(Loc::new(s)) {
        Ok((_, ast)) => Ok(ast),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
        _ => unreachable!("It's assumed that the entire file has already been read"),
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn ok_valid_ident() {
        let input = "valid_ident";
        let res = ident(Loc::new(input));

        match res {
            Ok((_, res)) => assert_eq!(input, res.val),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn err_invalid_ident() {
        let input = "123invalid";
        let res = ident(Loc::new(input));

        if let Ok(res) = res {
            panic!("This test should fail, but got {res:?}")
        }
    }

    #[test]
    fn ok_valid_udent() {
        let input = "UpperLetterStart";
        let res = udent(Loc::new(input));

        match res {
            Ok((_, res)) => assert_eq!(input, res.val),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn err_invalid_udent() {
        let input = "smallLetterStart";
        let res = udent(Loc::new(input));

        if let Ok(res) = res {
            panic!("This test should fail, but got {res:?}")
        }
    }

    #[test]
    fn ok_valid_intlit() {
        let input = "42";
        let res = lit(Loc::new(input));

        match res {
            Ok((_, lit)) => match lit {
                Lit::Int(lit) => assert_eq!(42, lit.val),
                lit => panic!("Unexpected literal: {lit:?}"),
            },
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_let_term_parser() {
        let input = "let x = 5; in x";
        let res = let_term(Loc::new(input));

        if let Err(e) = res {
            panic!("Unexpected error: {e:?}")
        }
    }

    #[test]
    fn ok_cond_parser() {
        let input = "if someName == 123 then 1 else 0";
        let res = cond(Loc::new(input));

        if let Err(e) = res {
            panic!("Unexpected error: {e:?}")
        }
    }

    #[test]
    fn ok_valid_ast() {
        let input = r#"
            let impl = \a b n ->
                if n == 0 then a
                else impl b (a + b) (n - 1);
            in let fib = \n -> impl 0 1 n;
            in fib 50
        "#;
        let res = parse(input);

        if let Err(e) = res {
            panic!("Unexpected error: {e:?}")
        }
    }

    #[test]
    fn err_invali_ast() {
        let input = "data Maybe = Just Int |";
        let res = parse(input);

        if let Ok(res) = res {
            panic!("This test should fail, but got {res:?}")
        }
    }
}
