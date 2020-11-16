mod value;
mod error;
mod ast;
mod env;
mod pattern;
mod lex;

use crate::lex::Lexer;

fn main() {
    let src = String::from("");
    let mut lexer = Lexer::lex(src);
}
