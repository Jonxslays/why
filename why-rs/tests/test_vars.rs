const SOURCE: &'static str = r#"
#one = /* multi line comment */ 1 ;
// This is a comment
#two = 2 ;

/*
 * Actual multiline comment
 */

#three = 3 ;
"#;

#[cfg(test)]
mod test_vars {
    use crate::SOURCE;
    use why_rs::Lexer;

    #[test]
    fn test_lexer_lexes_vars() {
        // Failing on purpose now
        let mut lexer = Lexer::new(SOURCE.to_string());
        lexer.lex();
    }
}
