/*
 * state.rs - Shell state: variables and command history
 *
 * ShellState is passed mutably through every builtin call.
 * Variables are stored in a BTreeMap (alloc::collections::BTreeMap
 * replaces std::collections::HashMap since we have no std hasher).
 * History is an ordered Vec of raw command strings.
 */

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShellState {
	pub variables: BTreeMap<String, String>,
	pub history: Vec<String>,
}

impl ShellState {
	pub fn new() -> Self {
		ShellState {
			variables: BTreeMap::new(),
			history: Vec::new(),
		}
	}

	/* set_variable - Insert or overwrite a shell variable */
	pub fn set_variable(&mut self, name: String, value: String) {
		self.variables.insert(name, value);
	}

	/* get_variable - Look up a shell variable by name */
	pub fn get_variable(&self, name: &str) -> Option<&String> {
		self.variables.get(name)
	}

	/* add_to_history - Append a command string to the history list */
	pub fn add_to_history(&mut self, command: String) {
		self.history.push(command);
	}

	/* get_history - Return a reference to the full history list */
	pub fn get_history(&self) -> &Vec<String> {
		&self.history
	}
}

impl Default for ShellState {
	fn default() -> Self {
		Self::new()
	}
}
