//! Count the total number of possible paths to reach a destination.

use std::hash::Hash;

use crate::FxIndexMap;
use indexmap::map::Entry::{Occupied, Vacant};

/// Account for `node` in the search.
///
/// Returns the number of paths from it when that is already settled. Otherwise the node is
/// recorded as being worked on and a frame is pushed for it, and `None` is returned; note that
/// the stack is therefore only ever pushed to when this returns `None`.
///
/// # Panics
///
/// If `node` is reached again while its own count is still unknown, it lies on a loop.
fn enter<T, FN, IN, FS>(
    node: T,
    counts: &mut FxIndexMap<T, Option<usize>>,
    stack: &mut Vec<(usize, IN::IntoIter, usize)>,
    successors: &mut FN,
    success: &mut FS,
) -> Option<usize>
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    match counts.entry(node) {
        Occupied(e) => match *e.get() {
            Some(count) => Some(count),
            None => panic!("the graph given to count_paths contains a loop"),
        },
        Vacant(e) => {
            if success(e.key()) {
                e.insert(Some(1));
                Some(1)
            } else {
                let index = e.index();
                let successors = successors(e.key()).into_iter();
                e.insert(None);
                stack.push((index, successors, 0));
                None
            }
        }
    }
}

fn count_paths_from<T, FN, IN, FS>(start: T, successors: &mut FN, success: &mut FS) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    // The nodes live here rather than in the stack, which refers to them by index, so that `T`
    // need not be `Clone`. A `None` count marks a node that is still being worked out.
    let mut counts: FxIndexMap<T, Option<usize>> = FxIndexMap::default();
    // Each frame is a node, the successors of it left to look at, and the number of paths
    // found through the ones already done.
    let mut stack: Vec<(usize, IN::IntoIter, usize)> = Vec::new();

    if let Some(count) = enter(start, &mut counts, &mut stack, successors, success) {
        return count;
    }

    let mut total = 0;
    while let Some(top) = stack.last_mut() {
        // The borrow of `stack` ends here, so that the body below is free to push onto it.
        let successor = top.1.next();
        if let Some(successor) = successor {
            if let Some(count) = enter(successor, &mut counts, &mut stack, successors, success) {
                // Nothing was pushed, so the frame on top is still the one being counted.
                stack.last_mut().unwrap().2 += count;
            }
        } else {
            // Every successor is accounted for, so this node's own count is settled.
            let (index, _, count) = stack.pop().unwrap();
            *counts.get_index_mut(index).unwrap().1 = Some(count);
            match stack.last_mut() {
                Some(parent) => parent.2 += count,
                // The starting node, which is the last frame to be popped.
                None => total = count,
            }
        }
    }
    total
}

/// Count the total number of possible paths to reach a destination.
///
/// # Example
///
/// On a 8x8 board, find the total paths from the bottom-left square to the top-right square.
///
/// ```
/// use pathfinding::prelude::count_paths;
///
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
///
/// # Panics
///
/// If the graph contains a loop, since the number of paths through it is then unbounded.
pub fn count_paths<T, FN, IN, FS>(start: T, mut successors: FN, mut success: FS) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    count_paths_from(start, &mut successors, &mut success)
}
