#[derive(Clone, Debug)]
pub enum TokenType {
    Eof,
    Ident,
    Str,
    Int,
    Uint,
    Float,
    Ufloat,
    Eq,
    Lparen,
    Rpaen,
    Hash,
    Dollar,
    Colon,
    Comma,
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
    pub value: Option<String>,
    pub addtl: Option<Vec<String>>,
}

impl Token {
    pub fn new(typ: TokenType) -> Self {
        Self { typ, value: None, addtl: None }
    }

    pub fn with_value(typ: TokenType, value: String) -> Self {
        Self { typ, value: Some(value), addtl: None }
    }
}
