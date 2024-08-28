//! Compute a shortest path using the [Fringe search
//! algorithm](https://en.wikipedia.org/wiki/Fringe_search).

use super::reverse_path;
use crate::FxIndexMap;
use indexmap::map::Entry::{Occupied, Vacant};
use num_traits::{Bounded, Zero};
use std::collections::VecDeque;
use std::hash::Hash;
use std::mem;

/// Compute a shortest path using the [Fringe search
/// algorithm](https://en.wikipedia.org/wiki/Fringe_search).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned along with its total cost, in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
///   from the node to the successor. This cost must be non-negative.
/// - `heuristic` returns an approximation of the cost from a given node to the goal. The
///   approximation must not be greater than the real cost, or a wrong shortest path may be returned.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
///   a dynamic solution instead of a fixed node.
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
/// use pathfinding::prelude::fringe;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn distance(&self, other: &Pos) -> u32 {
///     (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
///   }
///
///   fn successors(&self) -> Vec<(Pos, u32)> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///          .into_iter().map(|p| (p, 1)).collect()
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = fringe(&Pos(1, 1),
///                     |p| p.successors(),
///                     |p| p.distance(&GOAL) / 3,
///                     |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::fringe;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = fringe(&(1, 1),
///                     |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                    (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                                .into_iter().map(|p| (p, 1)),
///                     |&(x, y)| (GOAL.0.abs_diff(x) + GOAL.1.abs_diff(y)) / 3,
///                     |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
#[allow(clippy::missing_panics_doc)]
pub fn fringe<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Bounded + Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut now = VecDeque::new();
    let mut later = VecDeque::new();
    let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    let mut flimit = heuristic(start);
    now.push_back(0);
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));

    loop {
        if now.is_empty() {
            return None;
        }
        let mut fmin = C::max_value();
        while let Some(i) = now.pop_front() {
            let (g, successors) = {
                let (node, &(_, g)) = parents.get_index(i).unwrap(); // Cannot fail
                let f = g + heuristic(node);
                if f > flimit {
                    if f < fmin {
                        fmin = f;
                    }
                    later.push_back(i);
                    continue;
                }
                if success(node) {
                    let path = reverse_path(&parents, |&(p, _)| p, i);
                    return Some((path, g));
                }
                (g, successors(node))
            };
            for (successor, cost) in successors {
                let g_successor = g + cost;
                let n; // index for successor
                match parents.entry(successor) {
                    Vacant(e) => {
                        n = e.index();
                        e.insert((i, g_successor));
                    }
                    Occupied(mut e) => {
                        if e.get().1 > g_successor {
                            n = e.index();
                            e.insert((i, g_successor));
                        } else {
                            continue;
                        }
                    }
                }
                if !remove(&mut later, &n) {
                    remove(&mut now, &n);
                }
                now.push_front(n);
            }
        }
        mem::swap(&mut now, &mut later);
        flimit = fmin;
    }
}

fn remove<T: Eq>(v: &mut VecDeque<T>, e: &T) -> bool {
    v.iter().position(|x| x == e).is_some_and(|index| {
        v.remove(index);
        true
    })
}
