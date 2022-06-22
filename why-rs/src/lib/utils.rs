use std::env;
use std::fs;

pub fn collect_cli_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        super::exc!("Missing required argument: the file to compile.");
    }

    args
}

pub fn read_source_file(filename: &String) -> String {
    if let Ok(src) = fs::read_to_string(&filename) {
        src
    } else {
        super::exc!("Failed to read file: {}", filename);
    }
}

pub fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}
