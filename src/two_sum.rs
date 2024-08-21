use std::collections::HashMap;

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut freq_map = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = freq_map.get(&num) {
                return vec![j as i32, i as i32];
            }
            freq_map.insert(target - num, i);
        }
        vec![]
    }
}
