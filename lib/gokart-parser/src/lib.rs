mod error;
mod lex;
mod loc;
mod parse;
mod token;
mod ts;

use error::ParseResult;
use gokart_core::ast::Ast;
use lex::Lex;
use parse::Parse;

pub fn parse<'a>(input: &'a str) -> ParseResult<Ast<'a>> {
    let lex = Lex::new(input);

    Ast::parse(&mut lex.peekable())
}

// TODO: temp
// fn get_line_col<'a>(input: &'a str, p: &Pos) -> (usize, usize) {
//     let mut line = 1;
//     let mut col = 1;

//     for (idx, ch) in input.char_indices() {
//         if idx == p.idx {
//             break;
//         }
//         col += 1;
//         if ch == '\n' {
//             line += 1;
//             col = 1;
//         }
//     }

//     (line, col)
// }

// pub fn err_info<'a>(input: &'a str, err: &ParseErr) {
//     match err {
//         ParseErr::LexError(lex_err) => todo!(),
//         ParseErr::UnexpectedEnd => todo!(),
//         ParseErr::UnexpectedToken(_, pos) => {
//             println!("{err}");
//             let (line, col) = get_line_col(input, pos);
//             println!("at {line}:{col}");
//         }
//         ParseErr::InvalidInfix(pos) => todo!(),
//     }
// }
