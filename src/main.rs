/*
 * main.rs - rsh entry point
 *
 * Serix bare-metal build: #![no_std], #![no_main].
 * The standard library is replaced by the ulib crate which provides
 * syscall wrappers, print!/println! macros, and read_line().
 *
 * Entry: _start() called directly by the kernel after ELF load and
 *        Ring 0 -> Ring 3 transition via iretq.
 *
 * The REPL loop:
 *   1. Print prompt "> "
 *   2. Read a line from stdin (blocking, with echo and backspace)
 *   3. Add to history
 *   4. Tokenize with variable substitution
 *   5. Dispatch first token to builtins::execute()
 *   6. Print "Unknown command" if no builtin matched
 */

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

mod builtins;
mod state;
mod tokenizer;

use state::ShellState;

/*
 * panic - Bare-metal panic handler
 *
 * Prints the panic message to stdout then exits the process with
 * status -1. This surfaces panics visibly on the framebuffer/serial
 * console rather than hanging silently.
 */
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	ulib::println!("[rsh] PANIC: {}", info);
	ulib::exit(-1);
}

/*
 * _start - rsh entry point
 *
 * Called by the kernel when the process is first scheduled.
 * Prints a welcome banner, then runs the interactive REPL until
 * the user calls `exit` or stdin is closed.
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	ulib::println!("rsh - Serix shell");
	ulib::println!("Type 'help' for available commands\n");

	let mut state = ShellState::new();

	loop {
		ulib::print!("> ");

		let line = match ulib::io::read_line() {
			Some(l) => l,
			None => {
				/* stdin closed — should not normally happen on Serix */
				ulib::println!("\n[rsh] stdin closed");
				ulib::exit(0);
			}
		};

		let input = line.trim();
		if input.is_empty() {
			continue;
		}

		state.add_to_history(alloc::string::String::from(input));

		let tokens = tokenizer::tokenize(input, &state.variables);
		if tokens.is_empty() {
			continue;
		}

		let command = &tokens[0];
		let args = if tokens.len() > 1 { &tokens[1..] } else { &[] };

		if !builtins::execute(command, args, &mut state) {
			ulib::println!(
				"Unknown command: '{}'. Type 'help' for available commands.",
				command
			);
		}
	}
}
