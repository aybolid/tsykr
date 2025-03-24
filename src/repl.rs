use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{eval::ExecEnvironment, lexer::Lexer, parser::Parser};

const PROMPT: &str = ">> ";

/// Run the REPL. Yoohoo!!!
pub fn run() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let env = Rc::new(RefCell::new(ExecEnvironment::new()));

    println!("Starting tsykr REPL...");

    loop {
        let mut print_debug = false;

        print!("{PROMPT}");
        stdout.flush().unwrap();
        let mut buf = String::new();

        stdin.read_line(&mut buf).unwrap();

        if buf == "?env\n" {
            println!("{:#?}", env.borrow().store);
            continue;
        }

        if buf.starts_with("?") {
            print_debug = true;
            buf = buf.strip_prefix("?").unwrap().to_string();
        }

        let lexer = Lexer::new(buf);
        let mut parser = Parser::new(lexer);

        match parser.parse() {
            Ok(program) => {
                let result = program.eval_program(Rc::clone(&env));
                if print_debug {
                    println!("{:#?}", result);
                } else {
                    if let Some(value) = result {
                        println!("{}", value.inspect());
                    }
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
}
