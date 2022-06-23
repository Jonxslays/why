use std::collections::HashMap;
use super::Token;

#[derive(Clone, Debug)]
pub enum NodeType {
    Entrypoint,
    FunctionDecl,
    FunctionParamDecl,
    VarDecl,
    UnaryExpr,
    BinaryExpr,
    BooleanExpr,
    CallExpr,
    IdentExpr,
    IntExpr(usize),
    FloatExpr(f64),
    StringExpr,
    CompoundStmt,
    IfStmt,
    PrintStmt,
    ReturnStmt,
    WhileStmt,
    Error,
    Null,
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
    Eq,
    EqEq,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub typ: NodeType,
    pub children: HashMap<String, Node>,
    pub op: Option<Operator>,
}

impl Default for Node {
    fn default() -> Self {
        Self { typ: NodeType::Null, children: HashMap::default(), op: None }
    }
}

impl Node {
    pub fn new(typ: NodeType) -> Self {
        Self { typ, children: HashMap::new(), op: None }
    }

    pub fn new_op(typ: NodeType, op: Operator) -> Self {
        Self { typ, children: HashMap::new(), op: Some(op) }
    }

    pub fn push_child(&mut self, key: &str, node: Node) {
        self.children.insert(key.into(), node);
    }
}
