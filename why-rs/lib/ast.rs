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
}

#[derive(Clone, Debug)]
pub enum Condition {
    Lt,
    Gt,
    Lte,
    Gte,
    Ne,
    EqEq,
}

impl TryFrom<&Token> for Condition {
    type Error = &'static str;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token.typ {
            TokenType::EqEq => Ok(Condition::EqEq),
            TokenType::Ne => Ok(Condition::Ne),
            TokenType::Gte => Ok(Condition::Gte),
            TokenType::Lte => Ok(Condition::Lte),
            TokenType::Gt => Ok(Condition::Gt),
            TokenType::Lt => Ok(Condition::Lt),
            _ => Err("Failed to convert conditional token"),
        }
    }
}

impl TryFrom<&Token> for Operator {
    type Error = &'static str;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
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
            TokenType::Eq => Ok(Operator::Assign),
            _ => Err("Failed to convert operator token"),
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

impl TryFrom<&Token> for VarType {
    type Error = &'static str;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token.value.as_str() {
            "int" => Ok(VarType::Int),
            "string" => Ok(VarType::String),
            "float" => Ok(VarType::Float),
            _ => Err("Failed to convert operator token"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum VarType {
    Int,
    Float,
    String,
    Array(Box<VarType>),
    Mapping(Box<VarType>, Box<VarType>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Operator, Box<Expr>, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Int(i64),
    Float(f64),
    String(String),
    Ident(String),
    Assign(VarType, Box<Expr>, Box<Expr>),
    LoopQualifier(Box<Expr>, Box<Expr>),
    Complex(Box<Stmt>),
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Complex(Box<Stmt>, Box<Stmt>),
    If(Condition, Box<Stmt>),
    While(Condition, Box<Stmt>),
    Simple(Expr),
    ForEach(Expr, Box<Stmt>),
}

// impl Expr {
//     /// Evaluate the expression.
//     ///
//     /// # Panics
//     /// - If an unknown type of expression was encountered.
//     pub fn eval(&mut self) -> i64 {
//         match self {
//             Expr::Int(num) => *num,
//             Expr::Unary(_, expr) => -expr.eval(),
//             Expr::Binary(Operator::Mult, expr1, expr2) => expr1.eval() * expr2.eval(),
//             Expr::Binary(Operator::Add, expr1, expr2) => expr1.eval() + expr2.eval(),
//             Expr::Binary(Operator::Div, expr1, expr2) => expr1.eval() / expr2.eval(),
//             Expr::Binary(Operator::Subtract, expr1, expr2) => expr1.eval() - expr2.eval(),
//             Expr::Assign(VarType::Int, expr1, expr2) => {
//                 expr1.eval() = expr2.eval(),
//             }
//             _ => panic!("Unknown expression evaluation {:?}", self),
//         }
//     }
// }
