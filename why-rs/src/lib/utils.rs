use std::env;
use std::fs;

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

pub fn collect_cli_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exc!("Missing required argument: the file to compile.");
    }

    args
}

pub fn read_source_file(filename: &String) -> String {
    if let Ok(src) = fs::read_to_string(&filename) {
        src
    } else {
        exc!("Failed to read file: {}", filename);
    }
}
