use std::cmp::max;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut steps = 1;
    for n in nums {
        if steps == 0 {
            return false;
        }
        steps = max(steps - 1, n);
    }
    true
}
