use crate::value::Primitive;

enum Token {
    LParen, RParen,
    LCurly, RCurly,
    Match,
    Lambda,
    Identifier(String),
    Literal(Primitive),
}

struct Lexeme {
    token: Token,
    line: usize,
}

pub struct Lexer {
    buffer: String,
    cursor: usize,
}

impl Lexer {
    pub fn lex(src: String) -> Lexer {
        unimplemented!()
    }
}
