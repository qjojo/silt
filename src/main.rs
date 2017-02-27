use std::io;
use std::io::Write;
#[macro_use] extern crate text_io;

mod sexpr;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("silt REPL {}", VERSION);

    //Entering main REPL
    let mut buffer = String::new();
    let mut exit = false;

    while !exit  {
        print!("> ");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut buffer).ok().expect("Could not read line");
        buffer.pop();
        buffer.trim();
        if buffer == "exit" {
            exit = true;
        }
        let input = sexpr::read(buffer);
        println!("{}", input);
        buffer = String::new();
    }
    println!("Goodbye!");
}
