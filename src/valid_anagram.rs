use crate::contains_duplicates::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0; 26];

        for (sc, tc) in s.chars().zip(t.chars()) {
            count[(sc as u8 - b'a') as usize] += 1;
            count[(tc as u8 - b'a') as usize] -= 1;
        }

        count.iter().all(|&x| x == 0)
    }
}
