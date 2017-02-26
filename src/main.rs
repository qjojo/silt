use std::io;
use std::io::Write;

const VERSION: &'static str = "0.1.0";

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
        println!("{}", buffer);
        if buffer == "exit" {
            exit = true;
        }
        buffer = String::new();
    }
    println!("Goodbye!");
}
