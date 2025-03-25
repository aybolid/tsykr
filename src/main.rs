use anyhow::Result;
use clap::Parser;
use eval::{Eval, ExecutionEnvironment};
use lexer::Lexer;

mod eval;
mod lexer;
mod parser;
mod repl;

#[derive(Debug, Parser)]
struct Args {
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            let content = std::fs::read_to_string(path)?;

            let lexer = Lexer::new(content);
            let mut parser = parser::Parser::new(lexer);

            let env = ExecutionEnvironment::new_global();

            match parser.parse() {
                Ok(program) => {
                    let result = program.eval(env);
                    match result {
                        Ok(v) => println!("{}", v.to_string()),
                        Err(err) => eprintln!("Runtime error: {err}"),
                    }
                }
                Err(errs) => {
                    eprintln!("Parser errors:");
                    for err in errs {
                        eprintln!("\t{err}");
                    }
                    eprintln!()
                }
            }
        }
        None => {
            repl::run();
        }
    }

    Ok(())
}
