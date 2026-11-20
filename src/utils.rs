//! Miscellaneous utilities

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
    let root = isqrt(n);
    (n == root * root).then_some(root)
}

/// Integer square root of `n`, rounded down, computed digit by digit in base 4 so that it is
/// exact for unsigned integers of any width.
fn isqrt<T>(n: T) -> T
where
    T: PrimInt + Unsigned,
{
    // Largest power of 4 not greater than `n`, starting from the largest one `T` can hold.
    let mut bit = T::one() << ((T::zero().count_zeros() as usize - 2) & !1);
    while bit > n {
        bit = bit >> 2;
    }
    // `root` holds the root found so far, shifted left by the number of base-4 digits left to
    // compute, and `rem` what is left of `n` once that root is squared and subtracted.
    let mut rem = n;
    let mut root = T::zero();
    while !bit.is_zero() {
        if rem >= root + bit {
            rem = rem - (root + bit);
            root = (root >> 1) + bit;
        } else {
            root = root >> 1;
        }
        bit = bit >> 2;
    }
    root
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
#[expect(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
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
///
/// # Panics
///
/// This function panics if `upper` is 0, as no value lies in `0..0`.
#[must_use]
#[expect(clippy::cast_sign_loss)]
pub const fn constrain(value: isize, upper: usize) -> usize {
    assert!(upper > 0, "cannot constrain a value into an empty range");
    if value > 0 {
        value as usize % upper
    } else {
        // `unsigned_abs` rather than negation: `-isize::MIN` does not fit in an `isize`.
        (upper - value.unsigned_abs() % upper) % upper
    }
}

#[cfg(test)]
mod tests {
    use super::isqrt;

    #[test]
    fn isqrt_u8_exhaustive() {
        for n in 0..=u8::MAX {
            assert_eq!(isqrt(n), n.isqrt(), "{n}");
        }
    }

    #[test]
    fn isqrt_u16_exhaustive() {
        for n in 0..=u16::MAX {
            assert_eq!(isqrt(n), n.isqrt(), "{n}");
        }
    }

    #[test]
    fn isqrt_near_squares() {
        for r in (0..=u64::from(u32::MAX))
            .step_by(65_537)
            .chain([u64::from(u32::MAX)])
        {
            let square = r * r;
            assert_eq!(isqrt(square), r);
            if square > 0 {
                assert_eq!(isqrt(square - 1), r - 1);
            }
            assert_eq!(isqrt(square + 2 * r), r);
        }
        assert_eq!(isqrt(u64::MAX), u64::from(u32::MAX));
        assert_eq!(isqrt(u128::MAX), u128::from(u64::MAX));
        assert_eq!(isqrt(usize::MAX), usize::MAX.isqrt());
    }
}
