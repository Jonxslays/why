use std::env;
use std::fs;

pub const KEYWORDS: [&str; 10] = [
    "if", "in", "is", "break", "return", "let", "else", "const", "for", "while",
];

/// Obtains the CLI args passed to the programs execution.
///
/// # Returns
/// - [`Result<String, WhyExc>`] - A vector of strings with representing
/// the arguments on success.
///
/// # Errors
/// - If the use passed no CLI args to the program.
pub fn collect_cli_args() -> Result<Vec<String>, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        Ok(args)
    } else {
        super::exc!("Missing required argument: the file to compile.")
    }
}

/// Reads a source file into memory and returns it as a string
///
/// # Returns
/// - [`Result<String, WhyExc>`] - The text from the file on success.
///
/// # Errors
/// - If the file was unable to be read for any reason.
pub fn read_source_file(filename: &String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(src) => Ok(src),
        Err(e) => super::exc!("Failed to read file: {:?}: {}", filename, e),
    }
}

// #[derive(Clone, Debug)]
// pub struct WhyExc {
//     pub message: String,
// }

// impl WhyExc {
//     #[must_use]
//     pub fn new(message: String) -> Self {
//         Self { message }
//     }
// }
