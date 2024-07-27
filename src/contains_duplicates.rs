use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::<i32>::new();
        for elem in nums {
            if !seen.insert(elem) {
                return true;
            }
        }

        false
    }
}
