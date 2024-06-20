use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while let Ok(_num_bytes) = stdin.read_line(&mut input) {
        match input.trim().split_whitespace().collect::<Vec<&str>>()[..] {
            ["exit", code] => exit(code.parse::<i32>().unwrap()),
            [unknown_command, ..] => println!("{unknown_command}: command not found"),
            _ => {}
        }
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}
