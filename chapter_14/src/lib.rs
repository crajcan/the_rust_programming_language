//! Chapter_14 crate
//!
//! `chapter_14` is a collection of methods that illustrate examples of how to use _Cargo_

/// Adds one to the number given.
///
/// # Examples
/// ```
/// # let five = 5;
/// assert_eq!(6, chapter_14::add_one(five)) ;
/// ```
pub fn add_one(n: i32) -> i32 {
    n + 1
}
