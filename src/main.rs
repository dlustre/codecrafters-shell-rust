#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while let Ok(_num_bytes) = stdin.read_line(&mut input) {
        let formatted = input.trim();
        println!("{formatted}: command not found");
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}
