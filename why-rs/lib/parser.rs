use std::{iter::Peekable, slice::Iter};

// use super::Condition;
use super::Expr;
// use super::Keyword;
use super::Operator;
use super::Stmt;
use super::Token;
use super::TokenType;
// use super::VarType;
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
            TokenType::LParen => Ok(Expr::Parenthesized(Box::new(self.parse_expr()?))),
            TokenType::Minus => {
                let expr = self.parse_factor()?;
                Ok(Expr::UnaryOp(Operator::Subtract, Box::new(expr)))
            }
            TokenType::Let => {
                let ident = self.parse_expr()?;
                self.expect(TokenType::Eq)?;
                let expr = self.parse_expr()?;
                Ok(Expr::Assign(Box::new(ident), Box::new(expr)))
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
        let mut expr = self.parse_term()?;
        println!("Result: {:?}", expr);

        loop {
            let next = self.peek().unwrap();

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
                TokenType::RParen => {
                    self.next();
                }
                TokenType::LBrace => {
                    self.next();

                    while self.peek().is_some() {
                        if self.peek().unwrap().typ == TokenType::RBrace {
                            break;
                        }

                        let expr2 = self.parse_expr()?;
                        self.expect(TokenType::Semi)?;
                        expr = Expr::Complex(Box::new(expr), Box::new(Stmt::Simple(expr2)));
                    }

                    // self.expect(TokenType::RBrace)?;
                    expr = Expr::Braced(Box::new(expr));
                }
                _ => {
                    println!("Skipping unknown token {:?}", next);
                    break;
                } // Unknown
            }
        }

        Ok(expr)
    }

    /// # Errors
    /// # Panics
    /// - if a token is unwrapped but there are no more left
    pub fn parse_stmt(&mut self) -> StmtRes {
        // println!("Parsing statement");
        let mut expr = self.parse_expr()?;

        // println!("expr pre stmt loop {:?}", expr);
        println!("expr pre stmt loop {:?}", expr);

        loop {

            let next = self.peek().unwrap();
            println!("Stmt next token inside loop {:?}", next);

            match next.typ {
                TokenType::For => {
                    self.next();
                    let tmp_var = self.parse_expr()?;
                    self.expect(TokenType::In)?;
                    let iterable = self.parse_expr()?;
                    // self.expect(TokenType::LBrace)?;
                    // let body = self.parse_stmt()?;

                    // expr = Expr::Complex(Box::new(expr), Box::new(loop_qualifier));
                    // println!("{:?}", expr);
                    println!("Temp {:?}", tmp_var);
                    println!("Iter {:?}", iterable);
                    // let braced_expr = self.parse_expr()?;
                    // expr = Expr::Complex(Box::new(expr), Box::new(Stmt::ForEach(tmp_var, iterable, braced_expr)));
                    // println!("Body {:?}", expr);
                }
                TokenType::Semi => {
                    self.next();
                }
                TokenType::Eof => break,
                _ => panic!(
                    "Syntax Error: Unknown token: {:?} at {}",
                    next.typ, next.loc
                ),
            }
        }

        Ok(Stmt::Main(expr))
    }
}
