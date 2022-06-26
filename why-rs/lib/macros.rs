#[macro_export]
macro_rules! exc {
    ($msg:literal) => { {
        Err(format!("Error! --> {}", $msg))
    }};

    ($msg:literal, $($args:ident),*) => {{
        Err(format!("Error! --> {}", format!($msg, $($args),*)))
    }};

    ($msg:literal, $($args:literal),*) => {{
        Err(format!("Error! --> {}", format!($msg, $($args),*)))
    }};

    ($msg:literal, $($args:expr),*) => {{
        Err(format!("Error! --> {}", format!($msg, $($args),*)))
    }};
}

#[macro_export]
macro_rules! lex_exc {
    ($lexer:ident, $msg:literal) => {{
        let error = format!(
            "line {}, column {}: --> {}",
            $lexer.line, $lexer.col, $msg,
        );

        Err(error)
    }};

    ($lexer:ident, $msg:literal, $($args:ident),*) => {{
        let error = format!(
            "line {}, column {}: --> {}",
            $lexer.line, $lexer.col, format!($msg, $($args),*)
        );

        Err(error)
    }};

    ($lexer:ident, $msg:literal, $($args:literal),*) => {{
        let error = format!(
            "line {}, column {}: --> {}",
            $lexer.line, $lexer.col, format!($msg, $($args),*)
        );

        Err(error)
    }};

    ($lexer:ident, $msg:literal, $($args:expr),*) => {{
        let error = format!(
            "line {}, column {}: --> {}",
            $lexer.line, $lexer.col, format!($msg, $($args),*)
        );

        Err(error)
    }};
}

#[macro_export]
macro_rules! make_token {
    ($typ:expr, $val:literal, $lexer:ident) => {{
        $crate::Token::with_value_at($typ, $val.to_string(), $lexer.line, $lexer.col)
    }};

    ($typ:expr, $val:expr, $lexer:ident) => {{
        $crate::Token::with_value_at($typ, $val.to_string(), $lexer.line, $lexer.col)
    }};
}

#[macro_export]
macro_rules! make_token_mut {
    ($typ:expr, $val:literal, $lexer:ident) => {{
        $lexer.tokens.push($crate::make_token!($typ, $val, $lexer))
    }};

    ($typ:expr, $val:expr, $lexer:ident) => {{
        $lexer.tokens.push($crate::make_token!($typ, $val, $lexer))
    }};
}

#[macro_export]
macro_rules! make_token_mut_ok {
    ($typ:expr, $val:literal, $lexer:ident) => {{
        $crate::make_token_mut!($typ, $val, $lexer);
        Ok(())
    }};

    ($typ:expr, $val:expr, $lexer:ident) => {{
        $crate::make_token_mut!($typ, $val, $lexer);
        Ok(())
    }};
}
