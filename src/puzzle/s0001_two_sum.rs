use std::collections::HashMap;

/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (idx, numb ) in nums.iter().enumerate() {
        match map.get(&(target - numb)) {
            None => {
                map.insert(numb, idx);
            }
            Some(sub_index) => return vec![*sub_index as i32, idx as i32]
        }
    }
    vec![]
}

#[cfg(test)]
mod test {
    use crate::puzzle::s0001_two_sum::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
    }
}