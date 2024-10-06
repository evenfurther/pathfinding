//! Miscellaneous utilities

use integer_sqrt::IntegerSquareRoot;
use num_traits::{PrimInt, Unsigned};

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
pub fn uint_sqrt<T>(n: T) -> Option<T>
where
    T: PrimInt + Unsigned,
{
    let root = n.integer_sqrt();
    (n == root * root).then_some(root)
}

/// Move a two-dimensional coordinate into a given direction provided that:
/// - The `start` point is valid (given the `dimensions`).
/// - The `direction` is not `(0,0)`
/// - The target point is valid (given the `dimensions`).
///
/// # Example
///
/// ```
/// use pathfinding::utils::move_in_direction;
///
/// let board = (8, 8);
/// assert_eq!(move_in_direction((5, 5), (-1, -2), board), Some((4, 3)));
/// assert_eq!(move_in_direction((1, 1), (-1, -2), board), None);
/// ```
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn move_in_direction(
    start: (usize, usize),
    direction: (isize, isize),
    dimensions: (usize, usize),
) -> Option<(usize, usize)> {
    let (row, col) = start;
    if row >= dimensions.0 || col >= dimensions.1 || direction == (0, 0) {
        return None;
    }
    let (new_row, new_col) = (row as isize + direction.0, col as isize + direction.1);
    (new_row >= 0
        && (new_row as usize) < dimensions.0
        && new_col >= 0
        && (new_col as usize) < dimensions.1)
        .then_some((new_row as usize, new_col as usize))
}

/// Repeatedly call [`move_in_direction`] until the returned value
/// is `None`.
///
/// # Example
///
/// ```
/// use pathfinding::utils::in_direction;
///
/// let board = (8, 8);
/// let positions = in_direction((0, 0), (1, 2), board).collect::<Vec<_>>();
/// assert_eq!(positions, vec![(1, 2), (2, 4), (3, 6)]);
/// ```
pub fn in_direction(
    start: (usize, usize),
    direction: (isize, isize),
    dimensions: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    std::iter::successors(Some(start), move |current| {
        move_in_direction(*current, direction, dimensions)
    })
    .skip(1)
}

/// Constrain `value` into `0..upper` by adding or subtracting `upper`
/// as many times as necessary.
///
/// # Examples
///
/// ```
/// use pathfinding::utils::constrain;
///
/// assert_eq!(constrain(5, 7), 5);
/// assert_eq!(constrain(30, 7), 2);
/// assert_eq!(constrain(-30, 7), 5);
/// ```
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub const fn constrain(value: isize, upper: usize) -> usize {
    if value > 0 {
        value as usize % upper
    } else {
        (upper - (-value) as usize % upper) % upper
    }
}
