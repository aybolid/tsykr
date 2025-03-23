use std::io::Write;

use crate::{eval::ExecEnvironment, lexer::Lexer, parser::Parser};

const PROMPT: &str = ">> ";

/// Run the REPL. Yoohoo!!!
pub fn run() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut env = ExecEnvironment::new();

    println!("Starting tsykr REPL...");

    loop {
        print!("{PROMPT}");
        stdout.flush().unwrap();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        let lexer = Lexer::new(buf);
        let mut parser = Parser::new(&lexer);
        lexer;
        match parser.parse() {
            Ok(program) => {
                program.eval_program(&mut env);
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
}
