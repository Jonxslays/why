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

pub struct Token {
    typ: TokenType,
    value: Option<String>,
    addtl: Option<Vec<String>>,
}
