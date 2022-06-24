use std::{iter::Peekable, slice::Iter};

// use super::Condition;
use super::Expr;
use super::Operator;
// use super::Stmt;
use super::Keyword;
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
    pub fn parse_terminal(&mut self) -> ExprRes {
        println!("Parsing terminal");
        let next = self.next().unwrap();

        match next.typ {
            TokenType::NumLiteral(false) => Ok(Expr::Int((*next).value.parse::<i64>().unwrap())),
            TokenType::NumLiteral(true) => Ok(Expr::Float((*next).value.parse::<f64>().unwrap())),
            TokenType::StrLiteral => Ok(Expr::String((*next).value.clone())),
            TokenType::Ident => Ok(Expr::Ident((*next).value.clone())),
            TokenType::If => Ok(Expr::Keyword(Keyword::If)),
            TokenType::For => Ok(Expr::Keyword(Keyword::For)),
            TokenType::In => Ok(Expr::Keyword(Keyword::In)),
            TokenType::Is => Ok(Expr::Keyword(Keyword::Is)),
            TokenType::Break => Ok(Expr::Keyword(Keyword::Break)),
            TokenType::Return => Ok(Expr::Keyword(Keyword::Return)),
            TokenType::Let => Ok(Expr::Keyword(Keyword::Let)),
            // TokenType::LParen => {
            //     let expr = self.parse_expr()?;
            //     self.expect(TokenType::RParen)?;
            //     Ok(expr)
            // }
            TokenType::Minus => {
                let expr = self.parse_factor()?;
                Ok(Expr::UnaryOp(Operator::Subtract, Box::new(expr)))
            }
            TokenType::Eq => {
                self.next();
                let expr = self.parse_expr()?;
                Ok(Expr::Stmt(Box::new(expr)))
            }
            TokenType::Eof => Ok(Expr::Null),
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

            if op.is_err() {
                break;
            } else {
                self.next();
                let right = self.parse_factor()?;

                expr = Expr::BinaryOp(op.unwrap(), Box::new(expr), Box::new(right));
            }
        }

        Ok(expr)
    }

    pub fn parse_var_decl(&mut self) -> ExprRes {
        println!("Parsing assign");
        let mut expr = self.parse_term()?;
        println!("Assign exp: {:?}", expr);

        if let Expr::Keyword(Keyword::Let) = expr {
            let token = self.next().unwrap();
            let token_value = token.value.clone();

            match token.typ {
                TokenType::Ident => {
                    self.expect(TokenType::Eq)?;
                    let right = self.parse_expr()?;
                    println!("Before culprit var decl");
                    // self.expect(TokenType::Semi)?;
                    println!("After culprit");
                    // We overwrite the let binding with the an
                    // assignment expression
                    expr = Expr::Assign(Box::new(Expr::Ident(token_value)), Box::new(right));
                }
                _ => panic!("Unexpected token after let binding: {:?}", token),
            }
        } else if let Expr::Ident(ident) = expr {
            self.expect(TokenType::Eq)?;
            let right = self.parse_expr()?;
            expr = Expr::Reassign(Box::new(Expr::Ident(ident)), Box::new(right));
        } else if let Expr::BinaryOp(Operator::Dot, _, _) = expr {
            self.expect(TokenType::LParen)?;

            let maybe_next = self.peek();
            if let Some(next) = maybe_next {
                match next.typ {
                    TokenType::RParen => {
                        self.next();
                        self.expect(TokenType::Semi)?;
                        expr = Expr::Call(Box::new(expr), Box::new(Expr::Null));
                    }
                    _ => {
                        self.next();
                        let right = self.parse_expr()?;
                        expr = Expr::Call(Box::new(expr), Box::new(right));
                    }
                }
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
    pub fn parse_expr(&mut self) -> ExprRes {
        println!("Parsing expression");
        let mut expr = self.parse_var_decl()?;
        // let mut expr = self.parse_term()?;
        println!("Result: {:?}", expr);

        // match expr {
        //     Expr::Null => return Ok(expr),
        //     _ => (),
        // }

        loop {
            let next = self.peek().unwrap();

            match next.typ {
                TokenType::Ident => {
                    let right = self.parse_expr()?;
                    expr = Expr::Complex(Box::new(expr), Box::new(right));
                    println!("before expr culprit");
                    // self.expect(TokenType::Semi)?;
                    println!("after expr culprit");
                    println!("{:?}", expr);
                    println!("Completed ident reassign expr");
                }
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
                TokenType::Eq => {
                    self.next();
                    let right = self.parse_term()?;
                    expr = Expr::Stmt(Box::new(right));
                }
                // TokenType::LParen => {
                //     println!("Handling lparen in expr");
                //     self.next();
                //     let right = self.parse_expr()?;
                //     expr = Expr::Stmt(Box::new(right));
                // }
                TokenType::Semi => {
                    println!("handling semi in parse expr");
                    self.next();
                    let right = self.parse_expr()?;
                    expr = Expr::Complex(Box::new(expr), Box::new(right));
                    // break;
                }
                TokenType::Let => {
                    let right = self.parse_expr()?;
                    self.expect(TokenType::Semi)?;
                    expr = Expr::Complex(Box::new(expr), Box::new(right));
                }
                _ => {
                    println!("Skipping unknown token {:?}", next);
                    break;
                } // Unknown
            }
        }

        Ok(expr)
    }

    // /// # Errors
    // /// - if a token is unwrapped but there are no more left
    // pub fn parse_stmt(&mut self) -> StmtRes {
    //     // println!("Parsing statement");
    //     let expr = self.parse_expr()?;
    //     let mut stmt = Stmt::Simple(expr);

    //     loop {
    //         let next = self.peek().unwrap();
    //         // println!("{}", next);

    //         match next.typ {
    //             TokenType::Semi => {
    //                 self.next();
    //                 let right = self.parse_expr()?;
    //                 stmt = Stmt::Complex(Box::new(stmt), Box::new(Stmt::Simple(right)));
    //             }
    //             TokenType::Ident => {
    //                 let right = self.parse_expr()?;
    //                 stmt = Stmt::Complex(Box::new(stmt), Box::new(Stmt::Simple(right)));
    //             }
    //             TokenType::Eof => break,
    //             _ => panic!("Syntax error on {}: {:?}", next.loc, next.value),
    //         }
    //     }

    //     Ok(stmt)
    // }
}
