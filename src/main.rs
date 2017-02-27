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
    println!("testing: cons");
    let testatom = sexpr::Sexpr::<i32, i32>::new(12, 3, true);
    let testatom2 = sexpr::Sexpr::<i32, i32>::new(11, 4, true);
    println!("(cons {} {}) = {}", testatom, testatom2, sexpr::cons(&testatom, &testatom2));


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
