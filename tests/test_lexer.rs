#[allow(dead_code)] // TODO: Add more tests from this code here.
const SOURCE: &'static str = {
    r#"

    // Iterate each item in the list
    // equivalent to for each loops
my_list@->item {
    // % is a qualifier for the stdlib (not required)
    // but it can allow you to shadow builtins
    %print(item);
}
// Iterate from 1 to 20 exclusive
1->20@->number {
    print(number);
}
// Iterate from 1 to 20 inclusive
1=>20@->number {
    print(number);
}
// Iterate the mapping keys and access each element
my_dict@->key {
    print(dict@key);
    // or
    print(dict[key]);
    // or
    print(dict.get(key))
    // Keep in mind this last one can return NULL
}
// Iterate the mapping keys and values simultaneously
my_dict@=>(key, value) {
    print(key);
    print(value);
}
int number = 0;
// A while true loop
@->true! {
    (number > 100)? {
        @!; // Break out of the loop early
    } !-> {
        print("Number: {++number}");
    }
}
"#
};

#[cfg(test)]
mod test_lexer {
    // use crate::SOURCE;
    use why_rs::Lexer;
    use why_rs::Loc;
    use why_rs::Token;
    use why_rs::TokenType;

    #[test]
    fn test_new() -> Result<(), String> {
        let src = "Rewrite it in rust.";
        let lexer = Lexer::new(src)?;

        assert_eq!(lexer.c, 'R');
        assert_eq!(lexer.idx, 0);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 1);
        assert_eq!(lexer.src.len(), src.len());
        assert_eq!(lexer.src.iter().collect::<String>(), src.to_string());
        assert!(lexer.tokens.is_empty());
        println!("{}", lexer.tokens.capacity());
        assert!(lexer.tokens.capacity() == 4);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_new_fails_with_no_src_content() {
        let src = "";
        let _ = Lexer::new(src).unwrap();
    }

    #[test]
    fn test_can_advance() -> Result<(), String> {
        let src = "hikari-py.dev - check it out!";
        let mut lexer = Lexer::new(src)?;

        assert!(lexer.can_advance());
        lexer.idx += src.len() - 1;
        assert!(!lexer.can_advance());

        Ok(())
    }

    #[test]
    fn test_is_newline() {
        let newline = '\n';
        let carriage = '\r';
        let underscore = '_';

        assert!(Lexer::is_newline(newline));
        assert!(Lexer::is_newline(carriage));
        assert!(!Lexer::is_newline(underscore));
    }

    #[test]
    fn test_skip_whitespace() -> Result<(), String> {
        let src = "a    b";
        let mut lexer = Lexer::new(src)?;

        Lexer::skip_whitespace(&mut lexer);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 6);
        assert_eq!(lexer.idx, 5);

        Ok(())
    }

    #[test]
    fn test_skip_single_line_comment() -> Result<(), String> {
        let src = "// This is a comment\nhello";
        let mut lexer = Lexer::new(src)?;

        Lexer::skip_comment(&mut lexer, false)?;
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 21);
        assert_eq!(lexer.idx, 20);

        Ok(())
    }

    #[test]
    fn test_skip_multiline_comment() -> Result<(), String> {
        let src = "/* This is a comment\nhello */\nwoo";
        let mut lexer = Lexer::new(src)?;

        Lexer::skip_comment(&mut lexer, true)?;
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.col, 9);
        assert_eq!(lexer.idx, 29);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_invalid_comment() {
        let src = "/ / why is there a space there weirdo?";
        let mut lexer = Lexer::new(src).unwrap();

        Lexer::skip_comment(&mut lexer, false).unwrap();
    }

    #[test]
    fn test_end_multiline_comment() -> Result<(), String> {
        let multiline_ending = "*/";
        let not_multiline_ending = "*69420";
        let ending_lexer = Lexer::new(multiline_ending)?;
        let not_ending_lexer = Lexer::new(not_multiline_ending)?;

        assert!(Lexer::end_multiline_comment(&ending_lexer));
        assert!(!Lexer::end_multiline_comment(&not_ending_lexer));

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_eq_token() -> Result<(), String> {
        let src = "=";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_eq_token(),
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_eq_token_eq_eq() -> Result<(), String> {
        let src = "==";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_eq_token(),
            Token { typ: TokenType::EqEq, value: "==".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_eq_token_large_r_arrow() -> Result<(), String> {
        let src = "=>";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_eq_token(),
            Token { typ: TokenType::LargeRArrow, value: "=>".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_eq() -> Result<(), String> {
        let src = "= :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_eq(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_eq_other() -> Result<(), String> {
        let src = "=> :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_eq(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::LargeRArrow, value: "=>".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_minus_token() -> Result<(), String> {
        let src = "-";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_minus_token(),
            Token { typ: TokenType::Minus, value: "-".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_minus_token_minus_minus() -> Result<(), String> {
        let src = "--";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_minus_token(),
            Token { typ: TokenType::MinusMinus, value: "--".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_minus_token_minus_eq() -> Result<(), String> {
        let src = "-=";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_minus_token(),
            Token { typ: TokenType::MinusEq, value: "-=".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_minus_token_small_r_arrow() -> Result<(), String> {
        let src = "->";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_minus_token(),
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_minus() -> Result<(), String> {
        let src = "- :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_minus(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::Minus, value: "-".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_minus_other() -> Result<(), String> {
        let src = "-> :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_minus(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_plus_token() -> Result<(), String> {
        let src = "+";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_plus_token(),
            Token { typ: TokenType::Plus, value: "+".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_plus_token_plus_plus() -> Result<(), String> {
        let src = "++";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_plus_token(),
            Token { typ: TokenType::PlusPlus, value: "++".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_plus_token_plus_eq() -> Result<(), String> {
        let src = "+=";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_plus_token(),
            Token { typ: TokenType::PlusEq, value: "+=".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_plus() -> Result<(), String> {
        let src = "+ :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_plus(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::Plus, value: "+".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_plus_other() -> Result<(), String> {
        let src = "++ :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_plus(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::PlusPlus, value: "++".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_star_token() -> Result<(), String> {
        let src = "*";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_star_token(),
            Token { typ: TokenType::Star, value: "*".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_star_token_star_star() -> Result<(), String> {
        let src = "**";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_star_token(),
            Token { typ: TokenType::StarStar, value: "**".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_star_token_star_eq() -> Result<(), String> {
        let src = "*=";
        let lexer = Lexer::new(src)?;

        assert_eq!(
            lexer.get_star_token(),
            Token { typ: TokenType::StarEq, value: "*=".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_star() -> Result<(), String> {
        let src = "* :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_star(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::Star, value: "*".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_star_other() -> Result<(), String> {
        let src = "** :)";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_star(&mut lexer);

        assert_eq!(
            lexer.tokens[0],
            Token { typ: TokenType::StarStar, value: "**".to_string(), loc: Loc::default(), addtl: None },
        );

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_ident() -> Result<(), String> {
        let src = "hello world;";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "hello".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::Ident, value: "world".to_string(), loc: Loc { line: 1, col: 7 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 12 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 13 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_number() -> Result<(), String> {
        let src = "123;";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::NumLiteral(false), value: "123".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 4 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 5 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_semi() -> Result<(), String> {
        let src = ";";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_semi(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_dot() -> Result<(), String> {
        let src = ".";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_dot(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Dot, value: ".".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_comma() -> Result<(), String> {
        let src = ",";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_comma(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_colon() -> Result<(), String> {
        let src = ",";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_colon(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Colon, value: ":".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_at() -> Result<(), String> {
        let src = "@";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_at(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_and() -> Result<(), String> {
        let src = ",";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_and(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::And, value: "&".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_dollar() -> Result<(), String> {
        let src = "$";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_dollar(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Dollar, value: "$".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_exclamation() -> Result<(), String> {
        let src = "!";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_exclamation(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Exclamation, value: "!".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_caret() -> Result<(), String> {
        let src = "^";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_caret(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::Caret, value: "^".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_question_mark() -> Result<(), String> {
        let src = ",";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_question_mark(&mut lexer);

        let expected_tokens = vec![
            Token { typ: TokenType::QuestionMark, value: "?".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
        ];

        assert_eq!(expected_tokens, lexer.tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_enclosures() -> Result<(), String> {
        let src = "[](){}";
        let mut lexer = Lexer::new(src)?;

        Lexer::lex_enclosures(&mut lexer)?;
        Lexer::next(&mut lexer);
        assert_eq!(
            Token { typ: TokenType::LBracket, value: "[".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Lexer::lex_enclosures(&mut lexer)?;
        Lexer::next(&mut lexer);
        assert_eq!(
            Token { typ: TokenType::RBracket, value: "]".to_string(), loc: Loc { line: 1, col: 2 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Lexer::lex_enclosures(&mut lexer)?;
        Lexer::next(&mut lexer);
        assert_eq!(
            Token { typ: TokenType::LParen, value: "(".to_string(), loc: Loc { line: 1, col: 3 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Lexer::lex_enclosures(&mut lexer)?;
        Lexer::next(&mut lexer);
        assert_eq!(
            Token { typ: TokenType::RParen, value: ")".to_string(), loc: Loc { line: 1, col: 4 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Lexer::lex_enclosures(&mut lexer)?;
        Lexer::next(&mut lexer);
        assert_eq!(
            Token { typ: TokenType::LBrace, value: "{".to_string(), loc: Loc { line: 1, col: 5 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Lexer::lex_enclosures(&mut lexer)?;
        Lexer::next(&mut lexer);
        assert_eq!(
            Token { typ: TokenType::RBrace, value: "}".to_string(), loc: Loc { line: 1, col: 6 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_lex_enclosures_fails() {
        let src = "lol";
        let mut lexer = Lexer::new(src).unwrap();
        Lexer::lex_enclosures(&mut lexer).unwrap();
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_double_quoted_string() -> Result<(), String> {
        let src = "\"goodbye, world!\"";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_string(&mut lexer)?;

        assert_eq!(
            Token { typ: TokenType::StrLiteral, value: "goodbye, world!".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_lex_double_quoted_string_fails() {
        let src = "\"goodbye, world!";
        let mut lexer = Lexer::new(src).unwrap();
        Lexer::lex_string(&mut lexer).unwrap();
    }

    #[test]
    #[rustfmt::skip]
    fn test_lex_single_quoted_string() -> Result<(), String> {
        let src = "'lolcat'";
        let mut lexer = Lexer::new(src)?;
        Lexer::lex_string(&mut lexer)?;

        assert_eq!(
            Token { typ: TokenType::StrLiteral, value: "lolcat".to_string(), loc: Loc { line: 1, col: 1 }, addtl: None },
            lexer.tokens.pop().unwrap(),
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_lex_single_quoted_string_fails() {
        let src = "'goodbye, world!";
        let mut lexer = Lexer::new(src).unwrap();
        Lexer::lex_string(&mut lexer).unwrap();
    }

    #[test]
    fn test_next() -> Result<(), String> {
        let src = "123\n\r";
        let mut lexer = Lexer::new(src)?;

        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 1);
        assert_eq!(lexer.idx, 0);

        Lexer::next(&mut lexer);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 2);
        assert_eq!(lexer.idx, 1);

        Lexer::next(&mut lexer);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 3);
        assert_eq!(lexer.idx, 2);

        Lexer::next(&mut lexer);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 4);
        assert_eq!(lexer.idx, 3);

        Lexer::next(&mut lexer);
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.col, 1);
        assert_eq!(lexer.idx, 4);

        Ok(())
    }

    #[test]
    fn test_peek() -> Result<(), String> {
        let src = "abc";
        let lexer = Lexer::new(src)?;

        assert_eq!(lexer.peek(1).unwrap(), 'b');
        assert_eq!(lexer.peek(2).unwrap(), 'c');
        assert!(lexer.peek(3).is_none());

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexing_int_definition() -> Result<(), String> {
        let src = "int my_num = 69;";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::Ident, value: "my_num".to_string(), loc: Loc { line: 1, col: 5 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 12 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "69".to_string(), loc: Loc { line: 1, col: 14 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 16 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 17 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexing_float_definition() -> Result<(), String> {
        let src = "float my_float = 69.420;";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "float".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::Ident, value: "my_float".to_string(), loc: Loc { line: 1, col: 7 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 16 }, addtl: None },
            Token { typ: TokenType::NumLiteral(true), value: "69.420".to_string(), loc: Loc { line: 1, col: 18 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 24 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 25 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_lexing_fails_on_invalid_float_definition() {
        let src = "float my_float = 69.420.69;";
        let mut lexer = Lexer::new(src).unwrap();
        let _received_tokens = lexer.lex().unwrap();
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexing_array_definition() -> Result<(), String> {
        let src = "array@int my_list = [1, 2, 3, 4, 5, 6, 7];";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "array".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 1, col: 6 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 7 }, addtl: None },
            Token { typ: TokenType::Ident, value: "my_list".to_string(), loc: Loc { line: 1, col: 11 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 19 }, addtl: None },
            Token { typ: TokenType::LBracket, value: "[".to_string(), loc: Loc { line: 1, col: 21 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "1".to_string(), loc: Loc { line: 1, col: 22 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 23 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "2".to_string(), loc: Loc { line: 1, col: 25 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 26 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "3".to_string(), loc: Loc { line: 1, col: 28 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 29 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "4".to_string(), loc: Loc { line: 1, col: 31 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 32 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "5".to_string(), loc: Loc { line: 1, col: 34 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 35 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "6".to_string(), loc: Loc { line: 1, col: 37 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 38 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "7".to_string(), loc: Loc { line: 1, col: 40 }, addtl: None },
            Token { typ: TokenType::RBracket, value: "]".to_string(), loc: Loc { line: 1, col: 41 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 42 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 43 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexing_function_definition() -> Result<(), String> {
        let src = "$(int num) @int {\n    @num;\n}";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Dollar, value: "$".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::LParen, value: "(".to_string(), loc: Loc { line: 1, col: 2 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 3 }, addtl: None },
            Token { typ: TokenType::Ident, value: "num".to_string(), loc: Loc { line: 1, col: 7 }, addtl: None },
            Token { typ: TokenType::RParen, value: ")".to_string(), loc: Loc { line: 1, col: 10 }, addtl: None },
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 1, col: 12 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 13 }, addtl: None },
            Token { typ: TokenType::LBrace, value: "{".to_string(), loc: Loc { line: 1, col: 17 }, addtl: None },
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 2, col: 5 }, addtl: None },
            Token { typ: TokenType::Ident, value: "num".to_string(), loc: Loc { line: 2, col: 6 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 2, col: 9 }, addtl: None },
            Token { typ: TokenType::RBrace, value: "}".to_string(), loc: Loc { line: 3, col: 1 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 3, col: 2 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexing_mapping_definition() -> Result<(), String> {
        let src = "mapping@int->int my_dict = &{ 1->2, 3->4 };";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens: Vec<Token> = vec![
            Token { typ: TokenType::Ident, value: "mapping".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 1, col: 8 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 9 }, addtl: None },
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc { line: 1, col: 12 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 14 }, addtl: None },
            Token { typ: TokenType::Ident, value: "my_dict".to_string(), loc: Loc { line: 1, col: 18 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 26 }, addtl: None },
            Token { typ: TokenType::And, value: "&".to_string(), loc: Loc { line: 1, col: 28 }, addtl: None },
            Token { typ: TokenType::LBrace, value: "{".to_string(), loc: Loc { line: 1, col: 29 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "1".to_string(), loc: Loc { line: 1, col: 31 }, addtl: None },
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc { line: 1, col: 32 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "2".to_string(), loc: Loc { line: 1, col: 34 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 35 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "3".to_string(), loc: Loc { line: 1, col: 37 }, addtl: None },
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc { line: 1, col: 38 }, addtl: None },
            Token { typ: TokenType::NumLiteral(false), value: "4".to_string(), loc: Loc { line: 1, col: 40 }, addtl: None },
            Token { typ: TokenType::RBrace, value: "}".to_string(), loc: Loc { line: 1, col: 42 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 43 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 44 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }
}
