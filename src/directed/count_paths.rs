//! Count the total number of possible paths to reach a destination.

use std::hash::Hash;

use rustc_hash::FxHashMap;

fn internal<
    T: Eq + Hash + Clone,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
>(
    start: T,
    successors: &mut FN,
    success: &mut FS,
    cache: &mut FxHashMap<T, usize>,
) -> usize {
    if success(&start) {
        return 1;
    }

    if let Some(&n) = cache.get(&start) {
        return n;
    }

    let mut count = 0;
    for successor in successors(&start) {
        count += internal(successor.clone(), successors, success, cache);
    }

    cache.insert(start, count);

    count
}

/// Count the total number of possible paths to reach a destination.
///
/// # Example
///
/// On a 8x8 board, find the total paths from the bottom-left square to the top-right square.
///
/// ```
/// let n = count_paths(
///     (0, 0),
///     |&(x, y)| {
///         [(x + 1, y), (x, y + 1)]
///             .into_iter()
///             .filter(|&(x, y)| x < 8 && y < 8)
///     },
///     |&c| c == (7, 7),
/// );
/// assert_eq!(n, 3432);
/// ```
pub fn count_paths<
    T: Eq + Hash + Clone,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
>(
    start: T,
    mut successors: FN,
    mut success: FS,
) -> usize {
    internal(
        start,
        &mut successors,
        &mut success,
        &mut Default::default(),
    )
}
