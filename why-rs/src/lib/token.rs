#[derive(Clone, Debug)]
pub enum TokenType {
    Eof,
    Ident,
    Str,
    Eq,
    LParen,
    RParen,
    Hash,
    Dollar,
    Colon,
    Comma,
    Slash,
    Backslash,
    Semi,
    At,
    SmallRArrow,
    LargeRArrow,
    Lt,
    Gt,
    Lte,
    Gte,
    Ne,
    EqEq,
    Keyword,
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
pub struct Token {
    pub typ: TokenType,
    pub value: String,
    pub line: usize,
    pub col: usize,
    pub addtl: Option<Vec<String>>,
}

impl Token {
    pub fn new(typ: TokenType) -> Self {
        Self {
            typ,
            value: "".to_string(),
            addtl: None,
            line: 0,
            col: 0,
        }
    }

    pub fn at(typ: TokenType, line: usize, col: usize) -> Self {
        Self {
            typ,
            value: "".to_string(),
            addtl: None,
            line,
            col,
        }
    }

    pub fn with_value(typ: TokenType, value: String) -> Self {
        Self {
            typ,
            value,
            addtl: None,
            line: 0,
            col: 0,
        }
    }

    pub fn with_value_at(typ: TokenType, value: String, line: usize, col: usize) -> Self {
        Self {
            typ,
            value,
            addtl: None,
            line,
            col,
        }
    }
}
