use std::collections::HashMap;

const LOAD: usize = 1;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut letters: HashMap<(usize, usize), Vec<usize>> =
        HashMap::with_capacity(strs.len() / LOAD);

    for (i, word) in strs.iter().enumerate() {
        let hash: (usize, usize, usize) = word.bytes().fold((0, 0, 0), |acc, mut c| {
            c = c - 'a' as u8;
            (acc.0 + (1 << c), acc.1 + 1, acc.2 + c as usize)
            // (word hash [Idea #1], word length [Idea #2], word sum [Idea #3])
        });

        // to save space (drawing from the length limits in the problem statement)
        let hash = (hash.0, (hash.1 << 7) + hash.2);

        if let Some(v) = letters.get_mut(&hash) {
            v.push(i);
        } else {
            letters.insert(hash, vec![i]);
        }
    }

    letters
        .values()
        .map(|vals| vals.iter().map(|i| strs[*i].clone()).collect())
        .collect()
}
