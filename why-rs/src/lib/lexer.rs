use super::utils;
use super::Loc;
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

    /// Expects a given char to come next.
    /// If it does, return a new token of type `typ`.
    /// If it does not, return None
    fn expect(&self, typ: TokenType, c: char) -> Option<Token> {
        if self.peek(1).unwrap_or_default() == c {
            Some(Token::new(typ))
        } else {
            None
        }
    }

    /// Moves to the next character for further lexing.
    fn next(lexer: &mut Lexer) {
        if lexer.can_advance() {
            if utils::is_newline(lexer.c) {
                // If its a newline, reset the column and add a line
                lexer.line += 1;
                lexer.col = 1;
            } else {
                // Otherwise just increase the column
                lexer.col += 1;
            }

            // The index always increases
            lexer.idx += 1;
            // Get the next character
            lexer.c = lexer.src[lexer.idx];
        }
    }

    /// Moves to the next character, and also pushes a token.
    fn next_with(lexer: &mut Lexer, token: Token) {
        Lexer::next(lexer);
        lexer.tokens.push(token);
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
        if let Some(mut token) = self.expect(TokenType::LargeRArrow, '>') {
            // This is a fat right arrow
            token.value = "=>".to_string();
            token.loc = Loc::at(self.line, self.col);
            token
        } else if let Some(mut token) = self.expect(TokenType::EqEq, '=') {
            // This is a double equals
            token.value = "==".to_string();
            token.loc = Loc::at(self.line, self.col);
            token
        } else {
            // Regular equals
            Token::with_value_at(TokenType::Eq, "=".to_string(), self.line, self.col)
        }
    }

    /// Pushes an Eq, EqEq, or LargeRArrow token onto the token stack,
    /// and advances.
    fn lex_eq(lexer: &mut Lexer) {
        let token = lexer.get_eq_token();

        if let TokenType::Eq = token.typ {
            Lexer::next_with(lexer, token);
        } else {
            // The other eq tokens take 2 chars
            Lexer::next(lexer);
            Lexer::next_with(lexer, token);
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
        if !['/', '*'].contains(&next) {
            super::exc!("Invalid character after a '/': '{}'", next);
        }

        while lexer.can_advance() {
            Lexer::next(lexer);

            if lexer.c == '*' {
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

        while lexer.c.is_alphanumeric() {
            // Keep going til its some other type of character like space or semi
            name.push(lexer.c);
            Lexer::next(lexer);
        }

        let length = name.len();
        let token = Token::with_value_at(TokenType::Ident, name, lexer.line, lexer.col - length);
        lexer.tokens.push(token);
    }

    /// Generate a Semi token, push to the stack, and advance.
    fn lex_semi(lexer: &mut Lexer) {
        lexer.tokens.push(
            Token::with_value_at(TokenType::Semi, ";".to_string(), lexer.line, lexer.col)
        )
    }

    /// Generate a Sot token, push to the stack, and advance.
    fn lex_dot(lexer: &mut Lexer) {
        lexer.tokens.push(
            Token::with_value_at(TokenType::Dot, ".".to_string(), lexer.line, lexer.col),
        );
    }

    /// Generate an At token, push to the stack, and advance.
    fn lex_at(lexer: &mut Lexer) {
        lexer.tokens.push(
            Token::with_value_at(TokenType::At, "@".to_string(), lexer.line, lexer.col),
        );
    }

    fn lex_and(lexer: &mut Lexer) {
        lexer.tokens.push(
            Token::with_value_at(TokenType::And, "&".to_string(), lexer.line, lexer.col),
        );
    }

    fn lex_dollar(lexer: &mut Lexer) {
        lexer.tokens.push(
            Token::with_value_at(TokenType::Dollar, "$".to_string(), lexer.line, lexer.col),
        );
    }

    fn lex_exclamation(lexer: &mut Lexer) {
        lexer.tokens.push(
            Token::with_value_at(TokenType::Exclamation, "!".to_string(), lexer.line, lexer.col),
        );
    }

    /// Lexes the text attached to this lexer. Returns a vector of the lexed tokens.
    ///
    /// The same tokens are also available at `lexer.tokens`.
    pub fn lex(&mut self) -> Vec<Token> {
        let _in_string = false;

        loop {
            println!("Index: {}, Char: {:?}", self.idx, self.c);

            match self.c {
                ' ' | '\n' | '\r' => (),
                '=' => Lexer::lex_eq(self),
                '&' => Lexer::lex_and(self),
                ';' => Lexer::lex_semi(self),
                '.' => Lexer::lex_dot(self),
                '@' => Lexer::lex_at(self),
                '$' => Lexer::lex_dollar(self),
                '!' => Lexer::lex_exclamation(self),
                '/' => Lexer::skip_comment(self, false),
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

        self.tokens.push(Token::at(TokenType::Eof, self.line, self.col));
        self.tokens.clone()
    }
}
