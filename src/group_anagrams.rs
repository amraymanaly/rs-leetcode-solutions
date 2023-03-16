use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut letters: HashMap<usize, Vec<usize>> = HashMap::with_capacity(28 * strs.len() / 10);

    for (i, word) in strs.iter().enumerate() {
        let hash = word
            .chars()
            .fold(0, |acc, c| acc + (1 << ((c as u8) - ('a' as u8))));

        if let Some(v) = letters.get_mut(&hash) {
            v.push(i);
        } else {
            letters.insert(hash, vec![i]);
        }
    }

    // println!("map is {:?}", letters);

    letters
        .values()
        .map(|vals| vals.iter().map(|i| strs[*i].clone()).collect())
        .collect()
}
