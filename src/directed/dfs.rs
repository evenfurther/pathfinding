//! Compute a path using the [depth-first search
//! algorithm](https://en.wikipedia.org/wiki/Depth-first_search).

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FusedIterator;

/// Compute a path using the [depth-first search
/// algorithm](https://en.wikipedia.org/wiki/Depth-first_search).
/// The path starts from `start` up to a node for which `success` returns `true` is computed and
/// returned along with its total cost, in a `Some`. If no path can be found, `None` is returned
/// instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, which will be tried in order.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
/// a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node. Note that the start node ownership
/// is taken by `dfs` as no clones are made.
///
/// # Example
///
/// We will search a way to get from 1 to 17 while only adding 1 or multiplying the number by
/// itself.
///
/// If we put the adder first, an adder-only solution will be found:
///
/// ```
/// use pathfinding::prelude::dfs;
///
/// assert_eq!(dfs(1, |&n| vec![n+1, n*n].into_iter().filter(|&x| x <= 17), |&n| n == 17),
///            Some((1..18).collect()));
/// ```
///
/// However, if we put the multiplier first, a shorter solution will be explored first:
///
/// ```
/// use pathfinding::prelude::dfs;
///
/// assert_eq!(dfs(1, |&n| vec![n*n, n+1].into_iter().filter(|&x| x <= 17), |&n| n == 17),
///            Some(vec![1, 2, 4, 16, 17]));
/// ```
pub fn dfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    let mut path = vec![start];
    if step(&mut path, &mut successors, &mut success) {
        Some(path)
    } else {
        None
    }
}

fn step<N, FN, IN, FS>(path: &mut Vec<N>, successors: &mut FN, success: &mut FS) -> bool
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    if success(path.last().unwrap()) {
        true
    } else {
        let successors_it = successors(path.last().unwrap());
        for n in successors_it {
            if !path.contains(&n) {
                path.push(n);
                if step(path, successors, success) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }
}

/// Visit all nodes that are reachable from a start node. The node will be visited
/// in DFS order, starting from the `start` node and following the order returned
/// by the `successors` function.
///
/// # Examples
///
/// The iterator stops when there are no new nodes to visit:
///
/// ```
/// use pathfinding::prelude::dfs_reach;
///
/// let all_nodes = dfs_reach(3, |_| (1..=5)).collect::<Vec<_>>();
/// assert_eq!(all_nodes, vec![3, 1, 2, 4, 5]);
/// ```
///
/// The iterator can be used as a generator. Here are for examples
/// the multiples of 2 and 3 smaller than 15 (although not in
/// natural order but in the order they are discovered by the DFS
/// algorithm):
///
/// ```
/// use pathfinding::prelude::dfs_reach;
///
/// let mut it = dfs_reach(1, |&n| vec![n*2, n*3].into_iter().filter(|&x| x < 15)).skip(1);
/// assert_eq!(it.next(), Some(2));  // 1*2
/// assert_eq!(it.next(), Some(4));  // (1*2)*2
/// assert_eq!(it.next(), Some(8));  // ((1*2)*2)*2
/// assert_eq!(it.next(), Some(12)); // ((1*2)*2)*3
/// assert_eq!(it.next(), Some(6));  // (1*2)*3
/// assert_eq!(it.next(), Some(3));  // 1*3
/// // (1*3)*2 == 6 which has been seen already
/// assert_eq!(it.next(), Some(9));  // (1*3)*3
/// ```
pub fn dfs_reach<N, FN, IN>(start: N, successors: FN) -> DfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let (to_see, mut seen) = (vec![start.clone()], HashSet::new());
    seen.insert(start);
    DfsReachable {
        to_see,
        seen,
        successors,
    }
}

/// Struct returned by [`dfs_reach`](crate::directed::dfs::dfs_reach).
pub struct DfsReachable<N, FN> {
    to_see: Vec<N>,
    seen: HashSet<N>,
    successors: FN,
}

impl<N, FN, IN> Iterator for DfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.to_see.pop() {
            let mut to_insert = Vec::new();
            for s in (self.successors)(&n) {
                if !self.seen.contains(&s) {
                    to_insert.push(s.clone());
                    self.seen.insert(s);
                }
            }
            self.to_see.extend(to_insert.into_iter().rev());
            Some(n)
        } else {
            None
        }
    }
}

impl<N, FN, IN> FusedIterator for DfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
}
