//! Chapter_14 crate
//!
//! `chapter_14` is a collection of methods that illustrate examples of how to use _Cargo_

extern crate rand;

/// Adds one to the number given.
///
/// # Examples
/// ```
/// # let five = 5;
/// assert_eq!(6, add_one::add_one(five)) ;
/// ```
pub fn add_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
