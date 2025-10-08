use std::collections::HashMap;

pub fn tokenize(input: &str, variables: &HashMap<String, String>) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if escape_next {
            // Handle escape sequences
            match ch {
                'n' => current_token.push('\n'),
                't' => current_token.push('\t'),
                'r' => current_token.push('\r'),
                '\\' => current_token.push('\\'),
                '"' => current_token.push('"'),
                '$' => current_token.push('$'),
                _ => {
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
            // Variable substitution (only outside quotes)
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
            }
            continue;
        }

        if ch == '$' && in_quotes {
            // Inside quotes, keep the $ and variable name as-is
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenization() {
        let vars = HashMap::new();
        let result = tokenize("echo hello world", &vars);
        assert_eq!(result, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn test_quoted_strings() {
        let vars = HashMap::new();
        let result = tokenize("echo \"hello world\"", &vars);
        assert_eq!(result, vec!["echo", "hello world"]);
    }

    #[test]
    fn test_variable_substitution() {
        let mut vars = HashMap::new();
        vars.insert("NAME".to_string(), "Alice".to_string());
        let result = tokenize("echo $NAME", &vars);
        assert_eq!(result, vec!["echo", "Alice"]);
    }

    #[test]
    fn test_escape_characters() {
        let vars = HashMap::new();
        let result = tokenize("echo \\\"hello\\\"", &vars);
        assert_eq!(result, vec!["echo", "\"hello\""]);
    }

    #[test]
    fn test_mixed_quotes_and_variables() {
        let mut vars = HashMap::new();
        vars.insert("USER".to_string(), "Bob".to_string());
        let result = tokenize("echo \"Hello $USER\"", &vars);
        assert_eq!(result, vec!["echo", "Hello $USER"]);
    }

    #[test]
    fn test_escape_in_variable() {
        let vars = HashMap::new();
        let result = tokenize("echo \\$VAR", &vars);
        assert_eq!(result, vec!["echo", "$VAR"]);
    }
}
