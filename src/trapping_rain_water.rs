use std::cmp::min;
impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        // idea: i see a bar of height h, i fix on it, and move forward
        // till i meet a bar of height >= h. if i do, i add the distance
        // between them to the count. then i return to the bar right next
        // to the one i fixed on. This only works when bar heights are
        // in non-decreasing order... so, I'll reduce it to it!
        let c = water(height.iter());

        let h = &height[c.1..];
        c.0 + water(h.iter().rev()).0
    }
}
fn water<'a>(mut height: impl Iterator<Item = &'a i32>) -> (i32, usize) {
    let mut count = 0;
    let mut fix = None; // (index, height)
    let mut blocked = 0;
    let mut i = 0;
    while let Some(&current_height) = height.next() {
        if fix.is_none() && current_height > 0 {
            // initially
            fix = Some((i, current_height));
        } else if let Some((idx, h)) = fix {
            if current_height >= h {
                if i - idx > 1 {
                    let diff = i - idx - 1;
                    count += diff * h as usize - blocked;
                    fix = Some((i, current_height));
                    blocked = 0;
                } else {
                    fix = Some((i, current_height));
                }
            } else {
                blocked += min(current_height, h) as usize;
            }
        }
        i += 1;
    }
    let k = fix.unwrap_or((0, 0)).0;
    (count as i32, k)
}
