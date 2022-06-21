// use super::Token;
use super::WhyExc;
// use super::TokenType;

#[derive(Clone, Debug)]
pub struct Lexer {
    pub src: String,
    pub col: usize,
    pub line: usize,
    pub errors: Vec<WhyExc>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src,
            col: 1,
            line: 1,
            errors: vec![],
        }
    }

    pub fn lex(&mut self) {
        self.line = 69;
        self.col = 420;
        super::exc!(self, "uwu");
    }

    pub fn peek(&self, offset: isize) -> char {
        self.src
            .chars()
            .nth(((self.col - 1) as isize + offset) as usize)
            .unwrap()
            .clone()
    }
}
