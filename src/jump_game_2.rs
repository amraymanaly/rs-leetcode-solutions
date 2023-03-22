pub fn jump(nums: Vec<i32>) -> i32 {
    let mut jumps = 0;
    if nums.len() == 1 {
        return 0;
    }
    let mut pos: (usize, &i32) = (0, &nums[0]);
    while nums.len() > *pos.1 as usize + 1 + pos.0 {
        let p = pos.0;
        pos = max_in(&nums[pos.0 + 1..=pos.0 + *pos.1 as usize]);
        pos.0 += p + 1;
        jumps += 1;
    }

    jumps + 1
}

fn max_in(nums: &[i32]) -> (usize, &i32) {
    nums.iter()
        .enumerate()
        .min_by_key(|&x| nums.len() as isize - x.0 as isize - *x.1 as isize)
        .unwrap()
}
