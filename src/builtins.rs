use std::process;

pub fn execute(command: &str, args: &[&str]) -> bool {
    match command {
        "help" => {
            cmd_help();
            true
        }
        "echo" => {
            cmd_echo(args);
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
        _ => false,
    }
}

fn cmd_help() {
    println!("rsh - Available commands:");
    println!("  help  - Display this help message");
    println!("  echo  - Echo arguments to the screen");
    println!("  clear - Clear the terminal screen");
    println!("  exit  - Exit the shell");
}

fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn cmd_exit(args: &[&str]) {
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
