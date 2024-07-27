use crate::contains_duplicates::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered_chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let len = filtered_chars.len();
        for i in 0..len / 2 {
            if filtered_chars[i] != filtered_chars[len - 1 - i] {
                return false;
            }
        }
        true
    }
}
