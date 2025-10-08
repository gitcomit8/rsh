# rsh
rsh is a minimalist, extensible shell written in Rust designed to integrate seamlessly with the Serix kernel for early kernel interaction and user command execution.

## Features

- **Minimal Dependencies**: Built using only Rust's standard library
- **REPL Loop**: Interactive command-line interface
- **Advanced Tokenization**: Supports quoted strings, escape characters, and variable substitution
- **Scripting Support**: Variables, conditionals, loops, and command history
- **Modular Design**: Easy to extend with new commands and features
- **Built-in Commands**: Essential commands for shell interaction and scripting

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

### Basic Commands
- **help** - Display available commands and their descriptions
- **echo [args...]** - Echo arguments to the screen (supports variable expansion with $VAR)
- **clear** - Clear the terminal screen
- **exit [code]** - Exit the shell with optional exit code

### Scripting Commands
- **set VAR VALUE** - Set a shell variable
- **get VAR** - Get and display the value of a shell variable
- **if CONDITION COMMAND [ARGS...]** - Execute command if condition is true (non-empty)
- **repeat N COMMAND [ARGS...]** - Repeat a command N times
- **history** - Show command history

## Usage Examples

### Basic Usage
```
> help
rsh - Available commands:
  help              - Display this help message
  echo [args...]    - Echo arguments to the screen
  clear             - Clear the terminal screen
  exit [code]       - Exit the shell
  set VAR VALUE     - Set a shell variable
  get VAR           - Get the value of a shell variable
  if COND CMD       - Execute command if condition is true (non-empty)
  repeat N CMD      - Repeat a command N times
  history           - Show command history

> echo Hello, World!
Hello, World!
```

### Variable Management
```
> set NAME Alice
> get NAME
Alice

> echo $NAME
Alice

> echo "Literal: $NAME"
Literal: $NAME
```

### Escape Characters
```
> echo "Hello\nWorld"
Hello
World

> echo \$VAR
$VAR
```

### Conditionals and Loops
```
> set COUNT 3
> repeat $COUNT echo Hi
Hi
Hi
Hi

> if $NAME echo Found: $NAME
Found: Alice

> if 1 echo This runs
This runs
```

### Command History
```
> history
   1  set NAME Alice
   2  get NAME
   3  echo $NAME
   4  history
```

## Design

The shell is designed with modularity in mind:
- `main.rs` - REPL loop and command dispatching
- `builtins.rs` - Built-in command implementations
- `state.rs` - Shell state management (variables and history)
- `tokenizer.rs` - Advanced tokenization with quote handling, escape sequences, and variable substitution

This modular structure makes it easy to add new built-in commands or extend functionality with additional scripting features.

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.
