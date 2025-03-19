use std::io::Write;

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

/// Run the REPL. Yoohoo!!!
pub fn run() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("{PROMPT}");
        stdout.flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let lexer = Lexer::new(buf);
        for token in lexer {
            println!("{token:?}");
        }
    }
}
