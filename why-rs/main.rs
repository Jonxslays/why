use why_rs::internal;
use why_rs::Lexer;
use why_rs::Parser;

fn main() -> Result<(), String> {
    let args = internal::collect_cli_args()?;
    let src = internal::read_source_file(&args[1])?;

    let mut lexer = Lexer::new(&src)?;
    let tokens = lexer.lex()?;

    for token in &tokens {
        println!("{:?}", token);
    }

    let mut peekable_tokens = tokens.iter().peekable();

    let mut parser = Parser::new(&mut peekable_tokens);
    let ast = parser.parse()?;

    println!("Resulting AST:");
    println!("{:?}", ast);

    println!();

    Ok(())
}
