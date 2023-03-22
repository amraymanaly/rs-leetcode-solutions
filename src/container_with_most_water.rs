pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;

    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        ans = ans.max(height[left].min(height[right]) * (right - left) as i32);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    ans
}
