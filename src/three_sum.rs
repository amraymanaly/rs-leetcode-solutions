pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    nums.sort();

    let mut begin;
    let mut end;
    let mut sum;
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        begin = i + 1;
        end = nums.len() - 1;
        while begin < end {
            sum = nums[begin] + nums[end] + nums[i];
            if sum == 0 {
                res.push(vec![nums[i], nums[begin], nums[end]]);
                begin += 1;
                while begin < end && nums[begin - 1] == nums[begin] {
                    begin += 1;
                }
            } else if sum < 0 {
                begin += 1;
            } else {
                end -= 1;
            }
        }
    }
    res
}
