use std::{iter::Peekable, slice::Iter};

use crate::Loc;

use super::Condition;
use super::Expr;
use super::Operator;
use super::VarType;
use super::Stmt;
use super::Token;
use super::TokenType;
use super::WhyExc;

type ExprRes = Result<Expr, WhyExc>;
type StmtRes = Result<Stmt, WhyExc>;

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
    pub fn parse(&mut self) -> StmtRes {
        println!();
        println!("Entering parse loop...");
        println!();

        let ast = self.parse_stmt()?;
        self.expect(TokenType::Eof)?;
        Ok(ast)
    }

    pub fn assert_assignment_var_type(&self, loc: &Loc, var_type: &VarType, val: &Expr) -> Result<(), WhyExc> {
        match var_type {
            VarType::Int => {
                match val {
                    Expr::Int(_) => Ok(()),
                    _ => super::exc!("{} Expected integer type, but got {:?}", loc, val),
                }
            }
            VarType::Float => {
                match val {
                    Expr::Float(_) => Ok(()),
                    _ => super::exc!("{} Expected float type, but got {:?}", loc, val),
                }
            }
            VarType::String => {
                match val {
                    Expr::String(_) => Ok(()),
                    _ => super::exc!("{} Expected string type, but got {:?}", loc, val),
                }
            }
            // VarType::Array(_) => {
            //     match val {
            //         Expr::Float(_) => Ok(val),
            //         _ => super::exc!("Expected array type"),
            //     }
            // }
            // VarType::Mapping(_, _) => {
            //     match val {
            //         Expr::Float(_) => Ok(val),
            //         _ => super::exc!("Expected array type"),
            //     }
            // }
            _ => super::exc!("Unknown variable type"),
        }
    }


    pub fn parse_assignment(&mut self, var_type: VarType) -> ExprRes {
        // println!("Parsing assignment!");
        let next = self.next().unwrap();
        let loc = next.loc.clone();

        let var = Expr::Ident(next.value.clone());
        self.expect(TokenType::Eq)?;

        let val = self.parse_expr()?;
        self.expect(TokenType::Semi)?;

        self.assert_assignment_var_type(&loc, &var_type, &val)?;
        let expr = Expr::Assign(var_type, Box::new(var), Box::new(val));

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
        // println!("Parsing terminal");
        let next = self.next().unwrap();

        match next.typ {
            TokenType::NumLiteral(false) => {
                Ok(Expr::Int((*next).value.parse::<i64>().unwrap()))
            }
            TokenType::NumLiteral(true) => {
                Ok(Expr::Float((*next).value.parse::<f64>().unwrap()))
            }
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            TokenType::RBrace => {
                let expr = self.parse_stmt()?;
                // self.expect(TokenType::RBrace)?;
                Ok(Expr::Complex(Box::new(expr)))
            }
            TokenType::LBrace => {
                self.next();
                let expr = self.parse_stmt()?;
                // self.expect(TokenType::RBrace)?;
                Ok(Expr::Complex(Box::new(expr)))
            }
            TokenType::Minus => {
                let expr = self.parse_factor()?;
                Ok(Expr::Unary(Operator::Subtract, Box::new(expr)))
            }
            TokenType::Ident => {
                // Variable assignment
                let var_type = VarType::try_from(next);

                if var_type.is_err() {
                    // Not variable assignment
                    return Ok(Expr::Ident(next.value.to_string()));
                }

                let expr = self.parse_assignment(var_type.unwrap())?;
                Ok(expr)
            }
            TokenType::StrLiteral => {
                Ok(Expr::String((*next).value.clone()))
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
        // println!("Parsing factor");
        let expr = self.parse_terminal()?;
        let next = self.peek().unwrap();

        if next.typ == TokenType::StarStar {
            self.next();
            let right = self.parse_factor()?;

            return Ok(Expr::Binary(Operator::Pow, Box::new(expr), Box::new(right)));
        }

        Ok(expr)
    }

    pub fn parse_loop_qualifier(&mut self) -> ExprRes {
        let token = self.next().unwrap();

        println!("In parse loop: {:?}", token);

        match token.typ {
            TokenType::SmallRArrow => {
                // This is inclusive if or key access only
                let next = self.next().unwrap();
                println!("Sending the second ident back");
                Ok(Expr::Ident(next.value.clone()))
            }
            TokenType::LargeRArrow => {
                // This is inclusive range iterator or
                // mapping key value pair iterator
                todo!()
            }
            _ => panic!("Unexpected loop variable")
        }
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
        // println!("Parsing term");
        let mut expr = self.parse_factor()?;

        loop {
            let next = self.peek().unwrap();
            let op = Operator::try_from(*next);

            if op.is_err() {
                println!("Parse term error - skipping");
                break;
            } else {
                self.next();
                let right = self.parse_factor()?;

                expr = Expr::Binary(op.unwrap(), Box::new(expr), Box::new(right));
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
        // println!("Parsing expression");
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
                TokenType::At => {
                    // This is is a loop
                    println!("in parse expr {:?}", expr);
                    self.next();
                    let ident = self.parse_loop_qualifier()?;
                    println!("After parse expr {:?}", expr);
                    expr = Expr::LoopQualifier(Box::new(expr), Box::new(ident));
                }
                TokenType::RBrace => {
                    println!("got rbrace In parse expr for {:?}", expr);
                }
                _ => {
                    // println!("Skipping unknown token {:?}", next);
                    break;
                }, // Unknown
            }
        }

        Ok(expr)
    }

    pub fn parse_stmt(&mut self) -> StmtRes {
        // println!("Parsing statement");
        let expr = self.parse_expr()?;
        let mut stmt = Stmt::Simple(expr);

        loop {
            let next = self.peek().unwrap();
            // println!("{}", next);

            match next.typ {
                TokenType::Semi => {
                    self.next();
                    let right = self.parse_expr()?;
                    stmt = Stmt::Complex(Box::new(stmt), Box::new(Stmt::Simple(right)));
                }
                TokenType::Ident => {
                    let right = self.parse_expr()?;
                    // let peek = self.peek().unwrap();

                    // if peek.typ == TokenType::LBrace {
                    //     // This is a complex statement after a loop
                    //     // or function call or something
                    //     self.next();
                    //     let body = self.parse_stmt()?;
                    //     println!("BODY: {:?}", body);
                    //     break;
                    // }

                    stmt = Stmt::Complex(Box::new(stmt), Box::new(Stmt::Simple(right)));
                }
                TokenType::LBrace => {
                    // self.next();
                    let right = self.parse_expr()?;
                    stmt = Stmt::Complex(Box::new(stmt), Box::new(Stmt::Simple(right)));
                    println!(" after lbrace {:?}", stmt);
                }
                TokenType::RBrace => {
                    println!("We can catch it here");
                }
                TokenType::Eof => break,
                _ => panic!("Syntax error on {}: {:?}", next.loc, next.value),
            }
        }

        Ok(stmt)
    }
}
