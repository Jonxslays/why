use super::Node;
use super::Token;
// use super::TokenType;

#[derive(Clone, Debug)]
pub struct Parser {
    pub root: Node,
    pub tokens: Vec<Token>,
    pub idx: usize,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            root: Node::Root { children: vec![] },
            tokens,
            idx: 0,
        }
    }

    pub fn parse(&mut self) {
        println!("Entering parse loop...");

        while self.idx < self.tokens.len() {
            // let current = &self.tokens[self.idx];

            // match current.typ {
            //     TokenType::NumLiteral => {}
            //     TokenType::Ident => {}
            //     _ => (),
            // }
        }
    }
}
