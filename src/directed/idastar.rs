//! Compute a shortest path using the [IDA* search
//! algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_A*).

use crate::FxIndexSet;
use crate::add_costs;
use num_traits::Zero;
use std::{hash::Hash, ops::ControlFlow};

/// Compute a shortest path using the [IDA* search
/// algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_A*).
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
/// # Panics
///
/// This function panics if the cost of a path, or the sum of a path cost and a heuristic
/// estimate, does not fit into `C`. Silently returning a wrapped, and therefore wrong, cost
/// would be worse than failing loudly. If your costs can come close to the limits of the
/// type, use a wider type, or a wrapper type whose addition saturates.
///
/// # Example
///
/// We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
/// moves.
///
/// The first version uses an explicit type `Pos` on which the required traits are derived.
///
/// ```
/// use pathfinding::prelude::idastar;
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
/// let result = idastar(&Pos(1, 1), |p| p.successors(), |p| p.distance(&GOAL) / 3,
///                    |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::idastar;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = idastar(&(1, 1),
///                    |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                   (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                               .into_iter().map(|p| (p, 1)),
///                    |&(x, y)| (GOAL.0.abs_diff(x) + GOAL.1.abs_diff(y)) / 3,
///                    |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
/// # Tie-breaking
///
/// Successors whose estimated total cost is equal are explored in the order `successors`
/// returned them, so the choice belongs to the caller: yield the preferred one first.
///
#[must_use]
pub fn idastar<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Clone + Hash,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut path = FxIndexSet::default();
    path.insert(start.clone());

    std::iter::repeat(())
        .try_fold(heuristic(start), |bound, ()| {
            search(
                &mut path,
                bound,
                &mut successors,
                &mut heuristic,
                &mut success,
            )
            .map_break(Some)?
            // .filter(|min| *min > bound)
            .map_or(ControlFlow::Break(None), ControlFlow::Continue)
        })
        .break_value()
        .unwrap_or_default() // To avoid a missing panics section, as this always break
}

/// A node of the current path whose successors are being explored.
struct Frame<N, C> {
    /// Successors left to explore, along with the cost of the edge leading to them and their
    /// estimated cost from this node.
    successors: std::vec::IntoIter<(N, C, C)>,
    /// Cost of the path up to this node.
    cost: C,
    /// Smallest estimated cost found beyond the bound below this node so far.
    min: Option<C>,
}

/// Explore every path from the last node of `path` whose estimated cost stays within `bound`.
///
/// This is a depth-first search, written with an explicit stack rather than recursively so that
/// a long path cannot overflow the call stack. It breaks with the path found and its cost, or
/// continues with the smallest estimated cost which exceeded `bound`, if any.
fn search<N, C, FN, IN, FH, FS>(
    path: &mut FxIndexSet<N>,
    bound: C,
    successors: &mut FN,
    heuristic: &mut FH,
    success: &mut FS,
) -> ControlFlow<(Vec<N>, C), Option<C>>
where
    N: Eq + Clone + Hash,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    // One frame for every node of `path` being explored. The last node of `path` has no frame
    // while it is being entered.
    let mut stack: Vec<Frame<N, C>> = Vec::new();
    // Cost of the path up to the node being entered.
    let mut cost = Zero::zero();
    loop {
        // Enter the last node of `path`.
        let node = &path[path.len() - 1];
        let f = add_costs(cost, heuristic(node));
        // Smallest estimated cost beyond the bound, once the last node of `path` is done with.
        let mut done = if f > bound {
            Some(Some(f))
        } else if success(node) {
            return ControlFlow::Break((path.iter().cloned().collect(), f));
        } else {
            let mut neighbs: Vec<(N, C, C)> = successors(node)
                .into_iter()
                .filter_map(|(n, c)| {
                    (!path.contains(&n)).then(|| {
                        let h = heuristic(&n);
                        (n, c, add_costs(c, h))
                    })
                })
                .collect::<Vec<_>>();
            // A stable sort, so that successors of equal estimated cost stay in the order the
            // caller produced them: that order is the caller's way of breaking the tie.
            neighbs.sort_by_key(|(_, _, c1)| *c1);
            stack.push(Frame {
                successors: neighbs.into_iter(),
                cost,
                min: None,
            });
            None
        };
        // Find the next node to enter, leaving every node whose successors are exhausted.
        loop {
            if let Some(min) = done.take() {
                let Some(parent) = stack.last_mut() else {
                    // The start node is done with: every path within the bound was explored.
                    return ControlFlow::Continue(min);
                };
                path.pop();
                match min {
                    Some(m) if parent.min.is_none_or(|n| n >= m) => parent.min = Some(m),
                    _ => (),
                }
            }
            // Cannot fail: a frame was pushed or a parent frame was found above.
            let frame = stack.last_mut().unwrap();
            if let Some((node, extra, _)) = frame.successors.next() {
                cost = add_costs(frame.cost, extra);
                path.insert(node);
                break;
            }
            done = stack.pop().map(|frame| frame.min);
        }
    }
}
