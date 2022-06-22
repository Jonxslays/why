use super::utils;
use super::Token;
use super::TokenType;
use super::WhyExc;

/// A lexer, for generating tokens from text.
#[derive(Clone, Debug)]
pub struct Lexer {
    pub src: Vec<char>,
    pub idx: usize,
    pub line: usize,
    pub col: usize,
    pub c: char,
    pub tokens: Vec<Token>,
    pub errors: Vec<WhyExc>,
}

impl Lexer {
    /// Creates a new lexer to be used on a given string.
    pub fn new(src: String) -> Self {
        let src: Vec<char> = src.chars().collect();
        let src_len = src.len();
        let c = *src.first().unwrap_or_else(|| {
            super::exc!("No text in the file!!!");
        });

        Self {
            c,
            src,
            idx: 0,
            line: 1,
            col: 1,
            // Arbitrarily guessing a token will happen every 6 chars
            tokens: Vec::with_capacity(src_len / 6 + 1),
            errors: vec![],
        }
    }

    /// True, if there is more src file to read.
    fn can_advance(&self) -> bool {
        self.idx < self.src.len() - 1
    }

    /// Moves to the next character for further lexing.
    fn next(lexer: &mut Lexer) {
        if utils::is_newline(lexer.c) {
            // If its a newline, reset the column and add a line
            lexer.line += 1;
            lexer.col = 1;
        } else {
            // Otherwise just increase the column
            lexer.col += 1;
        }

        if lexer.can_advance() {
            // The index always increases
            lexer.idx += 1;
            // Get the next character
            lexer.c = lexer.src[lexer.idx];
        }
    }

    /// Peeks at some other character nearby.
    fn peek(&self, offset: isize) -> Option<char> {
        if !self.can_advance() {
            return None;
        }

        // The character at the current index + the offset
        Some(self.src[(self.idx as isize + offset) as usize])
    }

    // Determines which token this is, if the char was an equals
    fn get_eq_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '>' => super::make_token!(TokenType::LargeRArrow, "=>", self),
            '=' => super::make_token!(TokenType::EqEq, "==", self),
            _ => super::make_token!(TokenType::Eq, "=", self),
        }
    }

    /// Pushes an Eq, EqEq, or LargeRArrow token onto the token stack,
    /// and advances.
    fn lex_eq(lexer: &mut Lexer) {
        let token = lexer.get_eq_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Minus => (),
            _ => Lexer::next(lexer),
        }
    }

    fn get_minus_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '-' => super::make_token!(TokenType::MinusMinus, "--", self),
            '=' => super::make_token!(TokenType::MinusEq, "-=", self),
            '>' => super::make_token!(TokenType::SmallRArrow, "-=", self),
            _ => super::make_token!(TokenType::Minus, "-", self),
        }
    }

    fn lex_minus(lexer: &mut Lexer) {
        let token = lexer.get_minus_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Minus => (),
            _ => Lexer::next(lexer),
        }
    }

    fn get_plus_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '+' => super::make_token!(TokenType::PlusPlus, "++", self),
            '=' => super::make_token!(TokenType::PlusEq, "+=", self),
            _ => super::make_token!(TokenType::Plus, "+", self),
        }
    }

    fn lex_plus(lexer: &mut Lexer) {
        let token = lexer.get_plus_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Plus => (),
            _ => Lexer::next(lexer),
        }
    }

    fn get_star_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '*' => super::make_token!(TokenType::StarStar, "**", self),
            '=' => super::make_token!(TokenType::StarEq, "*=", self),
            _ => super::make_token!(TokenType::Star, "*", self),
        }
    }

    fn lex_star(lexer: &mut Lexer) {
        let token = lexer.get_star_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Star => (),
            _ => Lexer::next(lexer),
        }
    }

    /// Returns true if the current and next char are `*/`.
    fn end_multiline_comment(lexer: &Lexer) -> bool {
        {
            lexer.c == '*' &&
            lexer.peek(1).unwrap_or_default() == '/'
        }
    }

    /// Advances the current index/char until its no longer a comment.
    fn skip_comment(lexer: &mut Lexer, multiline: bool) {
        if multiline {
            // This could be a while... :)
            while lexer.can_advance() {
                if Lexer::end_multiline_comment(lexer) {
                    return;
                }

                Lexer::next(lexer);
            }
        }

        let next = lexer.peek(1).unwrap_or_default();
        if !['/', '*', '='].contains(&next) {
            super::lex_exc!(lexer, "Invalid character after a '/': '{}'", next);
        }

        while lexer.can_advance() {
            Lexer::next(lexer);

            if lexer.c == '=' {
                // This is a /= not a comment
                return super::make_token_mut!(TokenType::SlashEq, "/=", lexer);
            } else if lexer.c == '*' {
                // We are starting a multi line comment, recurse
                return Lexer::skip_comment(lexer, true);
            } else if !multiline && lexer.c == '/' {
                // This is a single line comment
                while lexer.can_advance() && !utils::is_newline(lexer.c) {
                    Lexer::next(lexer);
                }

                break;
            }
        }
    }

    /// Generate an Ident token, push to the stack, and advance.
    fn lex_ident(lexer: &mut Lexer) {
        let mut name = String::new();
        let mut token = Token::at(TokenType::Ident, lexer.line, lexer.col);

        while lexer.c.is_alphanumeric() || lexer.c == '_' {
            // Keep going til its some other type of character like space or semi
            name.push(lexer.c);
            Lexer::next(lexer);
        }

        token.value = name;
        lexer.tokens.push(token);
    }

    /// Generate a Semi token, push to the stack, and advance.
    fn lex_semi(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Semi, ";", lexer)
    }

    /// Generate a Sot token, push to the stack, and advance.
    fn lex_dot(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Dot, ".", lexer);
    }

    fn lex_comma(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Comma, ",", lexer);
    }

    fn lex_colon(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Colon, ":", lexer);
    }


    /// Generate an At token, push to the stack, and advance.
    fn lex_at(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::At, "@", lexer);
    }

    fn lex_and(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::And, "&", lexer);
    }

    fn lex_dollar(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Dollar, "$", lexer);
    }

    fn lex_exclamation(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Exclamation, "!", lexer);
    }

    fn lex_caret(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Caret, "^", lexer);
    }

    fn lex_question_mark(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::QuestionMark, "?", lexer);
    }

    fn lex_enclosures(lexer: &mut Lexer) {
        match lexer.c {
            '(' => super::make_token_mut!(TokenType::LParen, "(", lexer),
            ')' => super::make_token_mut!(TokenType::RParen, ")", lexer),
            '[' => super::make_token_mut!(TokenType::LBracket, "[", lexer),
            ']' => super::make_token_mut!(TokenType::RBracket, "]", lexer),
            '{' => super::make_token_mut!(TokenType::LBrace, "{", lexer),
            '}' => super::make_token_mut!(TokenType::RBrace, "}", lexer),
            _ => super::lex_exc!(lexer, "Got unexpected enclosure: '{}'", lexer.c),
        }
    }

    /// Lexes the text attached to this lexer. Returns a vector of the lexed tokens.
    ///
    /// The same tokens are also available at `lexer.tokens`.
    pub fn lex(&mut self) -> Vec<Token> {
        let _in_string = false;

        loop {
            println!("Index: {}, Char: {:?}", self.idx, self.c);

            match self.c {
                '=' => Lexer::lex_eq(self),
                '&' => Lexer::lex_and(self),
                ';' => Lexer::lex_semi(self),
                '.' => Lexer::lex_dot(self),
                ',' => Lexer::lex_comma(self),
                ':' => Lexer::lex_colon(self),
                '@' => Lexer::lex_at(self),
                '$' => Lexer::lex_dollar(self),
                '!' => Lexer::lex_exclamation(self),
                '-' => Lexer::lex_minus(self),
                '+' => Lexer::lex_plus(self),
                '*' => Lexer::lex_star(self),
                '^' => Lexer::lex_caret(self),
                '?' => Lexer::lex_question_mark(self),
                '/' => Lexer::skip_comment(self, false),
                '(' | ')' | '[' | ']' | '{' | '}' => Lexer::lex_enclosures(self),
                ' ' | '\n' | '\r' => (),
                _ => {
                    if self.c.is_alphanumeric() {
                        Lexer::lex_ident(self);
                        continue;
                    }
                }
            }

            if !self.can_advance() {
                break;
            }



            Lexer::next(self);
        }

        Lexer::next(self);

        super::make_token_mut!(TokenType::Eof, "", self);
        self.tokens.clone()
    }
}
