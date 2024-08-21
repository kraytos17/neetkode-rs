use std::{
    cmp::{max, min},
    i32,
};

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut min_val = i32::MAX;
        let mut max_prof = 0;
        for i in prices {
            min_val =min(i, min_val);
            max_prof = max(max_prof, i - min_val);
        }

        max_prof
    }
}
