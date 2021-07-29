//! Miscellaneous utilities

use integer_sqrt::IntegerSquareRoot;
use std::ops::Sub;

use num_traits::{PrimInt, Unsigned};

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

/// Return the square root of `n` if `n` is square, `None` otherwise.
///
/// # Example
///
/// ```
/// use pathfinding::utils::uint_sqrt;
///
/// assert_eq!(uint_sqrt(100usize), Some(10));
/// assert_eq!(uint_sqrt(10usize), None);
/// ```
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
pub fn uint_sqrt<T>(n: T) -> Option<T>
where
    T: PrimInt + Unsigned,
{
    let root = n.integer_sqrt();
    (n == root * root).then(|| root)
}
