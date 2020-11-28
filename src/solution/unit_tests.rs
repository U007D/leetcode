#![allow(clippy::unwrap_used, clippy::wildcard_imports)]
use super::*;
use crate::consts::*;
use assert2::assert;
use std::panic;

fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

#[test]
fn empty_vec_with_k_0_panics_as_per_requirements() {
    // Given
    let nums = Vec::new();
    let k = 0;

    // When
    let res = catch_unwind_silent(|| Solution::max_sliding_window(nums, k));

    // Then
    assert!(res.is_err());
}

#[test]
fn non_empty_vec_with_k_0_panics_as_per_requirements() {
    // Given
    let nums = vec![1];
    let k = 0;

    // When
    let res = catch_unwind_silent(|| Solution::max_sliding_window(nums, k));

    // Then
    assert!(res.is_err());
}

#[test]
fn non_empty_vec_with_k_gt_num_len_panics_as_per_requirements() {
    // Given
    let nums = vec![1];
    let k = 2;

    // When
    let res = catch_unwind_silent(|| Solution::max_sliding_window(nums, k));

    // Then
    assert!(res.is_err());
}

#[test]
fn vec_with_too_many_elements_panics_as_per_requirements() {
    // Given
    let nums = vec![1; MAX_NUMS_COUNT.checked_add(1).unwrap()];
    let k = 2;

    // When
    let res = catch_unwind_silent(|| Solution::max_sliding_window(nums, k));

    // Then
    assert!(res.is_err());
}

#[test]
fn vec_containing_too_small_value_panics_as_per_requirements() {
    // Given
    let nums = vec![1, MIN_NUM.checked_sub(1).unwrap()];
    let k = 2;

    // When
    let res = catch_unwind_silent(|| Solution::max_sliding_window(nums, k));

    // Then
    assert!(res.is_err());
}

#[test]
fn vec_containing_too_large_value_panics_as_per_requirements() {
    // Given
    let nums = vec![1, MAX_NUM.checked_add(1).unwrap()];
    let k = 2;

    // When
    let res = catch_unwind_silent(|| Solution::max_sliding_window(nums, k));

    // Then
    assert!(res.is_err());
}

#[test]
fn vec_with_1_value_returns_expected_result() {
    // Given
    let nums = vec![1];
    let k = 1;
    let expected_res = vec![1];

    // When
    let res = Solution::max_sliding_window(nums, k);

    // Then
    assert!(res == expected_res);
}

#[test]
fn vec_with_3_values_and_window_size_of_1_returns_expected_result() {
    // Given
    let nums = vec![1, 2, 3];
    let k = 1;
    let expected_res = vec![1, 2, 3];

    // When
    let res = Solution::max_sliding_window(nums, k);

    // Then
    assert!(res == expected_res);
}

#[test]
fn vec_with_3_values_and_window_size_of_2_returns_expected_results() {
    // Given
    let nums = vec![1, 2, 3];
    let k = 2;
    let expected_res = vec![2, 3];

    // When
    let res = Solution::max_sliding_window(nums, k);

    // Then
    assert!(res == expected_res);
}
