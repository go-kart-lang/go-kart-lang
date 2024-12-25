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

impl<'a, T> Parse<'a> for Vec<T>
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

        Ok(items)
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

macro_rules! parse {
    // sum type
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

    // product type
    ( $type:ident = $( : $before:ident : )* $( $field:ty $( : $tok:ident : )* )+ ) => {
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

impl<'a> Parse<'a> for Ast<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let defs = Vec::parse(ts)?;
        // TODO: check eof

        Ok(Ast::new(defs))
    }
}

parse!(Def = TypeDef | FuncDef | InfixDef);

impl<'a> Parse<'a> for TypeDef<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        // ts.expect(Token::Data)?;
        // let name = Udent::parse(ts)?;
        // ts.expect(Token::Assign)?;
        todo!()
    }
}

parse!(FuncDef = :Let: Ident Vec::<Ident> :Assign: ExprPtr);

parse!(InfixDef = InfixKind OprName InfixPriority);

parse!(Con = Udent Vec::<Udent>);

impl<'a> Parse<'a> for Opr<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        todo!()
    }
}

impl<'a> Parse<'a> for App<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        todo!()
    }
}

parse!(Cond = :If: ExprPtr :Then: ExprPtr :Else: ExprPtr);

// TODO: make non empty
parse!(Abs = :Backslash: Vec::<Ident> :Arrow: ExprPtr);

// TODO: maybe non empty?
parse!(Case = :Case: ExprPtr :Of: Vec::<Branch>);

parse!(Branch = :Pipe: PatPtr :Arrow: ExprPtr :Semicolon:);

impl<'a> Parse<'a> for Let<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        let kind = LetKind::parse(ts)?;
        todo!()
    }
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(ts: &mut TokenStream<'a>) -> ParseResult<Self> {
        todo!()
    }
}

parse!(As = Ident AtPatPtr);

parse!(AtPat = As | Var | Lit | PatPtr);

parse!(PatCon = Udent Vec::<AtPatPtr>);

parse!(Pat = AtPat | PatCon);

// todo:
// fn recover usize -> (line, col)
// parse
