use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing required argument: the file to compile.");
        std::process::exit(1);
    }

    println!("Hello, world!");
}
