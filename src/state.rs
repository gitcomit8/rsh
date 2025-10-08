use std::collections::HashMap;

pub struct ShellState {
    pub variables: HashMap<String, String>,
    pub history: Vec<String>,
}

impl ShellState {
    pub fn new() -> Self {
        ShellState {
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn add_to_history(&mut self, command: String) {
        self.history.push(command);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = ShellState::new();
        assert_eq!(state.variables.len(), 0);
        assert_eq!(state.history.len(), 0);
    }

    #[test]
    fn test_set_and_get_variable() {
        let mut state = ShellState::new();
        state.set_variable("TEST".to_string(), "value".to_string());
        assert_eq!(state.get_variable("TEST"), Some(&"value".to_string()));
    }

    #[test]
    fn test_history() {
        let mut state = ShellState::new();
        state.add_to_history("cmd1".to_string());
        state.add_to_history("cmd2".to_string());
        assert_eq!(state.get_history().len(), 2);
        assert_eq!(state.get_history()[0], "cmd1");
    }
}
