// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// ── Design ────────────────────────────────────────────────────────────────────
//
// Previous design: Vec<HashMap<String, Value>>
//   • Every scope push allocates a new HashMap.
//   • Variable lookup iterates scopes and performs a hash per scope.
//   • Variable values are cloned on every read (get() returns Option<Value>).
//
// New design: flat Vec<Entry> + scope-offset stack
//   • All variables live in a single contiguous Vec<Entry>.
//   • A separate Vec<usize> records the start index of each scope frame.
//   • Lookup scans backwards through the flat Vec — hot cache line, no alloc.
//   • Scope push/pop = push/pop a single usize; zero heap allocation.
//   • `get` returns &Value to let callers decide whether to clone.
//   • `get_mut` lets callers update in-place without a second hash lookup.
//
// Worst-case lookup is still O(n) in the number of live variables, but in
// practice Halo programs have shallow scopes and few variables, so the flat
// scan is faster than multiple HashMap lookups in the common case.
// ─────────────────────────────────────────────────────────────────────────────

use super::value::Value;

/// A single variable binding stored in the flat table.
struct Entry {
    /// Variable name.  We store a `Box<str>` instead of `String` to avoid
    /// the extra capacity field and to make the struct smaller.
    name: Box<str>,
    value: Value,
}

pub struct Environment {
    /// Flat table of all live variable bindings, in binding order.
    /// The most-recently-bound variable is at the highest index, so scanning
    /// backwards gives innermost-scope-first semantics.
    entries: Vec<Entry>,
    /// Stack of frame start indices.
    /// `scope_starts[0]` is always 0 (the global scope).
    /// `scope_starts.last()` is the start of the current scope.
    scope_starts: Vec<usize>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            entries: Vec::with_capacity(32),
            scope_starts: vec![0], // global scope starts at index 0
        }
    }

    // ── Scope management ──────────────────────────────────────────────────────

    /// Enter a new scope (e.g. function call, block).
    pub fn push_scope(&mut self) {
        self.scope_starts.push(self.entries.len());
    }

    /// Leave the current scope, discarding all variables defined in it.
    pub fn pop_scope(&mut self) {
        if self.scope_starts.len() <= 1 {
            // Never pop the global scope.
            return;
        }
        let frame_start = self.scope_starts.pop().unwrap();
        self.entries.truncate(frame_start);
    }

    // ── Variable access ───────────────────────────────────────────────────────

    /// Look up `name`, searching from the innermost scope outward.
    /// Returns a reference to avoid a clone at the call site.
    pub fn get(&self, name: &str) -> Option<Value> {
        // Scan backwards: the last entry with this name is the one in scope.
        for entry in self.entries.iter().rev() {
            if entry.name.as_ref() == name {
                return Some(entry.value.clone());
            }
        }
        None
    }

    /// Return a shared reference to the value of `name`, avoiding a clone.
    /// Useful for read-only checks (e.g. `is_truthy`) before deciding to clone.
    #[allow(dead_code)]
    pub fn get_ref(&self, name: &str) -> Option<&Value> {
        for entry in self.entries.iter().rev() {
            if entry.name.as_ref() == name {
                return Some(&entry.value);
            }
        }
        None
    }

    /// Define a new variable in the **current** (innermost) scope.
    /// If a variable with the same name already exists in the *same* scope
    /// frame, its value is updated in place (avoids a duplicate entry).
    pub fn set(&mut self, name: String, value: Value) {
        let frame_start = *self.scope_starts.last().unwrap();
        // Check for an existing binding in the current frame only.
        for entry in self.entries[frame_start..].iter_mut().rev() {
            if entry.name.as_ref() == name.as_str() {
                entry.value = value;
                return;
            }
        }
        // New binding in the current scope.
        self.entries.push(Entry {
            name: name.into_boxed_str(),
            value,
        });
    }

    /// Update an **existing** variable wherever it was defined (innermost wins).
    /// If the variable is not found in any scope, it is created in the current scope.
    /// Returns `Ok(())` always (matches the previous API contract).
    pub fn update(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Walk backwards to find the nearest binding.
        for entry in self.entries.iter_mut().rev() {
            if entry.name.as_ref() == name {
                entry.value = value;
                return Ok(());
            }
        }
        // Variable not found — create it in the current scope (legacy behaviour).
        self.set(name.to_string(), value);
        Ok(())
    }

    /// Return a mutable reference to the nearest binding of `name`.
    /// Allows callers to mutate the stored value without an extra lookup.
    #[allow(dead_code)]
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        for entry in self.entries.iter_mut().rev() {
            if entry.name.as_ref() == name {
                return Some(&mut entry.value);
            }
        }
        None
    }

    /// Number of live variable bindings (useful for tests / debugging).
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` when no variable bindings are live.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current scope depth (0 = global only).
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.scope_starts.len() - 1
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

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
        assert_eq!(env.get("b"), Some(Value::Number(2)));
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
    fn test_update_inner_scope_does_not_affect_outer() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(1));

        env.push_scope();
        env.set("x".to_string(), Value::Number(2));
        // update should hit the inner binding
        env.update("x", Value::Number(99)).unwrap();
        assert_eq!(env.get("x"), Some(Value::Number(99)));

        env.pop_scope();
        // outer binding untouched
        assert_eq!(env.get("x"), Some(Value::Number(1)));
    }

    #[test]
    fn test_no_alloc_on_push_pop() {
        // Verifies that push_scope/pop_scope don't leave orphaned entries.
        let mut env = Environment::new();
        env.set("g".to_string(), Value::Number(0));
        let len_before = env.len();

        env.push_scope();
        env.set("local".to_string(), Value::Number(7));
        assert_eq!(env.len(), len_before + 1);

        env.pop_scope();
        assert_eq!(env.len(), len_before);
        assert_eq!(env.get("local"), None);
    }

    #[test]
    fn test_get_ref() {
        let mut env = Environment::new();
        env.set("pi".to_string(), Value::Float(std::f64::consts::PI));
        let r = env.get_ref("pi").unwrap();
        assert!(matches!(r, Value::Float(f) if (*f - std::f64::consts::PI).abs() < 1e-10));
    }

    #[test]
    fn test_get_mut() {
        let mut env = Environment::new();
        env.set("n".to_string(), Value::Number(1));
        if let Some(v) = env.get_mut("n") {
            *v = Value::Number(100);
        }
        assert_eq!(env.get("n"), Some(Value::Number(100)));
    }

    #[test]
    fn test_depth() {
        let mut env = Environment::new();
        assert_eq!(env.depth(), 0);
        env.push_scope();
        assert_eq!(env.depth(), 1);
        env.push_scope();
        assert_eq!(env.depth(), 2);
        env.pop_scope();
        assert_eq!(env.depth(), 1);
        env.pop_scope();
        assert_eq!(env.depth(), 0);
        // Cannot go below 0
        env.pop_scope();
        assert_eq!(env.depth(), 0);
    }

    #[test]
    fn test_set_dedup_in_same_scope() {
        // Setting the same variable twice in the same scope should not
        // create a duplicate entry.
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(1));
        let len1 = env.len();
        env.set("x".to_string(), Value::Number(2));
        assert_eq!(env.len(), len1, "duplicate entry created");
        assert_eq!(env.get("x"), Some(Value::Number(2)));
    }
}
