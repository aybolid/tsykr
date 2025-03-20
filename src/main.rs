use clap::Parser;
use lexer::Lexer;

mod lexer;
mod repl;

#[derive(Debug, Parser)]
struct Args {
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            let content = std::fs::read_to_string(path)?;
            let lexer = Lexer::new(content);
            for token in lexer {
                print!("{:?} ", token);
            }
            println!()
        }
        None => repl::run()?,
    }

    Ok(())
}
