use crate::{
    error::{ParseErr, ParseResult},
    token::Token,
    ts::{TokenStream, TokenStremExt},
};
use gokart_core::ast::*;

pub trait Parse<'a>: Sized {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self>;
}

macro_rules! parse_prim {
    ( $type:ty : $( $rule:pat => $handler:expr, )+  ) => {
        impl<'a> Parse<'a> for $type {
            fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
                match ts.nextf()? {
                    $(
                        $rule => { $handler }
                    )+
                    (tok, loc) => Err(ParseErr::UnexpectedToken(tok.name(), loc.begin)),
                }
            }
        }
    };
}

// TODO: convert parse_prim! and parse! to one big macro

parse_prim!(InfixKind :
    (Token::Infixl, _) => Ok(InfixKind::Left),
    (Token::Infixr, _) => Ok(InfixKind::Right),
);

parse_prim!(LetKind :
    (Token::Let, _) => Ok(LetKind::NonRec),
    (Token::Letrec, _) => Ok(LetKind::Rec),
);

parse_prim!(Var<'a> :
    (Token::Ident(val), _) => Ok(Var::new(val)),
    (Token::Udent(val), _) => Ok(Var::new(val)),
);

parse_prim!(OprName<'a> :
    (Token::Opr(val), _) => Ok(OprName::new(val)),
);

parse_prim!(InfixPriority :
    (Token::Int(x), loc) => match x {
        _ if x < 0 => Err(ParseErr::InvalidInfix(loc.begin)),
        _ => Ok(InfixPriority::new(x as u64))
    },
);

parse_prim!(Ident<'a> :
    (Token::Ident(val), _) => Ok(Ident::new(val)),
);

parse_prim!(Udent<'a> :
    (Token::Udent(val), _) => Ok(Udent::new(val)),
);

parse_prim!(Lit<'a> :
    (Token::Int(val), _) => Ok(Lit::Int(val)),
    (Token::Double(val), _) => Ok(Lit::Double(val)),
    (Token::Str(val), _) => Ok(Lit::Str(val)),
);

macro_rules! parse {
    ( $type:ident = $( $branch:ident )|+ ) => {
        impl<'a> Parse<'a> for $type<'a> {
            fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
                let tsc = ts.clone();

                $(
                    if let Ok(x) = <$branch>::parse(ts) {
                        return Ok($type::$branch(x));
                    }
                    *ts = tsc.clone();
                )*

                let (tok, loc) = ts.nextf()?;
                Err(ParseErr::UnexpectedToken(tok.name(), loc.begin))
            }
        }
    };

    ( $type:ident = $( { $before:ident } )* $( $field:ty $( { $tok:ident } )* )+ ) => {
        impl<'a> Parse<'a> for $type<'a> {
            fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
                $(
                    ts.expect(Token::$before)?;
                )*
                Ok(Self::new(
                    $({
                        let res = <$field>::parse(ts)?;
                        $(
                            ts.expect(Token::$tok)?;
                        )*
                        res
                    }),+
                ))
            }
        }
    };
}

// TODO: include expect_eof in parse! macro (somehow)
impl<'a> Parse<'a> for Ast<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let defs = NeSeq::<Def>::parse(ts)?;
        ts.expect_eof()?;

        Ok(Ast::new(defs))
    }
}

parse!(Def = TypeDef | FuncDef | InfixDef);

parse!(TypeDef = {Data} Udent {Assign} Pipe<Con>);

parse!(FuncDef = {Let} Ident Seq::<Ident> {Assign} ExprPtr);

parse!(InfixDef = InfixKind OprName InfixPriority);

parse!(Con = Udent Seq::<Udent>);

parse!(Opr = AppExpr OprName InfixExprPtr);

parse!(App = AtExprPtr NeSeq<AtExprPtr>);

parse!(Cond = {If} ExprPtr {Then} ExprPtr {Else} ExprPtr);

parse!(Abs = {Backslash} NeSeq::<Ident> {Arrow} ExprPtr);

parse!(Case = {Case} ExprPtr {Of} NeSeq::<Branch>);

parse!(Branch = {Pipe} PatPtr {Arrow} ExprPtr {Semicolon});

parse!(LetFuncDef = Ident Seq::<Ident> {Assign} ExprPtr {Semicolon});

parse!(Let = LetKind NeSeq<LetFuncDef> {In} ExprPtr);

parse!(AtExpr = Lit | Var | WrapExpr);

parse!(WrapExpr = {LParen} ExprPtr {RParen});

parse!(AppExpr = App | AtExpr);

parse!(InfixExpr = AppExpr | Abs | Opr);

parse!(Expr = InfixExpr | Cond | Case | Let);

parse!(As = Ident {As} AtPatPtr);

parse!(AtPat = As | Ident | Lit | WrapPat);

parse!(PatCon = Udent Seq::<AtPatPtr>);

parse!(WrapPat = {LParen} PatPtr {RParen});

parse!(Pat = AtPat | PatCon);

impl<'a, T> Parse<'a> for Seq<T>
where
    T: Parse<'a>,
{
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let mut items = Vec::new();
        let mut tsc = ts.clone();

        while let Ok(item) = T::parse(ts) {
            items.push(item);
            tsc = ts.clone();
        }
        *ts = tsc;

        Ok(Seq::new(items))
    }
}

impl<'a, T> Parse<'a> for NeSeq<T>
where
    T: Parse<'a>,
{
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let first = T::parse(ts)?;
        let seq = Seq::<T>::parse(ts)?;

        let mut items = Vec::new();
        items.push(first);
        items.extend(seq.items);

        Ok(NeSeq::new(items))
    }
}

impl<'a, T> Parse<'a> for Pipe<T>
where
    T: Parse<'a>,
{
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let mut items = Vec::new();
        items.push(T::parse(ts)?);

        while let Some(Ok((Token::Pipe, _))) = ts.peek() {
            ts.expect(Token::Pipe)?;
            items.push(T::parse(ts)?);
        }

        Ok(Pipe::new(items))
    }
}

impl<'a, T> Parse<'a> for Box<T>
where
    T: Parse<'a>,
{
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let inner = T::parse(ts)?;

        Ok(Box::new(inner))
    }
}

// TODO: better error handling & messages
// create function to convert for Pos(idx) to {line}:{col}

// TODO: convert Parse into template with Iterator
