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
    ///
    /// # Returns
    /// - [`Result<Self, WhyExc>`] - The new lexer on success.
    ///
    /// # Errors
    /// - If the text file was empty.
    pub fn new(src: &str) -> Result<Self, WhyExc> {
        let src: Vec<char> = src.chars().collect();
        let src_len = src.len();
        let character = src.first();

        if let Some(c) = character {
            Ok(Self {
                c: *c,
                src,
                idx: 0,
                line: 1,
                col: 1,
                // Arbitrarily guessing a token will happen every 6 chars
                tokens: Vec::with_capacity(src_len / 6 + 1),
                errors: vec![],
            })
        } else {
            super::exc!("There was no text in the file.")
        }
    }

    /// True, if there is more src file to read.
    #[must_use]
    pub fn can_advance(&self) -> bool {
        self.idx < self.src.len() - 1
    }

    /// Returns true if the character is a newline.
    #[must_use]
    pub fn is_newline(c: char) -> bool {
        c == '\n' || c == '\r'
    }

    /// Moves to the next character for further lexing.
    pub fn next(lexer: &mut Lexer) {
        if Lexer::is_newline(lexer.c) {
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
    #[must_use]
    pub fn peek(&self, offset: isize) -> Option<char> {
        if !self.can_advance() {
            return None;
        }

        if offset < 0 {
            Some(self.src[self.idx - offset.abs() as usize])
        } else {
            Some(self.src[self.idx + offset.abs() as usize])
        }
    }

    pub fn skip_whitespace(lexer: &mut Lexer) {
        while lexer.can_advance() && lexer.peek(1).unwrap_or_default().is_whitespace() {
            Lexer::next(lexer);
        }

        Lexer::next(lexer);
    }

    /// Advances the current index/char until its no longer a comment.
    ///
    /// # Returns
    /// - [`Result<(), WhyExc>`] - Unit type on success.
    ///
    /// # Errors
    /// - If an invalid char was encountered after the initial `/`.
    pub fn skip_comment(lexer: &mut Lexer, multiline: bool) -> Result<(), WhyExc> {
        if multiline {
            // This could be a while... :)
            while lexer.can_advance() {
                if Lexer::end_multiline_comment(lexer) {
                    return Ok(());
                }

                Lexer::next(lexer);
            }
        }

        let next = lexer.peek(1).unwrap_or_default();
        if next != '/' && next != '*' && next != '=' {
            return super::lex_exc!(lexer, "Invalid character after a '/': '{}'", next);
        }

        while lexer.can_advance() {
            Lexer::next(lexer);

            if lexer.c == '=' {
                // This is a /= not a comment
                super::make_token_mut!(TokenType::SlashEq, "/=", lexer);
                return Ok(());
            } else if lexer.c == '*' {
                // We are starting a multi line comment, recurse
                return Lexer::skip_comment(lexer, true);
            } else if !multiline && lexer.c == '/' {
                // This is a single line comment
                while lexer.can_advance() && !Lexer::is_newline(lexer.c) {
                    Lexer::next(lexer);
                }

                break;
            }
        }

        Ok(())
    }

    // Determines which token this is, if the char was an equals
    #[must_use]
    pub fn get_eq_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '>' => super::make_token!(TokenType::LargeRArrow, "=>", self),
            '=' => super::make_token!(TokenType::EqEq, "==", self),
            _ => super::make_token!(TokenType::Eq, "=", self),
        }
    }

    /// Pushes an `Eq`, `EqEq`, or `LargeRArrow` token onto the token stack,
    /// and advances.
    pub fn lex_eq(lexer: &mut Lexer) {
        let token = lexer.get_eq_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Minus => (),
            _ => Lexer::next(lexer),
        }
    }

    #[must_use]
    pub fn get_minus_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '-' => super::make_token!(TokenType::MinusMinus, "--", self),
            '=' => super::make_token!(TokenType::MinusEq, "-=", self),
            '>' => super::make_token!(TokenType::SmallRArrow, "->", self),
            _ => super::make_token!(TokenType::Minus, "-", self),
        }
    }

    pub fn lex_minus(lexer: &mut Lexer) {
        let token = lexer.get_minus_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Minus => (),
            _ => Lexer::next(lexer),
        }
    }

    #[must_use]
    pub fn get_plus_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '+' => super::make_token!(TokenType::PlusPlus, "++", self),
            '=' => super::make_token!(TokenType::PlusEq, "+=", self),
            _ => super::make_token!(TokenType::Plus, "+", self),
        }
    }

    pub fn lex_plus(lexer: &mut Lexer) {
        let token = lexer.get_plus_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Plus => (),
            _ => Lexer::next(lexer),
        }
    }

    #[must_use]
    pub fn get_star_token(&self) -> Token {
        match self.peek(1).unwrap_or_default() {
            '*' => super::make_token!(TokenType::StarStar, "**", self),
            '=' => super::make_token!(TokenType::StarEq, "*=", self),
            _ => super::make_token!(TokenType::Star, "*", self),
        }
    }

    pub fn lex_star(lexer: &mut Lexer) {
        let token = lexer.get_star_token();
        lexer.tokens.push(token.clone());

        match token.typ {
            TokenType::Star => (),
            _ => Lexer::next(lexer),
        }
    }

    /// Returns true if the current and next char are `*/`.
    #[must_use]
    pub fn end_multiline_comment(lexer: &Lexer) -> bool {
        {
            lexer.c == '*' && lexer.peek(1).unwrap_or_default() == '/'
        }
    }

    /// Generate an Ident token, push to the stack, and advance.
    pub fn lex_ident(lexer: &mut Lexer) {
        let mut token = Token::at(TokenType::Ident, lexer.line, lexer.col);
        let mut name = String::new();

        while lexer.c.is_alphanumeric() || lexer.c == '_' {
            // Keep going til its some other type of character like space or semi
            name.push(lexer.c);
            Lexer::next(lexer);
        }

        token.value = name;
        lexer.tokens.push(token);
    }

    /// Generate an `Int` or `Float` token, push to the stack, and
    /// advance.
    ///
    /// # Returns
    /// - [`Result<(), WhyExc>`] - Unit type on success.
    ///
    /// # Errors
    /// - If the number had more than 1 dot in it, indicating an invalid
    /// float.
    ///     - Ex: `69.420.3` would trigger this error.
    pub fn lex_number(lexer: &mut Lexer) -> Result<(), WhyExc> {
        let mut token = Token::at(TokenType::NumLiteral, lexer.line, lexer.col);
        let mut digits = String::new();
        let mut dot_count = 0;

        while lexer.c.is_numeric() || lexer.c == '.' {
            if lexer.c == '.' {
                if dot_count > 0 {
                    // We already had a dot, there should't be another
                    return super::lex_exc!(lexer, "Invalid location for '.'");
                }

                dot_count += 1;
            }

            // Keep going til its some other type of character like space or semi
            digits.push(lexer.c);
            Lexer::next(lexer);
        }

        token.value = digits;
        lexer.tokens.push(token);
        Ok(())
    }

    /// Generate a Semi token, push to the stack, and advance.
    pub fn lex_semi(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Semi, ";", lexer);
    }

    /// Generate a Sot token, push to the stack, and advance.
    pub fn lex_dot(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Dot, ".", lexer);
    }

    pub fn lex_comma(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Comma, ",", lexer);
    }

    pub fn lex_colon(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Colon, ":", lexer);
    }

    /// Generate an `At` token, push to the stack, and advance.
    pub fn lex_at(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::At, "@", lexer);
    }

    pub fn lex_and(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::And, "&", lexer);
    }

    pub fn lex_dollar(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Dollar, "$", lexer);
    }

    pub fn lex_exclamation(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Exclamation, "!", lexer);
    }

    pub fn lex_caret(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::Caret, "^", lexer);
    }

    pub fn lex_question_mark(lexer: &mut Lexer) {
        super::make_token_mut!(TokenType::QuestionMark, "?", lexer);
    }

    /// Lexes potential closure chars like brackets and parentheses.
    ///
    /// # Returns
    /// - [`Result<(), WhyExc>`] - Unit type on success.
    ///
    /// # Errors
    /// - If this function was called incorrectly on a non enclosure
    /// type char. Valid chars: `(`, `)`, `[`, `]`, `{`, `}`
    pub fn lex_enclosures(lexer: &mut Lexer) -> Result<(), WhyExc> {
        match lexer.c {
            '(' => super::make_token_mut_ok!(TokenType::LParen, "(", lexer),
            ')' => super::make_token_mut_ok!(TokenType::RParen, ")", lexer),
            '[' => super::make_token_mut_ok!(TokenType::LBracket, "[", lexer),
            ']' => super::make_token_mut_ok!(TokenType::RBracket, "]", lexer),
            '{' => super::make_token_mut_ok!(TokenType::LBrace, "{", lexer),
            '}' => super::make_token_mut_ok!(TokenType::RBrace, "}", lexer),
            _ => super::lex_exc!(lexer, "Got unexpected enclosure: '{}'", lexer.c),
        }
    }

    /// Lexes a string token from the current position, and adds it
    /// to the lexers internal token stack.
    ///
    /// # Returns
    /// - [`Result<(), WhyExc>`] - Unit type on success.
    ///
    /// # Errors
    /// - If the string was never closed.
    pub fn lex_string(lexer: &mut Lexer) -> Result<(), WhyExc> {
        let mut content = String::new();
        let delim = lexer.c;
        Lexer::next(lexer);

        while lexer.c != delim && lexer.can_advance() {
            if lexer.c == '\\' {
                let next = lexer.peek(1).unwrap_or_default();

                if next == delim {
                    // We are escaping the delimiter
                    content.push(lexer.c);
                    content.push(next);
                    Lexer::next(lexer);
                    Lexer::next(lexer);
                    continue;
                }
            }

            content.push(lexer.c);
            Lexer::next(lexer);
        }

        if lexer.c == delim {
            super::make_token_mut_ok!(TokenType::StrLiteral, content, lexer)
        } else {
            // We never closed the quote
            super::lex_exc!(lexer, "`{}` was never closed", delim)
        }
    }

    /// Lexes the text attached to this lexer.
    ///
    /// # Returns
    /// - [`Result<Vec<Token>, WhyExc>`] - A vector containing the lexed
    /// tokens on success.
    ///
    /// # Errors
    /// - If something went wrong during lexing.
    pub fn lex(&mut self) -> Result<Vec<Token>, WhyExc> {
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
                '/' => Lexer::skip_comment(self, false)?,
                '"' | '\'' => Lexer::lex_string(self)?,
                ' ' | '\n' | '\r' => (),
                '(' | ')' | '[' | ']' | '{' | '}' => Lexer::lex_enclosures(self)?,
                _ => {
                    if self.c.is_numeric() {
                        Lexer::lex_number(self)?;
                        continue;
                    } else if self.c.is_alphabetic() {
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
        Ok(self.tokens.clone())
    }
}
