pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // FIXME: duplicates
    let mut solutions = vec![];
    // step 1: sort the numbers
    nums.sort();
    let nums = nums;

    // step 2: divide the numbers into three signed regions
    let mut negative = None; // index of last negative
    let mut positive = None; // index of first positive
    let mut zeros = 0; // number of zeros

    for (i, &num) in nums.iter().enumerate() {
        if num < 0 {
            negative = Some(i);
        } else if num == 0 {
            zeros += 1;
        } else if num > 0 {
            positive = Some(i);
            break;
        }
    }
    let (negative, positive, zeros) = (negative, positive, zeros);
    // step 3: solve it
    // step 3.1 => find all solutions with at least one zero
    // trivial zero triplets
    if zeros > 2 {
        // zero region
        solutions.push(vec![0, 0, 0]);
    }

    if positive.is_none() || negative.is_none() {
        return solutions;
    }

    let negatives = &nums[..=negative.unwrap()];
    let positives = &nums[positive.unwrap()..];

    if zeros > 0 {
        // all three regions
        // sets with exactly one zero in them
        for &pos in positives {
            if let Ok(_) = negatives.binary_search(&(-pos)) {
                solutions.push(vec![-pos, 0, pos]);
            }
        }
    }

    // two regions
    // step 3.2 => find solutions with one +ve, two -ves
    find_two_which_sum_to(&negatives, &positives, &mut solutions);
    // step 3.3 => find solutions with one -ve, two +ves
    find_two_which_sum_to(&positives, &negatives, &mut solutions);

    solutions
}

fn find_two_which_sum_to(nums: &[i32], targets: &[i32], solutions: &mut Vec<Vec<i32>>) {
    // FIXME: faulty
    if nums.len() < 2 {
        return;
    }
    // both nums and targets are sorted; a two-pointer approach
    let mut begin = 0;
    let mut end = nums.len() - 1;
    let mut sum;

    for &target in targets {
        if begin == end {
            break;
        }
        sum = nums[begin] + nums[end];
        if sum == -target {
            solutions.push(vec![target, nums[begin], nums[end]]);
            break; // bec. numbers are all distinct
        }
        if sum < target {
            begin += 1;
        } else {
            end -= 1;
        }
    }
}
