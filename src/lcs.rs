use std::collections::HashSet;

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let nums: HashSet<_> = nums.into_iter().collect();
        let mut longest_streak = 0;

        for &num in &nums {
            if !nums.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while nums.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                longest_streak = longest_streak.max(current_streak);
            }
        }

        longest_streak
    }
}
