/*
 * builtins.rs - Built-in shell command implementations
 *
 * execute() is the dispatch table. It matches the command string against
 * all known builtins and returns true if one matched, false otherwise.
 * The REPL in main.rs prints "Unknown command" when false is returned.
 *
 * Available commands:
 *   help      Print this command list
 *   echo      Print arguments, with $VAR expansion on each arg
 *   exit      Exit the shell with an optional status code
 *   clear     Send ANSI clear-screen escape sequence
 *   set       Set a shell variable: set VAR VALUE...
 *   get       Print a shell variable: get VAR
 *   if        Conditional: if COND CMD [ARGS...] — runs CMD if COND is truthy
 *   repeat    Loop: repeat N CMD [ARGS...] — runs CMD N times
 *   history   Print numbered command history
 */

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use crate::state::ShellState;

/*
 * execute - Dispatch a command to its builtin handler
 * @command: First token from the tokenizer (the command name)
 * @args:    Remaining tokens
 * @state:   Mutable shell state (variables, history)
 *
 * Return: true if the command was recognised and handled, false otherwise.
 */
pub fn execute(command: &str, args: &[String], state: &mut ShellState) -> bool {
	match command {
		"help"    => { cmd_help();             true }
		"echo"    => { cmd_echo(args, state);  true }
		"exit"    => { cmd_exit(args);         true }
		"clear"   => { cmd_clear();            true }
		"set"     => { cmd_set(args, state);   true }
		"get"     => { cmd_get(args, state);   true }
		"if"      => { cmd_if(args, state);    true }
		"repeat"  => { cmd_repeat(args, state); true }
		"history" => { cmd_history(state);     true }
		_         => false,
	}
}

/* cmd_help - Print the list of available commands */
fn cmd_help() {
	ulib::println!("rsh - Available commands:");
	ulib::println!("  help              - Display this help message");
	ulib::println!("  echo [args...]    - Echo arguments to the screen");
	ulib::println!("  clear             - Clear the terminal screen");
	ulib::println!("  exit [code]       - Exit the shell");
	ulib::println!("  set VAR VALUE     - Set a shell variable");
	ulib::println!("  get VAR           - Get the value of a shell variable");
	ulib::println!("  if COND CMD       - Execute command if condition is true (non-empty)");
	ulib::println!("  repeat N CMD      - Repeat a command N times");
	ulib::println!("  history           - Show command history");
}

/*
 * cmd_echo - Print arguments separated by spaces
 *
 * Each argument is checked for a leading $: if so, the remainder is treated
 * as a variable name and expanded from shell state. This provides a second
 * pass of variable substitution on top of what the tokenizer already did,
 * for cases where the variable value itself starts with $.
 */
fn cmd_echo(args: &[String], state: &ShellState) {
	let output: Vec<String> = args.iter().map(|arg| {
		if arg.starts_with('$') && arg.len() > 1 {
			let var_name = &arg[1..];
			state.get_variable(var_name).cloned().unwrap_or_else(|| arg.clone())
		} else {
			arg.clone()
		}
	}).collect();
	ulib::println!("{}", output.join(" "));
}

/*
 * cmd_exit - Exit the shell with an optional integer status code
 *
 * Usage: exit [code]
 * If no code is given, exits with 0.
 */
fn cmd_exit(args: &[String]) {
	let code = if args.is_empty() {
		0
	} else {
		args[0].parse::<i32>().unwrap_or(0)
	};
	ulib::exit(code);
}

/*
 * cmd_clear - Clear the framebuffer/terminal screen
 *
 * Sends the standard ANSI escape sequence. Whether this is visible depends
 * on whether the framebuffer console interprets ANSI codes.
 */
fn cmd_clear() {
	ulib::print!("\x1B[2J\x1B[1;1H");
}

/*
 * cmd_set - Set a shell variable
 *
 * Usage: set VAR VALUE [VALUE...]
 * All tokens after VAR are joined with spaces and stored as the value.
 */
fn cmd_set(args: &[String], state: &mut ShellState) {
	if args.len() < 2 {
		ulib::println!("Usage: set VAR VALUE");
		return;
	}
	let var_name = args[0].clone();
	let value = args[1..].join(" ");
	state.set_variable(var_name, value);
}

/*
 * cmd_get - Print the value of a shell variable
 *
 * Usage: get VAR
 */
fn cmd_get(args: &[String], state: &ShellState) {
	if args.is_empty() {
		ulib::println!("Usage: get VAR");
		return;
	}
	match state.get_variable(&args[0]) {
		Some(value) => ulib::println!("{}", value),
		None        => ulib::println!("Variable '{}' not found", args[0]),
	}
}

/*
 * cmd_if - Conditional command execution
 *
 * Usage: if CONDITION COMMAND [ARGS...]
 *
 * CONDITION is truthy if:
 *   - It starts with $ and the named variable is non-empty, OR
 *   - It is a non-empty string other than "0" or "false"
 */
fn cmd_if(args: &[String], state: &mut ShellState) {
	if args.len() < 2 {
		ulib::println!("Usage: if CONDITION COMMAND [ARGS...]");
		return;
	}

	let condition = &args[0];
	let condition_result = if condition.starts_with('$') && condition.len() > 1 {
		let var_name = &condition[1..];
		state.get_variable(var_name).map(|v| !v.is_empty()).unwrap_or(false)
	} else {
		!condition.is_empty() && condition != "0" && condition != "false"
	};

	if condition_result {
		let cmd_args = if args.len() > 2 { &args[2..] } else { &[] };
		execute(&args[1], cmd_args, state);
	}
}

/*
 * cmd_repeat - Execute a command N times
 *
 * Usage: repeat N COMMAND [ARGS...]
 */
fn cmd_repeat(args: &[String], state: &mut ShellState) {
	if args.len() < 2 {
		ulib::println!("Usage: repeat N COMMAND [ARGS...]");
		return;
	}

	let n = match args[0].parse::<usize>() {
		Ok(num) => num,
		Err(_) => {
			ulib::println!("Error: '{}' is not a valid number", args[0]);
			return;
		}
	};

	let cmd_args = if args.len() > 2 { &args[2..] } else { &[] };
	for _ in 0..n {
		execute(&args[1], cmd_args, state);
	}
}

/*
 * cmd_history - Print numbered command history
 *
 * Entries are printed as "   N  command" where N is 1-based.
 */
fn cmd_history(state: &ShellState) {
	let history = state.get_history();
	if history.is_empty() {
		ulib::println!("No history available");
		return;
	}
	for (i, cmd) in history.iter().enumerate() {
		ulib::println!("{:4}  {}", i + 1, cmd);
	}
}
