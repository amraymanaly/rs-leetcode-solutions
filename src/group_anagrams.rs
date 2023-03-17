use std::collections::HashMap;

// so here's the strategy...
// we want some way to assign a unique identity to each string,
// but an identity that disregards the order of letters in it.
// it's a lossy hash, if you will.

// idea: the hash must be built up using a commutative and an associative
// operation (associativity here recognized as "iterative commutativity")

// first idea: each letter is a bit position.
//      e.g.: bit => b, i, t => 1, 7, 19 => 10000000000010000010
// problem: it assumes that each letter can occur only once.
// that's because two b's, for e.g., make a c, in this scheme.

// second idea: if two letters can evaluate to one, then make the hash be
// that of the first idea + string length.
// problem: two letters from string 1 can cancel one letter from string 2,
// and vice versa, making this substitution idempotent to string length.
//      e.g.: tit, huh => two h's cancel i, two t's cancel u

// third idea: on inspecting this last problem, one finds that the malicious
// letters in string 1 will have to be of the form: x, x, y + 1
// and those in string 2: y, y, x + 1
// those can be caught with a simple sum!
// note that nothing can cancel out any trickery this time, because
// repeitions of this "diamond" cannot be idempotent to the simple sum.

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut letters: HashMap<(usize, usize, usize), Vec<usize>> =
        HashMap::with_capacity(28 * strs.len() / 10);

    for (i, word) in strs.iter().enumerate() {
        let hash = word
            .chars()
            .fold(0, |acc, c| acc + (1 << ((c as u8) - ('a' as u8))));

        let len = word.len();
        let sum: usize = word.bytes().map(|x| x as usize).sum();

        if let Some(v) = letters.get_mut(&(hash, len, sum)) {
            v.push(i);
        } else {
            letters.insert((hash, len, sum), vec![i]);
        }
    }

    letters
        .values()
        .map(|vals| vals.iter().map(|i| strs[*i].clone()).collect())
        .collect()
}
