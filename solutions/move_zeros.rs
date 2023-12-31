struct Solution;

impl Solution {
    // solution 1
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // 用 range 0..nums.len() 填充零值
        let len = nums.len();
        nums.retain(|&value| value != 0);
        while nums.len() < len {
            nums.push(0);
        }
    }
    // solution 2
    pub fn move_zeroes1(nums: &mut Vec<i32>) {
        let mut last_non_zero = 0;

        // 将非零元素向前移动
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(last_non_zero, i);
                last_non_zero += 1;
            }
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
