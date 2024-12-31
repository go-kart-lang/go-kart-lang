use crate::{
    ast::*,
    err::{ParseErr, ParseRes},
    lex::token,
    token::{Span, Token, TokenKind},
};
use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{eof, map},
    multi::{many0, many1, separated_list0},
    sequence::tuple,
};

macro_rules! failure {
    ( $span:expr, $msg:expr $(, $var:expr )* ) => {
        Err(::nom::Err::Failure(crate::err::ParseErr::new(
            $span, format!($msg, $( $var ),*)
        )))
    };
}

macro_rules! error {
    ( $span:expr, $msg:expr $(, $var:expr )* ) => {
        Err(::nom::Err::Error(crate::err::ParseErr::new(
            $span, format!($msg, $( $var ),*)
        )))
    };
}

fn expect(kind: TokenKind) -> impl Fn(Span) -> ParseRes<Token> {
    move |i: Span| match token(i)? {
        (s, tok) if tok.kind == kind => Ok((s, tok)),
        (s, tok) => error!(s, "Expect {} but got {}", kind.as_ref(), tok.kind.as_ref()),
    }
}

fn ident(i: Span) -> ParseRes<Span> {
    map(expect(TokenKind::Ident), |tok| tok.span)(i)
}

fn udent(i: Span) -> ParseRes<Span> {
    map(expect(TokenKind::Udent), |tok| tok.span)(i)
}

fn con(i: Span) -> ParseRes<(Span, Vec<Span>)> {
    let res = tuple((udent, many0(udent)));

    map(res, |(name, params)| (name, params))(i)
}

fn opr(i: Span) -> ParseRes<Term> {
    let res = tuple((app_term, opr_name, infix_term));

    map(res, |(left, name, right)| {
        TermNode::Opr(left, name, right).ptr()
    })(i)
}

fn let_kind(i: Span) -> ParseRes<LetKind> {
    match token(i)? {
        (s, tok) if tok.kind == TokenKind::Let => Ok((s, LetKind::NonRec)),
        (s, tok) if tok.kind == TokenKind::Letrec => Ok((s, LetKind::Rec)),
        (s, tok) => error!(s, "Expect LetKind but got {}", tok.kind.as_ref()),
    }
}

fn let_func_def(i: Span) -> ParseRes<FuncDef> {
    let res = tuple((
        ident,
        many0(ident),
        expect(TokenKind::Assign),
        term,
        expect(TokenKind::Semicolon),
    ));

    map(res, |(name, params, _, body, _)| {
        FuncDef::new(name, params, body)
    })(i)
}

fn let_term(i: Span) -> ParseRes<Term> {
    let res = tuple((let_kind, many1(let_func_def), expect(TokenKind::In), term));
    map(res, |(kind, funcs, _, body)| {
        TermNode::Let(kind, funcs, body).ptr()
    })(i)
}

fn wrap_term(i: Span) -> ParseRes<Term> {
    let res = tuple((expect(TokenKind::LParen), term, expect(TokenKind::RParen)));

    map(res, |(_, term, _)| term)(i)
}

fn lit(i: Span) -> ParseRes<Lit> {
    match token(i)? {
        (s, tok) if tok.kind == TokenKind::Int => match tok.span.fragment().parse::<i64>() {
            Ok(val) => Ok((s, Lit::Int(val))),
            Err(e) => failure!(s, "Bad int literal: {}", e),
        },
        (s, tok) if tok.kind == TokenKind::Double => todo!(),
        (s, tok) if tok.kind == TokenKind::Str => Ok((s, Lit::Str(tok.span.fragment()))),
        (s, tok) => error!(s, "Expect Literal but got {}", tok.kind.as_ref()),
    }
}

fn var(i: Span) -> ParseRes<Span> {
    match token(i)? {
        (s, tok) if tok.kind == TokenKind::Ident => Ok((s, tok.span)),
        (s, tok) if tok.kind == TokenKind::Udent => Ok((s, tok.span)),
        (s, tok) => error!(s, "Expect Var but got {}", tok.kind.as_ref()),
    }
}

fn at_term(i: Span) -> ParseRes<Term> {
    alt((
        map(lit, |x| TermNode::Lit(x).ptr()),
        map(var, |x| TermNode::Var(x).ptr()),
        wrap_term,
    ))(i)
}

fn app_term(i: Span) -> ParseRes<Term> {
    alt((app, at_term))(i)
}

fn infix_term(i: Span) -> ParseRes<Term> {
    alt((app_term, abs, opr))(i)
}

fn cond(i: Span) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::If),
        term,
        expect(TokenKind::Then),
        term,
        expect(TokenKind::Else),
        term,
    ));
    map(res, |(_, cond, _, left, _, right)| {
        Ptr::new(TermNode::Cond(cond, left, right))
    })(i)
}

fn app(i: Span) -> ParseRes<Term> {
    let res = tuple((at_term, many1(at_term)));

    map(res, |(head, children)| {
        Ptr::new(TermNode::App(head, children))
    })(i)
}

fn abs(i: Span) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::Backslash),
        many1(ident),
        expect(TokenKind::Arrow),
        term,
    ));

    map(res, |(_, params, _, body)| {
        Ptr::new(TermNode::Abs(params, body))
    })(i)
}

fn as_tpl(i: Span) -> ParseRes<Tpl> {
    let res = tuple((ident, expect(TokenKind::As), at_tpl));

    map(res, |(name, _, tpl)| TplNode::As(name, tpl).ptr())(i)
}

fn wrap_tpl(i: Span) -> ParseRes<Tpl> {
    let res = tuple((expect(TokenKind::LParen), tpl, expect(TokenKind::RParen)));

    map(res, |(_, tpl, _)| tpl)(i)
}

fn at_tpl(i: Span) -> ParseRes<Tpl> {
    alt((
        as_tpl,
        map(ident, |x| TplNode::Var(x).ptr()),
        map(lit, |x| TplNode::Lit(x).ptr()),
        wrap_tpl,
    ))(i)
}

fn con_tpl(i: Span) -> ParseRes<Tpl> {
    // TODO: maybe many1?
    let res = tuple((udent, many0(at_tpl)));

    map(res, |(name, tpls)| TplNode::Con(name, tpls).ptr())(i)
}

fn tpl(i: Span) -> ParseRes<Tpl> {
    alt((at_tpl, con_tpl))(i)
}

fn branch(i: Span) -> ParseRes<(Tpl, Term)> {
    let res = tuple((
        expect(TokenKind::Pipe),
        tpl,
        expect(TokenKind::Arrow),
        term,
        expect(TokenKind::Semicolon),
    ));

    map(res, |(_, tpl, _, term, _)| (tpl, term))(i)
}

fn case(i: Span) -> ParseRes<Term> {
    let res = tuple((
        expect(TokenKind::Case),
        term,
        expect(TokenKind::Of),
        many1(branch),
    ));

    map(res, |(_, cond, _, branches)| {
        Ptr::new(TermNode::Case(cond, branches))
    })(i)
}

fn term(i: Span) -> ParseRes<Term> {
    alt((infix_term, cond, case, let_term))(i)
}

fn type_def(i: Span) -> ParseRes<TypeDef> {
    let res = tuple((
        expect(TokenKind::Data),
        udent,
        expect(TokenKind::Assign),
        separated_list0(expect(TokenKind::Pipe), con),
    ));
    map(res, |(_, name, _, cons)| TypeDef::new(name, cons))(i)
}

fn func_def(i: Span) -> ParseRes<FuncDef> {
    let res = tuple((
        expect(TokenKind::Let),
        ident,
        many0(ident),
        expect(TokenKind::Assign),
        term,
    ));

    map(res, |(_, name, params, _, body)| {
        FuncDef::new(name, params, body)
    })(i)
}

fn infix_kind(i: Span) -> ParseRes<InfixKind> {
    match token(i)? {
        (s, tok) if tok.kind == TokenKind::Infixl => Ok((s, InfixKind::Left)),
        (s, tok) if tok.kind == TokenKind::Infixr => Ok((s, InfixKind::Right)),
        (s, tok) => error!(s, "Expect InfixKind but got {}", tok.kind.as_ref()),
    }
}

fn opr_name(i: Span) -> ParseRes<Span> {
    match token(i)? {
        (tail, tok) if tok.kind == TokenKind::Opr => Ok((tail, tok.span)),
        _ => todo!(),
    }
}

fn infix_priority(i: Span) -> ParseRes<InfixPriority> {
    match token(i)? {
        (s, tok) if tok.kind == TokenKind::Int => match tok.span.fragment().parse::<i64>() {
            Ok(val) if val > 0 => Ok((s, InfixPriority::new(val as u64))),
            Ok(val) => failure!(s, "InfixPriority cannot be negative but got {}", val),
            Err(e) => failure!(s, "Bad int literal for InfixPriority: {}", e),
        },
        (s, tok) => error!(s, "Expect InfixPriority but got {}", tok.kind.as_ref()),
    }
}

fn infix_def(i: Span) -> ParseRes<InfixDef> {
    let res = tuple((infix_kind, opr_name, infix_priority));

    map(res, |(kind, name, priority)| {
        InfixDef::new(kind, name, priority)
    })(i)
}

fn def(i: Span) -> ParseRes<Def> {
    alt((
        map(type_def, |x| Def::TypeDef(x)),
        map(func_def, |x| Def::FuncDef(x)),
        map(infix_def, |x| Def::InfixDef(x)),
    ))(i)
}

fn ast(i: Span) -> ParseRes<Ast> {
    let res = tuple((many1(def), multispace0, eof));
    map(res, |(x, _, _)| Ast::new(x))(i)
}

pub fn parse<'a>(input: &'a str) -> Result<Ast<'a>, ParseErr> {
    let i = Span::new(input);
    let res = ast(i.clone());

    match res {
        Ok((_, ast)) => Ok(ast),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
        _ => Err(ParseErr::new(i, "Unknown error")), // todo
    }
}
