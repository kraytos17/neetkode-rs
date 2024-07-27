use std::collections::HashMap;

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut str_map: HashMap<String, Vec<String>> = HashMap::new();
        for elem in strs {
            let mut chars: Vec<char> = elem.chars().collect();
            chars.sort_unstable();
            let sorted_elem: String = chars.into_iter().collect();
            str_map
                .entry(sorted_elem)
                .or_insert_with(Vec::new)
                .push(elem);
        }

        str_map.into_values().collect()
    }
}
