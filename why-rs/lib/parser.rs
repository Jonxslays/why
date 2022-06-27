use std::{iter::Peekable, slice::Iter};

use crate::Keyword;

// use super::Condition;
use super::Expr;
// use super::Keyword;
use super::Operator;
// use super::Stmt;
use super::Token;
use super::TokenType;
// use super::VarType;

type ParseResult = Result<Expr, String>;
// type StmtRes = Result<Stmt, String>;

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
    pub fn expect(&mut self, typ: TokenType) -> Result<(), String> {
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
    /// - [`ParseResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered, or no EOF token
    /// was found.
    pub fn parse(&mut self) -> ParseResult {
        println!();
        println!("Entering parse loop...");
        println!();

        let ast = self.parse_expr()?;
        self.expect(TokenType::Eof)?;
        Ok(Expr::Main(Box::new(ast)))
    }

    // pub fn parse_assignment(&mut self) -> ParseResult {
    //     let next = self.peek().unwrap();
    //     let ident = next.value.clone();

    //     self.next();
    //     self.expect(TokenType::Eq)?;

    //     let expr = self.parse_expr()?;
    //     let assignment = Expr::Assign(Box::new(Expr::Ident(ident)), Box::new(expr));

    //     Ok(assignment)
    // }

    /// Parses a terminal ast node
    ///
    /// # Returns
    /// - [`ParseResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token, or an unexpected token was
    /// received.
    pub fn parse_primary(&mut self) -> ParseResult {
        println!("Parsing primary");
        let next = self.next().unwrap();

        match next.typ {
            TokenType::NumLiteral(false) => Ok(Expr::Int((*next).value.parse::<i64>().unwrap())),
            TokenType::NumLiteral(true) => Ok(Expr::Float((*next).value.parse::<f64>().unwrap())),
            TokenType::StrLiteral => Ok(Expr::String((*next).value.clone())),
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenType::RParen)?;
                Ok(Expr::Parenthesized(Box::new(expr)))
            }
            TokenType::Ident => Ok(Expr::Ident((*next).value.clone())),
            TokenType::Minus => {
                let expr = self.parse_factor()?;
                Ok(Expr::UnaryOp(Operator::Subtract, Box::new(expr)))
            }
            _ => super::exc!("Unexpected token: {}", next),
        }
    }

    /// Parses a factor
    ///
    /// # Returns
    /// - [`ParseResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_factor(&mut self) -> ParseResult {
        println!("Parsing factor");
        let expr = self.parse_primary()?;
        let next = self.peek().unwrap();

        if next.typ == TokenType::StarStar {
            self.next();
            let right = self.parse_factor()?;

            return Ok(Expr::BinaryOp(
                Operator::Pow,
                Box::new(expr),
                Box::new(right),
            ));
        }

        Ok(expr)
    }

    /// Parses a term
    ///
    /// # Returns
    /// - [`ParseResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_term(&mut self) -> ParseResult {
        println!("Parsing term");
        let mut expr = self.parse_factor()?;

        loop {
            let next = self.peek().unwrap();
            let op = Operator::try_from(*next);

            if let Ok(operand) = op {
                self.next();
                let right = self.parse_factor()?;

                expr = Expr::BinaryOp(operand, Box::new(expr), Box::new(right));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    // pub fn parse_keyword(&mut self, expr: &mut Option<Expr>) -> ParseResult {
    //     todo!()
    // }

    /// Parses an expression
    ///
    /// # Returns
    /// - [`ParseResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_expr(&mut self) -> ParseResult {
        println!("Parsing expression");
        let mut expr: Option<Expr> = None;
        // let current = self.next().unwrap().clone();

        loop {
            let next = self.peek().unwrap();
            println!("NEXT: {:?}", next);

            match next.typ {
                TokenType::Keyword => {
                    let keyword = Keyword::try_from(*next)?;
                    match keyword {
                        Keyword::Let => {
                            println!("Got a let");
                            break;
                        }
                        _ => return super::exc!("{} is not implemented yet", keyword),
                    }
                }
                TokenType::Plus => {
                    expr = Some(self.parse_term()?);
                    self.next();

                    let right = self.parse_term()?;
                    expr = Some(Expr::BinaryOp(
                        Operator::Add,
                        Box::new(expr.unwrap()),
                        Box::new(right),
                    ));
                }
                TokenType::Minus => {
                    expr = Some(self.parse_term()?);
                    self.next();

                    let right = self.parse_term()?;
                    expr = Some(Expr::BinaryOp(
                        Operator::Subtract,
                        Box::new(expr.unwrap()),
                        Box::new(right),
                    ));
                }
                _ => {
                    println!("Skipping unknown token {:?}", next);
                    break;
                } // Unknown
            }
        }

        Ok(expr.unwrap_or(Expr::Null))
    }
}
