use super::Token;
use super::TokenType;
use super::WhyExc;

#[derive(Clone, Debug)]
pub struct Lexer {
    pub src: Vec<char>,
    pub idx: usize,
    pub c: char,
    pub tokens: Vec<Token>,
    pub errors: Vec<WhyExc>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        let src: Vec<char> = src.chars().collect();
        let c = src
            .first()
            .unwrap_or_else(|| {
                super::exc!("No text in the file!!!");
            })
            .clone();

        Self {
            c,
            src,
            idx: 0,
            tokens: vec![],
            errors: vec![],
        }
    }

    fn expect(&self, typ: TokenType, c: char) -> Option<Token> {
        if self.peek(1).unwrap_or(char::default()) == c {
            Some(Token::new(typ))
        } else {
            None
        }
    }

    fn lex_eq(&self) -> Token {
        if let Some(mut token) = self.expect(TokenType::LargeRArrow, '>') {
            token.value = "=>".to_string();
            token
        } else if let Some(mut token) = self.expect(TokenType::EqEq, '=') {
            token.value = "==".to_string();
            token
        } else {
            let token = Token::with_value(TokenType::Eq, "=".to_string());
            token
        }
    }

    fn parse_eq(idx: &mut usize, tokens: &mut Vec<Token>, token: Token) {
        if let TokenType::Eq = token.typ {
            return Lexer::advance_with(idx, tokens, token, 1);
        }

        Lexer::advance_with(idx, tokens, token, 2);
    }

    fn advance(idx: &mut usize, i: usize) {
        *idx += i;
    }

    fn advance_with(idx: &mut usize, tokens: &mut Vec<Token>, token: Token, i: usize) {
        tokens.push(token);
        *idx += i;
    }

    fn protect_index_error(&self) -> bool {
        !(self.idx >= self.src.len())
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let _in_string = false;

        loop {
            if !self.protect_index_error() {
                break;
            }

            let character = self.src[self.idx];
            match character {
                '=' => {
                    let token = self.lex_eq();
                    Lexer::parse_eq(&mut self.idx, &mut self.tokens, token);
                }
                _ => Lexer::advance(&mut self.idx, 1),
            }
        }

        self.tokens.push(Token::new(TokenType::Eof));
        self.tokens.clone()
    }

    pub fn peek(&self, offset: isize) -> Option<char> {
        if !self.protect_index_error() {
            return None;
        }

        Some(self.src[(self.idx as isize + offset) as usize])
    }
}
