#[derive(Clone, Debug)]
pub enum NodeType {
    Entrypoint,
    FunctionDecl,
    FunctionParamDecl,
    VarDecl,
    BinaryExpr,
    BooleanExpr,
    CallExpr,
    IdentExpr,
    NumberExpr,
    StringExpr,
    CompoundStmt,
    IfStmt,
    PrintStmt,
    ReturnStmt,
    WhiteStmt,
    Error,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    MinusEq,
    PlusEq,
    StarEq,
    SlashEq,
    StarStar,
    Star,
    Slash,
}

#[derive(Clone, Debug)]
pub enum AstNode {
    Int(isize),
    Uint(usize),
    Root {
        children: Vec<AstNode>,
    },
    UnaryExpr {
        op: Operator,
        child: Box<AstNode>,
    },
    BinaryExpr {
        op: Operator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}
