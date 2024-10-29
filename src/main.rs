use std::collections::HashSet;
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    let tokens: Vec<&str> = command.split(" ").collect();

    match tokens[..] {
        ["exit", code] => handle_exit_with_code(code),
        ["echo", ..] => println!("{}", tokens[1..].join(" ")),
        ["type", cmd] => handle_type_command(cmd),
        _ => println!("{}: command not found", command),
    }
}

fn handle_type_command(command: &str) {
    let shell_builtin_commands = HashSet::from(["echo", "exit", "type"]);
    let path_env = env::var("PATH").unwrap();

    if shell_builtin_commands.contains(command) {
        println!("{} is a shell builtin", command);
    } else {
        let splits = &mut path_env.split(":");

        if let Some(path) =
            splits.find(|path| fs::metadata(format!("{}/{}", path, command)).is_ok())
        {
            println!("{} is {}/{}", command, path, command);
        } else {
            println!("{}: not found", command)
        }
    }
}

fn handle_exit_with_code(code: &str) {
    let code = code.parse::<i32>().unwrap();
    exit(code);
}
