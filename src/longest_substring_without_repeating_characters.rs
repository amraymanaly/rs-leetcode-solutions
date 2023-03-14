use std::cmp::max;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut idx_s = 0usize;
    let mut matrix: [Option<usize>; 178] = [None; 178];
    let mut answer = 0;
    let mut temp;

    for (idx, b) in s.as_bytes().iter().enumerate() {
        unsafe {
            temp = matrix.get_unchecked_mut(*b as usize);
        }
        if let Some(collision) = *temp {
            idx_s = max(collision + 1, idx_s);
        }

        answer = max(answer, idx - idx_s + 1);
        *temp = Some(idx);
    }

    answer as i32
}
