mod lexer;
mod repl;

fn main() {
    if let Err(err) = repl::run() {
        eprintln!("REPL Error: {}", err);
    }
}
