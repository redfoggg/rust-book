//! # docs
//!
//! `docs` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = docs::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(a: i32) -> i32 {
    a + 1
}
