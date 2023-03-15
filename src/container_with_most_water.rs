use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    find_max_area(&height)
}

fn find_max_area(heights: &[i32]) -> i32 {
    // a recursive solution
    // max_area[xs:x] = max(max_area[xs], max{
    //      for each (i, y) in xs.reverse().enumerate() {
    //          (i+1) * min(x, y)
    //      }
    // })
    match heights.len() {
        0 | 1 => 0,
        l => {
            let x = heights[l - 1];
            let mut m = 0;
            for i in 0..l - 1 {
                m = max(m, (l - i - 1) as i32 * min(x, heights[i]));
            }
            m
        }
    }
}
