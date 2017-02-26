use std::io;
use std::io::Write;

mod sexpr;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("silt REPL {}", VERSION);

    //Entering main REPL
    let mut buffer = String::new();
    let mut exit = false;

    //debug statements
    print!("> ");
    io::stdout().flush().ok();
    println!("testing: atom function");
    let testatom = sexpr::Sexpr::<i32, i32>::new(12, 0, false);
    if testatom.is_atom() {
        println!("{} is an atom!", testatom);
    } else {
        println!("{} is an s-expression", testatom);
    }

    while !exit  {
        print!("> ");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut buffer).ok().expect("Could not read line");
        buffer.pop();
        buffer.trim();
        println!("{}", buffer);
        if buffer == "exit" {
            exit = true;
        }
        buffer = String::new();
    }
    println!("Goodbye!");
}
