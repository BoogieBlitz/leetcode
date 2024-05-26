use std::collections::HashMap;

/// Two Sum
/// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result_map: HashMap<&i32, usize> = HashMap::new();
    for (left_idx, num) in nums.iter().enumerate() {
        match result_map.get(&(target - num)) {
            Some(right_idx) => return vec![left_idx as i32, *right_idx as i32],
            None => {
                result_map.insert(num, left_idx);
            }
        }
    }
    vec![0, 0]
}

pub fn main() {
    let nums: Vec<i32> = vec![2,7,11,15];
    let target: i32 = 9;
    println!("{:?}", two_sum(nums, target));
}
