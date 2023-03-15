use std::cmp::Ordering;

#[allow(unused_must_use)]
pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }
    // binary search!
    let smallest_string = strs.iter().enumerate().min_by_key(|l| l.1.len()).unwrap();
    let smallest_string = strs.remove(smallest_string.0);
    let mut idx = None;
    (1..=smallest_string.len())
        .collect::<Vec<usize>>()
        .binary_search_by(|&c| {
            if strs.iter().all(|s| s[..c] == smallest_string[..c]) {
                idx = Some(c - 1);
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    if let Some(idx) = idx {
        smallest_string[..=idx].into()
    } else {
        "".into()
    }
}
