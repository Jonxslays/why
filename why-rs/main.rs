use why_rs::internal;
use why_rs::Lexer;
use why_rs::Parser;
use why_rs::WhyExc;

fn main() -> Result<(), WhyExc> {
    let args = internal::collect_cli_args()?;
    let src = internal::read_source_file(&args[1])?;

    let mut lexer = Lexer::new(&src)?;
    let tokens = lexer.lex()?;

    let mut parser = Parser::new(tokens);
    parser.parse();

    Ok(())
}
