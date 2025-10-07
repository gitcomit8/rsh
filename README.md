# rsh
rsh is a minimalist, extensible shell written in Rust designed to integrate seamlessly with the Serix kernel for early kernel interaction and user command execution.

## Features

- **Minimal Dependencies**: Built using only Rust's standard library
- **REPL Loop**: Interactive command-line interface
- **Whitespace Tokenization**: Simple and efficient command parsing
- **Modular Design**: Easy to extend with new commands and features
- **Built-in Commands**: Essential commands for shell interaction

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/rsh
```

## Built-in Commands

- **help** - Display available commands and their descriptions
- **echo [args...]** - Echo arguments to the screen
- **clear** - Clear the terminal screen
- **exit [code]** - Exit the shell with optional exit code

## Usage Examples

```
> help
rsh - Available commands:
  help  - Display this help message
  echo  - Echo arguments to the screen
  clear - Clear the terminal screen
  exit  - Exit the shell

> echo Hello, World!
Hello, World!

> exit
```

## Design

The shell is designed with modularity in mind:
- `main.rs` - REPL loop and command dispatching
- `builtins.rs` - Built-in command implementations

This structure makes it easy to add new built-in commands or extend functionality for scripting features in the future.

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.
