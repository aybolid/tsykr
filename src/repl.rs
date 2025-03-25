use std::{io::Write, rc::Rc};

use crate::{
    eval::{Eval, ExecutionEnvironment},
    lexer::Lexer,
    parser::Parser,
};

const PROMPT: &str = ">> ";

/// Run the REPL. Yoohoo!!!
pub fn run() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let env = ExecutionEnvironment::new_global();

    println!("Starting tsykr REPL...\n");

    loop {
        print!("{PROMPT}");
        stdout.flush().unwrap();
        let mut buf = String::new();

        stdin.read_line(&mut buf).unwrap();

        let mut debug_print = false;
        if buf.starts_with("?") {
            debug_print = true;
            buf = buf.strip_prefix("?").unwrap().to_string();
        }

        let lexer = Lexer::new(buf);
        let mut parser = Parser::new(lexer);

        match parser.parse() {
            Ok(program) => match program.eval(Rc::clone(&env)) {
                Ok(value) => {
                    if debug_print {
                        println!("   {:?}", value)
                    } else {
                        println!("   {}", value.to_string())
                    }
                }
                Err(err) => eprintln!("   Evaluation error: {err}"),
            },
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
