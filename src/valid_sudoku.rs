pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut j;
    for (i, row) in board.iter().enumerate() {
        j = 0;
        if !check(row.iter()) {
            return false;
        }
        if !check(board.iter().map(|x| &x[i])) {
            return false;
        }
        if !check(board.iter().map(|_| {
            let c = &board[(i / 3) * 3 + j / 3][(i % 3) * 3 + j % 3];
            j += 1;
            c
        })) {
            return false;
        }
    }
    true
}

#[inline]
fn to(c: u8) -> usize {
    1 << (c - '1' as u8)
}

fn check<'a>(s: impl Iterator<Item = &'a char>) -> bool {
    const NINE_ONES: usize = 0x1ff;
    let mut wall = NINE_ONES;
    let mut prev = NINE_ONES;
    for &c in s {
        if c == '.' {
            continue;
        }
        // println!("found {c}");
        wall ^= to(c as u8);
        if wall > prev {
            // println!("sus");
            return false;
        }
        prev = wall;
    }
    // println!("alright");
    true
}
