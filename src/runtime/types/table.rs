//! Module containing a currently-unused [`Table`] type.
//!
//! This type is planned to be used to add support for complex user-defined
//! data structures, OOP, operator overloading, and more.

use std::collections::HashMap;

use super::object::Object;

/// Currently unused. See the [module](self) documentation for more information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    inner: HashMap<String, Object>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        self.inner.get(key)
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.inner.insert(key, value);
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
