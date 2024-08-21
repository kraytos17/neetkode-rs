use std::collections::HashMap;

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max_len = 0;
        let mut char_map: HashMap<char, i32> = HashMap::new();
        
        for (end, c) in s.chars().enumerate() {
            if let Some(&prev) = char_map.get(&c) {
                if prev >= start {
                    start = prev + 1;
                }
            }
            
            char_map.insert(c, end as i32);
            max_len = max_len.max(end as i32 - start + 1);
        }
        
        max_len
    }
}
