use std::process;
use crate::state::ShellState;

pub fn execute(command: &str, args: &[String], state: &mut ShellState) -> bool {
    match command {
        "help" => {
            cmd_help();
            true
        }
        "echo" => {
            cmd_echo(args, state);
            true
        }
        "exit" => {
            cmd_exit(args);
            true
        }
        "clear" => {
            cmd_clear();
            true
        }
        "set" => {
            cmd_set(args, state);
            true
        }
        "get" => {
            cmd_get(args, state);
            true
        }
        "if" => {
            cmd_if(args, state);
            true
        }
        "repeat" => {
            cmd_repeat(args, state);
            true
        }
        "history" => {
            cmd_history(state);
            true
        }
        _ => false,
    }
}

fn cmd_help() {
    println!("rsh - Available commands:");
    println!("  help              - Display this help message");
    println!("  echo [args...]    - Echo arguments to the screen");
    println!("  clear             - Clear the terminal screen");
    println!("  exit [code]       - Exit the shell");
    println!("  set VAR VALUE     - Set a shell variable");
    println!("  get VAR           - Get the value of a shell variable");
    println!("  if COND CMD       - Execute command if condition is true (non-empty)");
    println!("  repeat N CMD      - Repeat a command N times");
    println!("  history           - Show command history");
}

fn cmd_echo(args: &[String], state: &ShellState) {
    // Process each argument and expand variables if needed
    let output: Vec<String> = args.iter().map(|arg| {
        if arg.starts_with('$') && arg.len() > 1 {
            let var_name = &arg[1..];
            state.get_variable(var_name).cloned().unwrap_or_else(|| arg.clone())
        } else {
            arg.clone()
        }
    }).collect();
    println!("{}", output.join(" "));
}

fn cmd_exit(args: &[String]) {
    let code = if args.is_empty() {
        0
    } else {
        args[0].parse::<i32>().unwrap_or(0)
    };
    process::exit(code);
}

fn cmd_clear() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn cmd_set(args: &[String], state: &mut ShellState) {
    if args.len() < 2 {
        println!("Usage: set VAR VALUE");
        return;
    }
    let var_name = args[0].clone();
    let value = args[1..].join(" ");
    state.set_variable(var_name, value);
}

fn cmd_get(args: &[String], state: &ShellState) {
    if args.is_empty() {
        println!("Usage: get VAR");
        return;
    }
    let var_name = &args[0];
    match state.get_variable(var_name) {
        Some(value) => println!("{}", value),
        None => println!("Variable '{}' not found", var_name),
    }
}

fn cmd_if(args: &[String], state: &mut ShellState) {
    if args.len() < 2 {
        println!("Usage: if CONDITION COMMAND [ARGS...]");
        return;
    }
    
    let condition = &args[0];
    // Evaluate condition: non-empty string is true
    let condition_result = if condition.starts_with('$') && condition.len() > 1 {
        let var_name = &condition[1..];
        state.get_variable(var_name).map(|v| !v.is_empty()).unwrap_or(false)
    } else {
        !condition.is_empty() && condition != "0" && condition != "false"
    };
    
    if condition_result {
        let command = &args[1];
        let cmd_args = if args.len() > 2 {
            &args[2..]
        } else {
            &[]
        };
        execute(command, cmd_args, state);
    }
}

fn cmd_repeat(args: &[String], state: &mut ShellState) {
    if args.len() < 2 {
        println!("Usage: repeat N COMMAND [ARGS...]");
        return;
    }
    
    let n = match args[0].parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: '{}' is not a valid number", args[0]);
            return;
        }
    };
    
    let command = &args[1];
    let cmd_args = if args.len() > 2 {
        &args[2..]
    } else {
        &[]
    };
    
    for _ in 0..n {
        execute(command, cmd_args, state);
    }
}

fn cmd_history(state: &ShellState) {
    let history = state.get_history();
    if history.is_empty() {
        println!("No history available");
        return;
    }
    
    for (i, cmd) in history.iter().enumerate() {
        println!("{:4}  {}", i + 1, cmd);
    }
}

