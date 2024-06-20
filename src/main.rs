use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{exit, Command},
};

fn command_in_dir(dir: &Path, command: &str) -> Option<PathBuf> {
    if dir.is_dir() {
        for entry in dir.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if entry.file_name().to_str().unwrap() == command {
                    return Some(entry.path());
                }
            }
        }
    }
    return None;
}

fn command_in_dirs(dirs: Vec<&Path>, command: &str) -> Option<PathBuf> {
    for dir in dirs {
        if let Some(command_path) = command_in_dir(dir, command) {
            return Some(command_path);
        }
    }
    return None;
}

fn main() {
    let paths = option_env!("PATH")
        .unwrap_or("")
        .split(":")
        .map(|p| Path::new(p))
        .collect::<Vec<&Path>>();

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
                "cd" => match params[..] {
                    [path] => match env::set_current_dir(Path::new(path)) {
                        Ok(_) => {}
                        Err(_) => println!("{path}: No such file or directory"),
                    },
                    [..] => println!("too many arguments"),
                },
                "pwd" => println!("{}", env::current_dir().unwrap().to_str().unwrap()),
                "type" => {
                    for param in params {
                        match *param {
                            "pwd" | "type" | "exit" | "echo" => {
                                println!("{param} is a shell builtin")
                            }
                            _ => {
                                if let Some(command_path) = command_in_dirs(paths.to_owned(), param)
                                {
                                    println!(
                                        "{param} is {}",
                                        command_path.to_str().unwrap_or_default()
                                    );
                                } else {
                                    println!("{param}: not found");
                                }
                            }
                        }
                    }
                }
                "exit" => match params[..] {
                    [maybe_code, ..] => match maybe_code.parse::<i32>() {
                        Ok(code) => exit(code),
                        Err(_) => println!("Invalid exit code"),
                    },
                    [] => println!("No error code provided"),
                },
                "echo" => println!("{}", params.join(" ")),
                command => {
                    if let Some(command_path) = command_in_dirs(paths.to_owned(), command) {
                        let output = Command::new(command_path)
                            .args(params)
                            .output()
                            .expect("failed to execute process");
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    } else {
                        println!("{command}: not found");
                    }
                }
            }
        }
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}
