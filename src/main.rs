use std::io::{self, Write};

mod builtins;

fn main() {
    println!("rsh - A minimalist shell for Serix kernel");
    println!("Type 'help' for available commands\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let command = tokens[0];
        let args = &tokens[1..];

        if !builtins::execute(command, args) {
            println!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                command
            );
        }
    }
}
