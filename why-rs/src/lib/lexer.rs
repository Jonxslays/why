use super::Token;
// use super::TokenType;

#[derive(Clone, Debug)]
pub struct Lexer {
    pub src: String,
    pub col: usize,
    pub line: usize,
    pub error: Option<String>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src,
            col: 1,
            line: 1,
            error: None,
            tokens: vec![],
        }
    }

    pub fn lex(&mut self) {

    }

    pub fn peek(&self, offset: isize) -> char {
        self.src
            .chars()
            .nth(((self.col - 1) as isize + offset) as usize)
            .unwrap()
            .clone()
    }
}
