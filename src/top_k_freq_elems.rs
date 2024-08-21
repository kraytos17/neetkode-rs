use std::collections::HashMap;

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::with_capacity(nums.len());
        for num in nums {
            match map.get_mut(&num) {
                Some(entry) => *entry += 1,
                None => {
                    map.insert(num, 0);
                }
            }
        }

        let mut freq: Vec<(i32, i32)> = map.into_iter().collect();
        freq.sort_unstable_by_key(|tuple_value| tuple_value.1);
        let freq_len = freq.len();

        freq.into_iter()
            .skip(freq_len - k as usize)
            .map(|tuple_value| tuple_value.0)
            .collect()
    }
}
