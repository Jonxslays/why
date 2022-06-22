use why_rs::utils;
use why_rs::Lexer;

fn main() {
    let args = utils::collect_cli_args();
    let src = utils::read_source_file(&args[1]);

    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();

    for token in &tokens {
        println!("{:?}", token);
    }
}