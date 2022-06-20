use std::env;
use std::fs;

pub fn collect_cli_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing required argument: the file to compile.");
        std::process::exit(1);
    }

    args
}

pub fn read_source_file(filename: &String) -> String {
    if let Ok(src) = fs::read_to_string(&filename) {
        src
    } else {
        eprintln!("Failed to read file: {}", filename);
        std::process::exit(1);
    }
}
