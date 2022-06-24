use std::{iter::Peekable, slice::Iter};

// use super::Condition;
use super::Expr;
// use super::Keyword;
use super::Operator;
// use super::Stmt;
use super::Token;
use super::TokenType;
// use super::VarType;
use super::WhyExc;

type ExprRes = Result<Expr, WhyExc>;
// type StmtRes = Result<Stmt, WhyExc>;

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
    pub fn parse(&mut self) -> ExprRes {
        println!();
        println!("Entering parse loop...");
        println!();

        let ast = self.parse_expr()?;
        self.expect(TokenType::Eof)?;
        Ok(Expr::Main(Box::new(ast)))
    }

    pub fn parse_assignment(&mut self) -> ExprRes {
        let next = self.peek().unwrap();
        let ident = next.value.clone();

        self.next();
        self.expect(TokenType::Eq)?;

        let expr = self.parse_expr()?;
        let assignment = Expr::Assign(Box::new(Expr::Ident(ident)), Box::new(expr));

        Ok(assignment)
    }

    pub fn parse_parenthetic(&mut self) -> ExprRes {
        let mut expr = Expr::Null;

        loop {
            let next = self.peek().unwrap();

            match next.typ {
                TokenType::RParen => {
                    self.next();
                    break;
                }
                TokenType::Eof => panic!("Parentheses were never closed: {:?}", next.loc),
                _ => {
                    let right = self.parse_expr()?;
                    expr = Expr::Stmt(Box::new(expr), Box::new(right));
                }
            }
        }

        Ok(expr)
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
    pub fn parse_terminal(&mut self) -> ExprRes {
        println!("Parsing terminal");
        let next = self.next().unwrap();

        match next.typ {
            TokenType::NumLiteral(false) => Ok(Expr::Int((*next).value.parse::<i64>().unwrap())),
            TokenType::NumLiteral(true) => Ok(Expr::Float((*next).value.parse::<f64>().unwrap())),
            TokenType::StrLiteral => Ok(Expr::String((*next).value.clone())),
            TokenType::Ident => Ok(Expr::Ident((*next).value.clone())),
            TokenType::LParen => {
                let expr = self.parse_parenthetic()?;
                Ok(Expr::Parenthesized(Box::new(expr)))
            },
            TokenType::Minus => {
                let expr = self.parse_factor()?;
                Ok(Expr::UnaryOp(Operator::Subtract, Box::new(expr)))
            }
            TokenType::Let => {
                let assignment = self.parse_assignment()?;
                self.expect(TokenType::Semi)?;
                Ok(assignment)
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
    pub fn parse_factor(&mut self) -> ExprRes {
        println!("Parsing factor");
        let expr = self.parse_terminal()?;
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
    /// - [`ExprResult`] - The resulting expression on success
    ///
    /// # Errors
    /// - If a syntax, or other, error was encountered.
    ///
    /// # Panics
    /// - If there is no next token.
    pub fn parse_term(&mut self) -> ExprRes {
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

    pub fn parse_let_binding(&mut self) -> ExprRes {
        let next = self.peek().unwrap();
        let ident = next.value.clone();

        self.next();
        self.expect(TokenType::Eq)?;

        let expr = self.parse_expr()?;
        let declaration = Expr::VarDecl(Box::new(Expr::Ident(ident)), Box::new(expr));

        Ok(declaration)
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
    pub fn parse_expr(&mut self) -> ExprRes {
        println!("Parsing expression");
        // let current = self.next().unwrap().clone();
        let mut expr = self.parse_term()?;
        println!("Result: {:?}", expr);

        loop {
            let next = self.peek().unwrap();
            println!("NEXT: {:?}", next);

            match next.typ {
                TokenType::Plus => {
                    self.next();
                    let right = self.parse_term()?;
                    expr = Expr::BinaryOp(Operator::Add, Box::new(expr), Box::new(right));
                }
                TokenType::Minus => {
                    self.next();
                    let right = self.parse_term()?;
                    expr = Expr::BinaryOp(Operator::Subtract, Box::new(expr), Box::new(right));
                }
                TokenType::Let => {
                    self.next();
                    let right = self.parse_let_binding()?;
                    expr = Expr::Stmt(Box::new(expr), Box::new(right));
                    self.expect(TokenType::Semi)?;
                }
                TokenType::Ident => {
                    let right = self.parse_assignment()?;
                    expr = Expr::Stmt(Box::new(expr), Box::new(right));
                    self.expect(TokenType::Semi)?;
                }
                TokenType::RParen => {
                    self.next();

                }
                TokenType::Semi => {break},
                _ => {
                    println!("Skipping unknown token {:?}", next);
                    break;
                } // Unknown
            }
        }

        Ok(expr)
    }
}
