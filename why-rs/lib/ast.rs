use super::Token;
use super::TokenType;

#[derive(Clone, Debug)]
pub enum Operator {
    Assign,
    Add,
    Increment,
    IncrementBy,
    Subtract,
    Decrement,
    DecrementBy,
    MultBy,
    DivBy,
    Pow,
    Mult,
    Div,
    Lt,
    Gt,
    Lte,
    Gte,
    Ne,
    EqEq,
}

impl TryFrom<Token> for Operator {
    type Error = &'static str;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.typ {
            TokenType::Plus => Ok(Operator::Add),
            TokenType::PlusPlus => Ok(Operator::Increment),
            TokenType::PlusEq => Ok(Operator::IncrementBy),
            TokenType::Minus => Ok(Operator::Subtract),
            TokenType::MinusMinus => Ok(Operator::Decrement),
            TokenType::MinusEq => Ok(Operator::DecrementBy),
            TokenType::StarEq => Ok(Operator::MultBy),
            TokenType::SlashEq => Ok(Operator::DivBy),
            TokenType::StarStar => Ok(Operator::Pow),
            TokenType::Star => Ok(Operator::Mult),
            TokenType::Slash => Ok(Operator::Div),
            TokenType::Lt => Ok(Operator::Lt),
            TokenType::Lte => Ok(Operator::Lte),
            TokenType::Gt => Ok(Operator::Gt),
            TokenType::Gte => Ok(Operator::Gte),
            TokenType::Ne => Ok(Operator::Ne),
            TokenType::EqEq => Ok(Operator::EqEq),
            TokenType::Eq => Ok(Operator::Assign),
            _ => Err("Can only convert operators"),
        }
    }
}

// pub struct EvalResult {
//     inner: T,
// }

// impl EvalResult {
//     pub fn new<T>(inner: T) -> Self {
//         Self { inner }
//     }
// }

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Operator, Box<Expr>, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Int(i64),
    Float(f64),
}

// #[derive(Clone, Debug)]
// pub enum Stmt {
//     Complex(Box<Stmt>, Box<Stmt>),
//     Simple(Expr),
// }

impl Expr {
    /// Evaluate the expression.
    ///
    /// # Panics
    /// - If an unknown type of expression was encountered.
    pub fn eval(&mut self) -> i64 {
        match self {
            Expr::Int(num) => *num,
            Expr::Unary(_, expr) => -expr.eval(),
            Expr::Binary(Operator::Mult, expr1, expr2) => expr1.eval() * expr2.eval(),
            Expr::Binary(Operator::Add, expr1, expr2) => expr1.eval() + expr2.eval(),
            Expr::Binary(Operator::Div, expr1, expr2) => expr1.eval() / expr2.eval(),
            Expr::Binary(Operator::Subtract, expr1, expr2) => expr1.eval() - expr2.eval(),
            _ => panic!("Unknown expression evaluation {:?}", self),
        }
    }
}
