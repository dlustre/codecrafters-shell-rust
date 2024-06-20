use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut input = String::new();
    while let Ok(_num_bytes) = stdin.read_line(&mut input) {
        if let Some((command, params)) = input
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .split_first()
        {
            match *command {
                "type" => {
                    for param in params {
                        match *param {
                            "type" | "exit" | "echo" => println!("{param} is a shell builtin"),
                            _ => println!("{param}: not found"),
                        }
                    }
                }
                "exit" => match params[..] {
                    [code, ..] => match code.parse::<i32>() {
                        Ok(code) => exit(code),
                        Err(_) => println!("Invalid exit code"),
                    },
                    [] => println!("No error code provided"),
                },
                "echo" => println!("{}", params.join(" ")),
                _ => println!("{command}: not found"),
            }
        }
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}
