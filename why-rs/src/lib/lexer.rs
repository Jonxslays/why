use super::Token;
use super::WhyExc;
use super::TokenType;

#[derive(Clone, Debug)]
pub struct Lexer {
    pub src: Vec<char>,
    pub col: usize,
    pub line: usize,
    pub idx: usize,
    pub errors: Vec<WhyExc>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src: src.chars().collect(),
            col: 1,
            line: 1,
            idx: 0,
            errors: vec![],
        }
    }

    fn parse_ident(&self, c: char) -> (Token, usize) {
        let mut buffer = String::new();
        let mut i: usize = 1;

        loop {
            let next = self.peek(i as isize);
            if !next.is_alphanumeric() || next == '\0' {
                break;
            }

            buffer.push(c);
            i += 1;
        }

        (Token::with_value(TokenType::Ident, buffer), i)
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let _in_string = false;
        let mut tokens = vec![];

        // TODO: Rework this loop now that src is a vector
        // and we are keeping track of the index
        for character in &self.src {
            if ['\r', '\n'].contains(&character) {
                continue;
            }

            if character.is_whitespace() {
                self.col += 1;
                continue;
            }

            if character.is_alphabetic() {
                let (token, i) = self.parse_ident(*character);
                tokens.push(token);
                self.col += i;
                continue;
            }

            // println!("Unhandled character: {}", character);
        }

        tokens
    }

    pub fn peek(&self, offset: isize) -> char {
        self.src[(self.col - 1) + offset as usize]
    }
}
