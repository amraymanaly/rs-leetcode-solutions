use std::collections::{HashMap, HashSet};

pub fn longest_palindrome(s: String) -> String {
    // idea: longest[uptill i] = longest[uptill i-1] or palindrome with char at i in the end
    let cs: Vec<char> = s.chars().collect();

    let mut palindromes = HashSet::with_capacity(s.len());
    let mut letters: HashMap<char, Vec<usize>> = HashMap::with_capacity(s.len());

    let is_palindromic = |start, end| {
        let (mut a, mut b) = (start, end);
        while a < b {
            if palindromes.contains(&(a, b)) {
                break;
            }
        }
        



        if start == end || palindromes.contains(&(start, end)) {
            return true;
        }
        palindromes.insert((start, end));
        true
    };

    let mut longest = &s[0..1];

    for (i, c) in s.chars().enumerate() {
        if let Some(v) = letters.get_mut(&c) {
            v.push(i);
        } else {
            letters.insert(c, vec![i]);
        }
        // find the longest palindrome with c at the end
        for &j in letters.get(&c).unwrap() {
            if is_palindromic(j, i)
        }
    }

    "".into()
}
