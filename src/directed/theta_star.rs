//! Compute a path that is not constrained to the edges of the graph, using the [Theta\*
//! algorithm](https://arxiv.org/abs/1401.3843).

use indexmap::map::Entry::{Occupied, Vacant};
use num_traits::Zero;
use std::hash::Hash;

use super::astar::SmallestCostHolder;
use super::reverse_path;
use crate::FxIndexMap;
use std::collections::BinaryHeap;

/// Compute a path from `start` to a node for which `success` returns `true`, allowing the path
/// to leave the edges of the graph wherever a straight line is available.
///
/// [`astar`](super::astar::astar) returns a path made of graph edges, so on a grid it can only
/// travel in the directions the grid offers and crosses open ground as a staircase. Theta\*
/// keeps the same search order, but each time it reaches a node it asks whether the node before
/// it can see the successor directly, and if so joins them in a straight line instead. The
/// waypoints are still graph nodes; the segments between them are not.
///
/// - `start` is the starting node.
/// - `successors` returns the neighbours of a node with the cost of moving to each of them.
/// - `heuristic` approximates the cost from a node to the goal. It must not overestimate it.
/// - `sight` returns the cost of travelling straight from one node to another, or `None` when
///   the line between them is blocked. It is asked about nodes that are not neighbours.
/// - `success` checks whether the goal has been reached.
///
/// # Differences from the rest of this crate
///
/// **The path is not optimal.** It is never longer than the one [`astar`](super::astar::astar)
/// would return for the same graph, and is usually close to the shortest any-angle path, but
/// unlike `astar` and [`dijkstra`](super::dijkstra::dijkstra) there is no guarantee: finding the
/// true optimum needs a visibility graph rather than a grid.
///
/// **Consecutive nodes of the path need not be neighbours.** The path is a list of waypoints
/// joined by straight lines, so `path.windows(2)` are not necessarily edges of `successors`.
/// That is the point of the algorithm, but it differs from every other path this crate returns.
///
/// `sight` must agree with `successors`: a straight line may never cost more than walking the
/// same way through intermediate nodes. Otherwise the shortcut can make a path worse, and the
/// cost returned stops matching the path.
///
/// # Example
///
/// Five nodes in a row, each a unit step from the next. Every node can see every other, so the
/// intermediate ones carry no information and the path collapses to its endpoints — at the same
/// cost `astar` would report for walking all five.
///
/// ```
/// use pathfinding::prelude::theta_star;
///
/// let (path, cost) = theta_star(
///     &0,
///     |&n: &i32| (n < 4).then(|| (n + 1, 1)),
///     |&n| 4 - n,
///     |&a: &i32, &b: &i32| Some((b - a).abs()),
///     |&n| n == 4,
/// )
/// .expect("no path found");
///
/// assert_eq!(cost, 4);
/// assert_eq!(path, vec![0, 4]);
/// ```
#[expect(clippy::missing_panics_doc)]
pub fn theta_star<N, C, FN, IN, FH, FL, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut sight: FL,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FL: FnMut(&N, &N) -> Option<C>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: Zero::zero(),
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));
    while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
        let successors = {
            let (node, &(_, c)) = parents.get_index(index).unwrap(); // Cannot fail
            if success(node) {
                let path = reverse_path(&parents, |&(p, _)| p, index);
                return Some((path, cost));
            }
            // We may have inserted a node several time into the binary heap if we found
            // a better way to access it. Ensure that we are currently dealing with the
            // best path and discard the others.
            if cost > c {
                continue;
            }
            successors(node)
        };
        // The node the path reached this one from, which is the one a shortcut would start at.
        // The starting node has no predecessor and stands in for its own.
        let previous = match parents.get_index(index).unwrap().1.0 {
            usize::MAX => index,
            parent => parent,
        };
        for (successor, move_cost) in successors {
            // Prefer a straight line from the previous waypoint. It can never be dearer than
            // going through this node, so the step through this node is only used when the
            // line is blocked.
            let shortcut = {
                let (from, &(_, from_cost)) = parents.get_index(previous).unwrap();
                sight(from, &successor).map(|line_cost| from_cost + line_cost)
            };
            let (new_parent, new_cost) = match shortcut {
                Some(total) => (previous, total),
                None => (index, cost + move_cost),
            };
            let h; // heuristic(&successor)
            let n; // index for successor
            match parents.entry(successor) {
                Vacant(e) => {
                    h = heuristic(e.key());
                    n = e.index();
                    e.insert((new_parent, new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        h = heuristic(e.key());
                        n = e.index();
                        e.insert((new_parent, new_cost));
                    } else {
                        continue;
                    }
                }
            }

            to_see.push(SmallestCostHolder {
                estimated_cost: new_cost + h,
                cost: new_cost,
                index: n,
            });
        }
    }
    None
}
