#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        if trimmed_input == "exit 0" {
            std::process::exit(0);
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
