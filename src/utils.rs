//! Miscellaneous utilities

use std::ops::Sub;

/// Compute the absolute difference between two values.
///
/// # Example
///
/// The absolute difference between 4 and 17 as unsigned values will be 13.
///
/// ```
/// use pathfinding::utils::absdiff;
///
/// assert_eq!(absdiff(4u32, 17u32), 13u32);
/// assert_eq!(absdiff(17u32, 4u32), 13u32);
/// ```
#[inline]
pub fn absdiff<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}
