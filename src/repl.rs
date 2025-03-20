use std::io::Write;

use crate::{lexer::Lexer, parser::Parser};

const PROMPT: &str = ">> ";

/// Run the REPL. Yoohoo!!!
pub fn run() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    println!("Starting the tsykr REPL...");

    loop {
        print!("{PROMPT}");
        stdout.flush().unwrap();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        let lexer = Lexer::new(buf);
        let mut parser = Parser::new(lexer);

        match parser.parse() {
            Ok(program) => println!("{program:#?}"),
            Err(errs) => {
                println!("Parser errors:");
                for err in errs {
                    eprintln!("\t{err}");
                }
            }
        }
    }
}
