use std::collections::HashMap;

pub fn can_jump(nums: Vec<i32>) -> bool {
    S::new(nums)
}

struct S {
    nums: Vec<i32>,
    does_it: HashMap<usize, CanItReach>,
}

#[derive(Copy, Clone)]
enum CanItReach {
    Decided(bool),
    Computing,
}

impl S {
    pub fn new(nums: Vec<i32>) -> bool {
        S {
            nums,
            does_it: HashMap::new(),
        }
        .does_it_reach_last(0)
    }

    pub fn does_it_reach_last(&mut self, index: usize) -> bool {
        let max_jumps = self.nums[index];
        let b = if index == self.nums.len() - 1 {
            true
        } else if max_jumps == 0 {
            false
        } else {
            // println!("jumping!");
            self.does_it.insert(index, CanItReach::Computing);
            (1..=max_jumps).any(|jump| {
                vec![index as i32 - jump, index as i32 + jump]
                    .iter()
                    .any(|&p| {
                        // println!("trying {p}");
                        // std::thread::sleep(std::time::Duration::from_secs(1));
                        if p < 0 || p as usize >= self.nums.len() {
                            false
                        } else {
                            match self.does_it.get(&(p as usize)) {
                                Some(&b) => match b {
                                    CanItReach::Decided(ans) => ans,
                                    CanItReach::Computing => false, // to avoid infinite recursion
                                },
                                None => self.does_it_reach_last(p as usize),
                            }
                        }
                    })
            })
        };

        // println!("index {index} says {b}");

        self.does_it.insert(index, CanItReach::Decided(b));
        b
    }
}
