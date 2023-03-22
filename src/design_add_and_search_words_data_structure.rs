use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;

struct WordDictionary {
    words: HashSet<MyString>,
}

#[derive(Hash)]
struct MyString {
    string: String,
}

impl PartialEq for MyString {
    fn eq(&self, other: &Self) -> bool {
        if self.string.len() != self.string.len() {
            return false;
        }
        self.string
            .chars()
            .zip(other.string.chars())
            .all(|(a, b)| a == b || a == '.' || b == '.')
    }
}

impl Eq for MyString {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            words: HashSet::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.words.insert(MyString { string: word });
    }

    fn search(&self, word: String) -> bool {
        self.words.get(&MyString { string: word }).is_some()
    }
}
