#![deny(missing_docs)]
//! A simple in-memory key-value store.

use std::collections::HashMap;

/// A simple key-value store.
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new, empty `KvStore`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        KvStore::default()
    }

    /// Sets the value for a given key in the store.
    ///
    /// If the key already exists, the value will be updated.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key.
    /// * `value` - A `String` representing the value to be associated with the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Gets the value associated with the given key.
    ///
    /// Returns `Some(String)` if the key exists, otherwise returns `None`.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));
    /// assert_eq!(store.get("key2".to_string()), None);
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Removes the value associated with the given key.
    ///
    /// If the key does not exist, this does nothing.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// store.remove("key1".to_string());
    /// assert_eq!(store.get("key1".to_string()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
