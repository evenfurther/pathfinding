//! Compute a shortest path using the [breadth-first search
//! algorithm](https://en.wikipedia.org/wiki/Breadth-first_search).

use indexmap::map::Entry::Vacant;
use indexmap::IndexMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::usize;

use super::reverse_path;

/// Compute a shortest path using the [breadth-first search
/// algorithm](https://en.wikipedia.org/wiki/Breadth-first_search).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
/// a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node.
///
/// # Example
///
/// We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
/// moves.
///
/// The first version uses an explicit type `Pos` on which the required traits are derived.
///
/// ```
/// use pathfinding::prelude::bfs;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn successors(&self) -> Vec<Pos> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").len(), 5);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::bfs;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = bfs(&(1, 1),
///                  |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                 (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)],
///                  |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").len(), 5);
/// ```
pub fn bfs<N, FN, IN, FS>(start: &N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = VecDeque::new();
    let mut parents: IndexMap<N, usize> = IndexMap::new();
    to_see.push_back(0);
    parents.insert(start.clone(), usize::MAX);
    while let Some(i) = to_see.pop_front() {
        let successors = {
            let node = parents.get_index(i).unwrap().0;
            if success(node) {
                let path = reverse_path(&parents, |&p| p, i);
                return Some(path);
            }
            successors(node)
        };
        for successor in successors {
            if let Vacant(e) = parents.entry(successor) {
                to_see.push_back(e.index());
                e.insert(i);
            }
        }
    }
    None
}

/// Return one of the shortest loop from start to start if it exists, `None` otherwise.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node.
///
/// Except the start node which will be included both at the beginning and the end of
/// the path, a node will never be included twice in the path as determined
/// by the `Eq` relationship.
pub fn bfs_loop<N, FN, IN>(start: &N, mut successors: FN) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    // If the node is linked to itself, we have the shortest path.
    if successors(start).into_iter().any(|n| &n == start) {
        return Some(vec![start.clone(), start.clone()]);
    }
    // We will go through all the successors and look for a path to the start.
    let mut shortest = None;
    let mut shortest_len = usize::MAX;
    for successor in successors(start).into_iter() {
        if let Some(path) = bfs(&successor, &mut successors, |n| n == start) {
            let path_len = path.len();
            if path_len < shortest_len {
                shortest_len = path_len;
                shortest = Some(path);
            }
            if path_len == 2 {
                break; // We will never find a shorter path than successor->start
            }
        }
    }
    shortest.map(|mut path| {
        let mut cycle = Vec::with_capacity(path.len() + 1);
        cycle.push(start.clone());
        cycle.append(&mut path);
        cycle
    })
}
