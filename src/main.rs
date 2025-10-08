use std::io::{self, Write};

mod builtins;
mod state;
mod tokenizer;

use state::ShellState;

fn main() {
    println!("rsh - A minimalist shell for Serix kernel");
    println!("Type 'help' for available commands\n");

    let mut state = ShellState::new();

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

        // Add to history
        state.add_to_history(input.to_string());

        // Tokenize with variable substitution
        let tokens = tokenizer::tokenize(input, &state.variables);
        if tokens.is_empty() {
            continue;
        }

        let command = &tokens[0];
        let args = if tokens.len() > 1 {
            &tokens[1..]
        } else {
            &[]
        };

        if !builtins::execute(command, args, &mut state) {
            println!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                command
            );
        }
    }
}
