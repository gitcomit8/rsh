/*
 * tokenizer.rs - Command-line tokenizer with variable expansion
 *
 * tokenize() splits an input string into a Vec<String> using a simple
 * character-by-character state machine. Supported features:
 *
 *   Whitespace splitting  Words are separated by spaces/tabs unless quoted.
 *   Double-quoted strings "hello world" becomes one token. Inside quotes,
 *                         $ is treated literally (no variable expansion).
 *   Escape sequences      Outside quotes: \n \t \r \\ \" \$
 *                         Unknown sequences preserve the backslash.
 *   Variable expansion    $VARNAME outside quotes is replaced by its value
 *                         from the variables map. Unset variables expand to
 *                         nothing (the empty string).
 */

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/*
 * tokenize - Split input into tokens with variable substitution
 * @input:     Raw input line (leading/trailing whitespace already trimmed)
 * @variables: Shell variable map used for $VAR expansion
 *
 * Return: Ordered list of token strings; empty if input is blank.
 */
pub fn tokenize(input: &str, variables: &BTreeMap<String, String>) -> Vec<String> {
	let mut tokens = Vec::new();
	let mut current_token = String::new();
	let mut in_quotes = false;
	let mut escape_next = false;
	let mut chars = input.chars().peekable();

	while let Some(ch) = chars.next() {
		if escape_next {
			match ch {
				'n'  => current_token.push('\n'),
				't'  => current_token.push('\t'),
				'r'  => current_token.push('\r'),
				'\\' => current_token.push('\\'),
				'"'  => current_token.push('"'),
				'$'  => current_token.push('$'),
				_    => {
					/* Unknown escape: preserve backslash and character */
					current_token.push('\\');
					current_token.push(ch);
				}
			}
			escape_next = false;
			continue;
		}

		if ch == '\\' {
			escape_next = true;
			continue;
		}

		if ch == '"' {
			in_quotes = !in_quotes;
			continue;
		}

		if ch == '$' && !in_quotes {
			/* Collect the variable name: alphanumeric + underscore */
			let mut var_name = String::new();
			while let Some(&next_ch) = chars.peek() {
				if next_ch.is_alphanumeric() || next_ch == '_' {
					var_name.push(chars.next().unwrap());
				} else {
					break;
				}
			}
			if !var_name.is_empty() {
				if let Some(value) = variables.get(&var_name) {
					current_token.push_str(value);
				}
				/* Unset variable expands to nothing */
			}
			continue;
		}

		if ch == '$' && in_quotes {
			/* Inside quotes: $ is literal */
			current_token.push(ch);
			continue;
		}

		if ch.is_whitespace() && !in_quotes {
			if !current_token.is_empty() {
				tokens.push(current_token.clone());
				current_token.clear();
			}
		} else {
			current_token.push(ch);
		}
	}

	if !current_token.is_empty() {
		tokens.push(current_token);
	}

	tokens
}
