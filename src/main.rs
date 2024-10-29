use std::collections::HashSet;
use std::{
    env, fs,
    io::{self, Write},
    process::{exit, Command},
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    let tokens: Vec<&str> = command.split(" ").collect();

    match tokens[..] {
        ["pwd"] if tokens.len() == 1 => handle_pwd_command(command),
        ["pwd", ..] => command_not_found(command),
        ["exit", code] => handle_exit_with_code(code),
        ["echo", ..] => println!("{}", tokens[1..].join(" ")),
        ["type", cmd] => handle_type_command(cmd),
        _ => handle_external_run(command),
    }
}

fn handle_type_command(command: &str) {
    let shell_builtin_commands = HashSet::from(["echo", "exit", "type", "pwd"]);
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
            command_not_found(command)
        }
    }
}

fn handle_external_run(command: &str) {
    let commands: Vec<&str> = command.split(" ").collect();

    if commands.is_empty() {
        return;
    }

    let result = if commands.len() > 1 {
        Command::new(commands[0])
            .args(commands[1..].iter())
            .output()
    } else {
        Command::new(commands[0]).output()
    };

    match result {
        Ok(executable) => {
            let output = String::from_utf8_lossy(&executable.stdout);
            println!("{}", output.trim_end());
        }
        Err(_) => command_not_found(command),
    }
}

fn handle_pwd_command(command: &str) {
    let directory = env::current_dir();

    match directory {
        Ok(result) => println!("{}", result.display()),
        Err(_) => command_not_found(command),
    }
}

fn handle_exit_with_code(code: &str) {
    let code = code.parse::<i32>().unwrap();
    exit(code);
}

fn command_not_found(command: &str) {
    println!("{}: not found", command);
}
