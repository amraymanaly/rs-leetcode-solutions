// we want to find the length of the longest substring without repeating character LSWRC in a string

pub fn length_of_longest_substring(s: String) -> i32 {
    // the idea is as follows:
    // if i know (LSWRC overall, LSWRC including the last character) in a string
    // then it's trivial to compute it again for that string concatenated with one character

    // loop-invariant variables with 4 invariants
    // best overall (1)
    let mut overall = &"".to_string()[..];

    // best edgy (2)
    let mut edgy = overall.clone();

    // &s[edgy_starts_from..] == edgy (3)
    let mut edgy_starts_from = 0usize;

    // for all 0 <= i < 27 --> edgy_matrix[i] == edgy.find(the corresponding char) (4)
    let mut edgy_matrix: [Option<usize>; 178] = [None; 178];

    // helper variables
    let mut temp;

    for (mark, i) in s.as_bytes().iter().enumerate() {
        if let Some(mut pos_in_edgy) = edgy_matrix[*i as usize] {
            pos_in_edgy += 1;
            edgy_starts_from += pos_in_edgy;

            for c in edgy[..pos_in_edgy].as_bytes() {
                edgy_matrix[*c as usize] = None;
            }
            for c in edgy[pos_in_edgy..].as_bytes() {
                temp = edgy_matrix[*c as usize].unwrap();
                edgy_matrix[*c as usize] = Some(temp - pos_in_edgy);
            }
        }

        edgy = &s[edgy_starts_from..=mark];
        edgy_matrix[*i as usize] = Some(mark - edgy_starts_from);

        if edgy.len() > overall.len() {
            overall = edgy;
        }
    }

    overall.len() as i32
}
