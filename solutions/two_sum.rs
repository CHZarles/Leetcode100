use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();
        let mut result: Vec<i32> = Vec::new();

        for (index, &value) in nums.iter().enumerate() {
            let complement = target - value;

            if let Some(&complement_index) = map.get(&complement) {
                result.push(complement_index as i32);
                result.push(index as i32);
                break;
            }

            map.insert(value, index);
        }

        result
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let indices = Solution::two_sum(nums, target);
    println!("Indices: {:?}", indices);
}
