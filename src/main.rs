use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use clap::Parser;
use eval::ExecEnvironment;
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

            let env = Rc::new(RefCell::new(ExecEnvironment::new()));

            match parser.parse() {
                Ok(program) => {
                    program.eval_program(env);
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
