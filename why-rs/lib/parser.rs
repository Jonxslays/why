use std::{iter::Peekable, slice::Iter};

use super::Expr;
use super::Operator;
use super::Token;
use super::TokenType;
use super::WhyExc;

type ExprResult = Result<Expr, WhyExc>;

#[derive(Debug)]
pub struct Parser<'a> {
    pub tokens: &'a mut Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(tokens: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        Self { tokens }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    pub fn peek(&mut self) -> Option<&&Token> {
        self.tokens.peek()
    }

    /// Expects a particular token type next.
    ///
    /// # Returns
    /// - [`()`] - Unit type on success
    ///
    /// # Errors
    /// - If an unexpected token was next.
    ///
    /// # Panics
    /// - If the next token could not be unwrapped.
    pub fn expect(&mut self, typ: TokenType) -> Result<(), WhyExc> {
        let next = self.next();

        if next.is_none() {
            return super::exc!("Unexpected end of input.");
        }

        if (*next.unwrap()).typ != typ {
            return super::exc!("Expected {:?}, but got {}", typ, next.unwrap());
        }

        Ok(())
    }

    /// Parses a stream of tokens into an expression representing the
    /// entire program.
    ///
    /// # Returns
    /// - [`ExprResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered, or no EOF token
    /// was found.
    pub fn parse(&mut self) -> ExprResult {
        println!();
        println!("Entering parse loop...");
        println!();

        let ast = self.parse_expr()?;
        self.expect(TokenType::Eof)?;
        Ok(ast)
    }

    /// Parses a terminal ast node
    ///
    /// # Returns
    /// - [`ExprResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token, or an unexpected token was
    /// received.
    pub fn parse_terminal(&mut self) -> ExprResult {
        println!("Parsing terminal");
        let next = self.next().unwrap();

        match next.typ {
            TokenType::NumLiteral(false) => Ok(Expr::Int((*next).value.parse::<i64>().unwrap())),
            TokenType::NumLiteral(true) => Ok(Expr::Float((*next).value.parse::<f64>().unwrap())),
            TokenType::RParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenType::LParen)?;
                Ok(expr)
            }
            TokenType::Minus => {
                let expr = self.parse_factor()?;
                Ok(Expr::Unary(Operator::Subtract, Box::new(expr)))
            }
            _ => super::exc!("Unexpected token: {}", next),
        }
    }

    /// Parses a factor
    ///
    /// # Returns
    /// - [`ExprResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_factor(&mut self) -> ExprResult {
        println!("Parsing factor");
        let expr = self.parse_terminal()?;
        let next = self.peek().unwrap();

        if next.typ == TokenType::StarStar {
            self.next();
            let right = self.parse_factor()?;

            return Ok(Expr::Binary(Operator::Pow, Box::new(expr), Box::new(right)));
        }

        Ok(expr)
    }

    /// Parses a term
    ///
    /// # Returns
    /// - [`ExprResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_term(&mut self) -> ExprResult {
        println!("Parsing term");
        let mut expr = self.parse_factor()?;

        loop {
            let next = self.peek().unwrap();

            match next.typ {
                TokenType::Star => {
                    self.next();
                    let right = self.parse_factor()?;

                    expr = Expr::Binary(Operator::Mult, Box::new(expr), Box::new(right));
                }
                TokenType::Slash => {
                    self.next();
                    let right = self.parse_factor()?;

                    expr = Expr::Binary(Operator::Div, Box::new(expr), Box::new(right));
                }
                _ => break, // Unknown
            }
        }

        Ok(expr)
    }

    /// Parses an expression
    ///
    /// # Returns
    /// - [`ExprResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_expr(&mut self) -> ExprResult {
        println!("Parsing expression");
        let mut expr = self.parse_term()?;

        loop {
            let next = self.peek().unwrap();

            match next.typ {
                TokenType::Plus => {
                    self.next();
                    let right = self.parse_term()?;
                    expr = Expr::Binary(Operator::Add, Box::new(expr), Box::new(right));
                }
                TokenType::Minus => {
                    self.next();
                    let right = self.parse_term()?;
                    expr = Expr::Binary(Operator::Subtract, Box::new(expr), Box::new(right));
                }
                _ => break, // Unknown
            }

            self.expect(TokenType::Semi)?;
        }

        Ok(expr)
    }
}
