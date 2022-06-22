#[derive(Clone, Debug)]
pub enum TokenType {
    Eof,
    Ident,
    Eq,
    LParen,
    RParen,
    Hash,
    Dollar,
    Colon,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Caret,
    Slash,
    Backslash,
    Semi,
    At,
    And,
    SmallRArrow,
    LargeRArrow,
    Lt,
    Gt,
    Lte,
    Gte, // TODO: Add support for +=, *=, ++, --, etc...
    Ne,
    EqEq,
    LBrace,
    RBrace,
    Exclamation,
    LBracket,
    RBracket,
    Bar,
    QuestionMark,
    Percent,
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub typ: TokenType,
    pub value: String,
    pub loc: Loc,
    pub addtl: Option<Vec<String>>,
}

impl Loc {
    pub fn new() -> Self {
        Self { line: 1, col: 1 }
    }

    pub fn at(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl Token {
    pub fn new(typ: TokenType) -> Self {
        Self {
            typ,
            value: "".to_string(),
            addtl: None,
            loc: Loc::new(),
        }
    }

    pub fn at(typ: TokenType, line: usize, col: usize) -> Self {
        Self {
            typ,
            value: "".to_string(),
            addtl: None,
            loc: Loc::at(line, col),
        }
    }

    pub fn with_value(typ: TokenType, value: String) -> Self {
        Self {
            typ,
            value,
            addtl: None,
            loc: Loc::new(),
        }
    }

    pub fn with_value_at(typ: TokenType, value: String, line: usize, col: usize) -> Self {
        Self {
            typ,
            value,
            addtl: None,
            loc: Loc::at(line, col),
        }
    }
}
