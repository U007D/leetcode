#[cfg(test)]
mod unit_tests;

#[allow(clippy::wildcard_imports)]
use crate::consts::*;
use assert2::assert;
use std::{convert::TryFrom, num::NonZeroUsize, ops::Not};

pub struct Solution {}

impl Solution {
    #[allow(
        clippy::needless_pass_by_value,
        clippy::indexing_slicing,
        clippy::integer_arithmetic
    )]
    #[must_use]
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Safely convert count binding `k: i32` to canonical count type (`usize`)
        let k = NonZeroUsize::try_from(
            usize::try_from(k)
                .expect("Input parameter `k` cannot be converted to `usize`. (Is `k` negative?)"),
        )
        .expect("Input parameter 'k' cannot be 0.");

        // Constraints from requirements assumed to be true
        assert!(nums.is_empty().not() && nums.len() <= MAX_NUMS_COUNT);
        assert!(nums.iter().all(|&n| n >= MIN_NUM && n <= MAX_NUM));
        assert!(k.get() <= nums.len());

        (0..=nums.len() - k.get())
            .map(|start| {
                nums[start..start + k.get()]
                    .iter()
                    .max()
                    .cloned()
                    .expect("Internal error: Search range is empty.  Must specify non-empty range.")
            })
            .collect()
    }
}
