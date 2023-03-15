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

    // state variables
    let mut max_so_far = 0;

    // loop variables
    let mut local_max = 0;
    let l = heights.len();

    for i in 1..l {
        for j in 0..i {
            local_max = max(local_max, (i - j) as i32 * min(heights[i], heights[j]));
        }
        // println!("local_max is {local_max}");
        max_so_far = max(max_so_far, local_max);
        local_max = 0;
    }

    max_so_far
}
