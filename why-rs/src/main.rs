use why_rs::Lexer;
use why_rs::utils;

fn main() {
    let args = utils::collect_cli_args();
    let src = utils::read_source_file(&args[1]);
    let mut lexer = Lexer::new(src);
    lexer.lex();

    println!("{}", lexer.peek(5));
}
