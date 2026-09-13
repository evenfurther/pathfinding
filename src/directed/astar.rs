//! Compute a shortest path (or all shorted paths) using the [A* search
//! algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).

use indexmap::map::Entry::{Occupied, Vacant};
use num_traits::Zero;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::iter::FusedIterator;

use super::reverse_path;
use crate::FxIndexMap;

/// Compute a shortest path using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
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
/// use pathfinding::prelude::astar;
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
/// let result = astar(&Pos(1, 1), |p| p.successors(), |p| p.distance(&GOAL) / 3,
///                    |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::astar;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = astar(&(1, 1),
///                    |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                   (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                               .into_iter().map(|p| (p, 1)),
///                    |&(x, y)| (GOAL.0.abs_diff(x) + GOAL.1.abs_diff(y)) / 3,
///                    |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
#[expect(clippy::missing_panics_doc)]
pub fn astar<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
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
        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;
            let h; // heuristic(&successor)
            let n; // index for successor
            match parents.entry(successor) {
                Vacant(e) => {
                    h = heuristic(e.key());
                    n = e.index();
                    e.insert((index, new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        h = heuristic(e.key());
                        n = e.index();
                        e.insert((index, new_cost));
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

/// Compute all shortest paths using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
///
/// Whereas `astar` (non-deterministic-ally) returns a single shortest
/// path, `astar_bag` returns all shortest paths (in a
/// non-deterministic order).
///
/// The shortest paths starting from `start` up to a node for which `success` returns `true` are
/// computed and returned in an iterator along with the cost (which, by definition, is the same for
/// each shortest path), wrapped in a `Some`. If no paths are found, `None` is returned.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
///   from the node to the successor.
/// - `heuristic` returns an approximation of the cost from a given node to the goal. The
///   approximation must not be greater than the real cost, or a wrong shortest path may be returned.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
///   a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// Each path comprises both the start and an end node. Note that while every path shares the same
/// start node, different paths may have different end nodes.
#[expect(clippy::missing_panics_doc)]
pub fn astar_bag<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(AstarSolution<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    let mut min_cost = None;
    // A node has only as many optimal parents as it has incoming edges, and a goal is reached
    // only once, so plain vectors are both cheaper to fill and cheaper to walk than hash sets.
    let mut sinks: Vec<usize> = Vec::new();
    // Costs are non-negative, so a cycle among optimal parents needs every edge on it to cost
    // nothing. If no such edge is ever seen, the solution walk cannot loop and does not need to
    // guard against it.
    let mut zero_cost_edge = false;
    to_see.push(SmallestCostHolder {
        estimated_cost: Zero::zero(),
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: FxIndexMap<N, (Vec<usize>, C)> = FxIndexMap::default();
    parents.insert(start.clone(), (Vec::new(), Zero::zero()));
    while let Some(SmallestCostHolder {
        cost,
        index,
        estimated_cost,
        ..
    }) = to_see.pop()
    {
        if matches!(min_cost, Some(min_cost) if estimated_cost > min_cost) {
            break;
        }
        let successors = {
            let (node, &(_, c)) = parents.get_index(index).unwrap(); // Cannot fail
            if success(node) {
                min_cost = Some(cost);
                if !sinks.contains(&index) {
                    sinks.push(index);
                }
            }
            // We may have inserted a node several time into the binary heap if we found
            // a better way to access it. Ensure that we are currently dealing with the
            // best path and discard the others.
            if cost > c {
                continue;
            }
            successors(node)
        };
        for (successor, move_cost) in successors {
            zero_cost_edge |= move_cost.is_zero();
            let new_cost = cost + move_cost;
            let h; // heuristic(&successor)
            let n; // index for successor
            match parents.entry(successor) {
                Vacant(e) => {
                    h = heuristic(e.key());
                    n = e.index();
                    e.insert((vec![index], new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        h = heuristic(e.key());
                        n = e.index();
                        let s = e.get_mut();
                        s.0.clear();
                        s.0.push(index);
                        s.1 = new_cost;
                    } else {
                        if e.get().1 == new_cost {
                            // New parent with an identical cost, this is not
                            // considered as an insertion.
                            let s = e.get_mut();
                            if !s.0.contains(&index) {
                                s.0.push(index);
                            }
                        }
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

    min_cost.map(|cost| {
        let parents = parents.into_iter().map(|(k, (ps, _))| (k, ps)).collect();
        (
            AstarSolution {
                sinks,
                // The start is inserted into `parents` before anything else.
                start: 0,
                may_loop: zero_cost_edge,
                parents,
                current: vec![],
                terminated: false,
            },
            cost,
        )
    })
}

/// Compute all shortest paths using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
///
/// Whereas `astar` (non-deterministic-ally) returns a single shortest
/// path, `astar_bag` returns all shortest paths (in a
/// non-deterministic order).
///
/// This is a utility function which collects the results of the `astar_bag` function into a
/// vector. Most of the time, it is more appropriate to use `astar_bag` directly.
///
/// ### Warning
///
/// The number of results with the same value might be very large in some graphs. Use with caution.
pub fn astar_bag_collect<N, C, FN, IN, FH, FS>(
    start: &N,
    successors: FN,
    heuristic: FH,
    success: FS,
) -> Option<(Vec<Vec<N>>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    astar_bag(start, successors, heuristic, success)
        .map(|(solutions, cost)| (solutions.collect(), cost))
}

/// This structure is used to implement Rust's max-heap as a min-heap
/// version for A*. The smallest `estimated_cost` (which is the sum of
/// the `cost` and the heuristic) is preferred. For the same
/// `estimated_cost`, the highest `cost` will be favored, as it may
/// indicate that the goal is nearer, thereby requiring fewer
/// exploration steps.
struct SmallestCostHolder<K> {
    estimated_cost: K,
    cost: K,
    index: usize,
}

impl<K: PartialEq> PartialEq for SmallestCostHolder<K> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl<K: PartialEq> Eq for SmallestCostHolder<K> {}

impl<K: Ord> PartialOrd for SmallestCostHolder<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> Ord for SmallestCostHolder<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

/// Iterator structure created by the `astar_bag` function.
#[derive(Clone)]
pub struct AstarSolution<N> {
    sinks: Vec<usize>,
    /// Index of the start vertex, which is where every path ends when walked backwards.
    start: usize,
    /// Whether any edge costing nothing was relaxed, which is what makes a vertex able to be
    /// its own optimal parent. Without one, the walk back cannot loop.
    may_loop: bool,
    parents: Vec<(N, Vec<usize>)>,
    current: Vec<Vec<usize>>,
    terminated: bool,
}

impl<N: Clone + Eq + Hash> AstarSolution<N> {
    /// Extend the partial path backwards until it reaches the start.
    ///
    /// Returns `false` if the choices made so far cannot be extended to the start, in which case
    /// the caller has to backtrack and try the next alternative.
    ///
    /// Two things make this more than a walk up the parent links. A parent already on the path
    /// being built is skipped, because an edge costing nothing makes a vertex an optimal parent
    /// of itself, or of a vertex it forms a zero-cost cycle with, and following those never
    /// ends. And the walk stops at the start vertex rather than at a vertex without parents,
    /// because a zero-cost cycle through the start gives the start parents of its own.
    /// Extend the partial path backwards until it reaches the start.
    ///
    /// Returns `false` if the choices made so far cannot be extended to the start, in which case
    /// the caller has to backtrack and try the next alternative.
    fn complete(&mut self) -> bool {
        if self.may_loop {
            self.complete_without_looping()
        } else {
            self.complete_directly();
            true
        }
    }

    /// The common case, where no edge costs nothing.
    ///
    /// A cycle among optimal parents needs every edge on it to cost nothing, so here the walk
    /// back cannot loop and every vertex can simply follow its parents to the start.
    fn complete_directly(&mut self) {
        loop {
            let ps = match self.current.last() {
                None => self.sinks.clone(),
                Some(last) => {
                    let tail = *last.last().unwrap();
                    if tail == self.start {
                        break;
                    }
                    self.parents(tail).clone()
                }
            };
            if ps.is_empty() {
                break;
            }
            self.current.push(ps);
        }
    }

    /// The case where some edge costs nothing.
    ///
    /// Such an edge makes a vertex an optimal parent of itself, or of a vertex it forms a
    /// zero-cost cycle with, so parents already on the path being built have to be skipped or
    /// the walk never ends. Doing that can leave a vertex with nowhere to go, which is a dead
    /// end for this combination of choices rather than a result.
    fn complete_without_looping(&mut self) -> bool {
        loop {
            let ps = match self.current.last() {
                None => self.sinks.clone(),
                Some(last) => {
                    let tail = *last.last().unwrap();
                    if tail == self.start {
                        return true;
                    }
                    self.parents(tail)
                        .iter()
                        .copied()
                        .filter(|p| !self.chosen().any(|c| c == *p))
                        .collect::<Vec<_>>()
                }
            };
            if ps.is_empty() {
                return false;
            }
            self.current.push(ps);
        }
    }

    /// The vertices picked so far, one per level, from the goal backwards.
    fn chosen(&self) -> impl Iterator<Item = usize> + '_ {
        self.current
            .iter()
            .filter_map(|level| level.last().copied())
    }

    fn next_vec(&mut self) {
        while self.current.pop_if(|v| v.len() == 1).is_some() {}
        self.current.last_mut().map(Vec::pop);
    }

    fn node(&self, i: usize) -> &N {
        &self.parents[i].0
    }

    fn parents(&self, i: usize) -> &Vec<usize> {
        &self.parents[i].1
    }
}

impl<N: Clone + Eq + Hash> Iterator for AstarSolution<N> {
    type Item = Vec<N>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.terminated {
                return None;
            }
            if !self.complete() {
                // This combination of choices cannot reach the start. Step to the next one and
                // try again, rather than reporting a path that stops short.
                self.next_vec();
                self.terminated = self.current.is_empty();
                continue;
            }
            let path = self
                .current
                .iter()
                .rev()
                .map(|v| v.last().copied().unwrap())
                .map(|i| self.node(i).clone())
                .collect::<Vec<_>>();
            self.next_vec();
            self.terminated = self.current.is_empty();
            return Some(path);
        }
    }
}

impl<N: Clone + Eq + Hash> FusedIterator for AstarSolution<N> {}
