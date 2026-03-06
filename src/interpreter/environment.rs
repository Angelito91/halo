// The Halo Programming Language
// Environment for variable scoping
// Version: 0.1.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::value::Value;
use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        scopes.push(HashMap::new()); // Global scope
        Environment { scopes }
    }

    /// Get value of a variable (searches from innermost to outermost scope)
    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    /// Set value in current scope
    pub fn set(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Update existing variable (in the scope where it was defined)
    pub fn update(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        // If variable doesn't exist, create it in current scope
        self.set(name.to_string(), value);
        Ok(())
    }

    /// Create a new scope (for function calls, blocks)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exit a scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Get current scope depth
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(42));
        assert_eq!(env.get("x"), Some(Value::Number(42)));
    }

    #[test]
    fn test_undefined_variable() {
        let env = Environment::new();
        assert_eq!(env.get("undefined"), None);
    }

    #[test]
    fn test_scope_isolation() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(10));

        env.push_scope();
        env.set("x".to_string(), Value::Number(20));
        assert_eq!(env.get("x"), Some(Value::Number(20)));

        env.pop_scope();
        assert_eq!(env.get("x"), Some(Value::Number(10)));
    }

    #[test]
    fn test_nested_scopes() {
        let mut env = Environment::new();
        env.set("a".to_string(), Value::Number(1));

        env.push_scope();
        env.set("b".to_string(), Value::Number(2));
        assert_eq!(env.get("a"), Some(Value::Number(1)));
        assert_eq!(env.get("b"), Some(Value::Number(2)));

        env.push_scope();
        env.set("c".to_string(), Value::Number(3));
        assert_eq!(env.get("a"), Some(Value::Number(1)));
        assert_eq!(env.get("b"), Some(Value::Number(2)));
        assert_eq!(env.get("c"), Some(Value::Number(3)));

        env.pop_scope();
        assert_eq!(env.get("c"), None);
    }

    #[test]
    fn test_update_existing() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(10));
        env.update("x", Value::Number(20)).unwrap();
        assert_eq!(env.get("x"), Some(Value::Number(20)));
    }

    #[test]
    fn test_update_creates_if_not_exists() {
        let mut env = Environment::new();
        env.update("x", Value::Number(42)).unwrap();
        assert_eq!(env.get("x"), Some(Value::Number(42)));
    }

    #[test]
    fn test_scope_depth() {
        let mut env = Environment::new();
        assert_eq!(env.depth(), 1);

        env.push_scope();
        assert_eq!(env.depth(), 2);

        env.push_scope();
        assert_eq!(env.depth(), 3);

        env.pop_scope();
        assert_eq!(env.depth(), 2);
    }
}
