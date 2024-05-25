/// Two Sum
/// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for (left_idx, num) in nums.iter().enumerate() {
        if nums.contains(&(target-num)) {
            let right_idx: usize = nums.iter().position(|r| r == &(target - num)).unwrap();

            if left_idx != right_idx {
                result.push(left_idx as i32);
                result.push(right_idx as i32);
                break;
            }
        }
    }
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        let nums: Vec<i32> = vec![3,3];
        let target: i32 = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
