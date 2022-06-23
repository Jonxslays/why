#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    NumLiteral(bool),
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
    Null,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub value: String,
    pub loc: Loc,
    pub addtl: Option<Vec<String>>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            typ: TokenType::Null,
            value: String::new(),
            loc: Loc::default(),
            addtl: None,
        }
    }
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

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line: {}, Col: {}", self.line, self.col)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({}) @ {}", self.typ, self.value, self.loc)
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
