#[derive(Clone, Debug, PartialEq)]
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
    PlusPlus,
    Minus,
    MinusMinus,
    MinusEq,
    PlusEq,
    StarEq,
    SlashEq,
    StarStar,
    Star,
    Caret,
    Slash,
    Backslash,
    Semi,
    At,
    And,
    SmallRArrow,
    LargeRArrow,
    NumLiteral,
    StrLiteral,
    Lt,
    Gt,
    Lte,
    Gte,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub value: String,
    pub loc: Loc,
    pub addtl: Option<Vec<String>>,
}

impl Loc {
    #[must_use]
    pub fn new() -> Self {
        Self { line: 1, col: 1 }
    }

    #[must_use]
    pub fn at(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl Default for Loc {
    fn default() -> Self {
        Self::new()
    }
}

impl Token {
    #[must_use]
    pub fn new(typ: TokenType) -> Self {
        Self {
            typ,
            value: "".to_string(),
            addtl: None,
            loc: Loc::new(),
        }
    }

    #[must_use]
    pub fn at(typ: TokenType, line: usize, col: usize) -> Self {
        Self {
            typ,
            value: "".to_string(),
            addtl: None,
            loc: Loc::at(line, col),
        }
    }

    #[must_use]
    pub fn with_value(typ: TokenType, value: String) -> Self {
        Self {
            typ,
            value,
            addtl: None,
            loc: Loc::new(),
        }
    }

    #[must_use]
    pub fn with_value_at(typ: TokenType, value: String, line: usize, col: usize) -> Self {
        Self {
            typ,
            value,
            addtl: None,
            loc: Loc::at(line, col),
        }
    }
}
