use std::ops::Index;

struct Solution {
    nums: Vec<i32>,
    target: i32,
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for (i, value) in nums.iter().enumerate() {
            let mut flag = 0;
            for (j, value2) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                if value + value2 == target {
                    ans.push(i as i32);
                    ans.push(j as i32);
                    flag = 1;
                    break;
                }
            }
            if flag == 1 {
                break;
            }
        }
        return ans;
    }
}

fn main() {
    let nums: Vec<i32> = vec![3, 2, 4];
    let target = 6;
    println!("output is {:?}", Solution::two_sum(nums, target));
}
