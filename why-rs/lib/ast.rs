#[allow(dead_code)]
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
pub enum Node {
    Int(isize),
    Uint(usize),
    Root {
        children: Vec<Node>,
    },
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}
