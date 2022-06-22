#[macro_export]
macro_rules! exc {
    ($msg:literal) => { {
        eprintln!("Error! {}", $msg);
        std::process::exit(1);
    }};

    ($msg:literal, $($args:ident),*) => {{
        eprintln!("Error! {}", format!($msg, $($args),*));
        std::process::exit(1);
    }};

    ($msg:literal, $($args:literal),*) => {{
        eprintln!("Error! {}", format!($msg, $($args),*));
        std::process::exit(1);
    }};

    ($msg:literal, $($args:expr),*) => {{
        eprintln!("Error! {}", format!($msg, $($args),*));
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! lex_exc {
    ($lexer:ident, $msg:literal) => {{
        eprintln!("Error! {}", format!(
            "line {}, column {}:\n{}",
            $lexer.line, $lexer.col, $msg,
        ));

        std::process::exit(1);
    }};

    ($lexer:ident, $msg:literal, $($args:ident),*) => {{
        eprintln!("Error! {}", format!(
            "line {}, column {}:\n{}",
            $lexer.line, $lexer.col, format!($msg, $($args),*)
        ));

        std::process::exit(1);
    }};

    ($lexer:ident, $msg:literal, $($args:literal),*) => {{
        eprintln!("Error! {}", format!(
            "line {}, column {}:\n{}",
            $lexer.line, $lexer.col, format!($msg, $($args),*)
        ));

        std::process::exit(1);
    }};

    ($lexer:ident, $msg:literal, $($args:expr),*) => {{
        eprintln!("Error! {}", format!(
            "line {}, column {}:\n{}",
            $lexer.line, $lexer.col, format!($msg, $($args),*)
        ));

        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! make_token {
    ($typ:expr, $val:literal, $lexer:ident) => {{
        return $crate::Token::with_value_at($typ, $val.to_string(), $lexer.line, $lexer.col)
    }}
}

#[macro_export]
macro_rules! make_token_mut {
    ($typ:expr, $val:literal, $lexer:ident) => {{
        $lexer.tokens.push(
            $crate::Token::with_value_at($typ, $val.to_string(), $lexer.line, $lexer.col)
        )
    }}
}
