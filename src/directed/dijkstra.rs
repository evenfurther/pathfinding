//! Compute a shortest path using the [Dijkstra search
//! algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).

use super::reverse_path;
use crate::FxIndexMap;
use indexmap::map::Entry::{Occupied, Vacant};
use num_traits::Zero;
use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

/// Compute a shortest path using the [Dijkstra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned along with its total cost, in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
///   from the node to the successor. This cost must be non-negative.
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
/// use pathfinding::prelude::dijkstra;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn successors(&self) -> Vec<(Pos, usize)> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///          .into_iter().map(|p| (p, 1)).collect()
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = dijkstra(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::dijkstra;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = dijkstra(&(1, 1),
///                       |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                      (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                                  .into_iter().map(|p| (p, 1)),
///                       |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
pub fn dijkstra<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    dijkstra_internal(start, &mut successors, &mut success)
}

pub(crate) fn dijkstra_internal<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    success: &mut FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let (parents, reached) = run_dijkstra(start, successors, success);
    reached.map(|target| {
        (
            reverse_path(&parents, |&(p, _)| p, target),
            parents.get_index(target).unwrap().1.1,
        )
    })
}

/// Compute a shortest path using a bidirectional variant of the [Dijkstra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
///
/// Two searches are run simultaneously: one forward from `start` following `successors`, and one
/// backward from `end` following `predecessors`. They progress in order of increasing cost until
/// they meet in the middle. On large graphs this often settles far fewer nodes than a single
/// unidirectional search, since two small frontiers are usually cheaper to grow than one large
/// one.
///
/// The shortest path from `start` to `end` is computed and returned along with its total cost, in
/// a `Some`. If no path can be found, `None` is returned instead.
///
/// - `start` is the starting node.
/// - `end` is the destination node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
///   from the node to the successor. This cost must be non-negative.
/// - `predecessors` returns a list of predecessors for a given node, along with the cost for
///   moving from the predecessor to the node. This cost must be non-negative, and for a given edge
///   it must match the cost reported by `successors`. For an undirected graph, where every edge can
///   be traversed in both directions at the same cost, the same closure can be used for both
///   `successors` and `predecessors`.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node.
///
/// # Example
///
/// We search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
/// moves. Knight moves are symmetrical, so the same closure describes both the successors and the
/// predecessors of a square.
///
/// ```
/// use pathfinding::prelude::dijkstra_bidirectional;
///
/// fn neighbours(&(x, y): &(i32, i32)) -> Vec<((i32, i32), usize)> {
///     vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///          (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///         .into_iter().map(|p| (p, 1)).collect()
/// }
///
/// let result = dijkstra_bidirectional(&(1, 1), &(4, 6), neighbours, neighbours);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
#[expect(clippy::missing_panics_doc)]
pub fn dijkstra_bidirectional<N, C, FS, IS, FP, IP>(
    start: &N,
    end: &N,
    mut successors: FS,
    mut predecessors: FP,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FS: FnMut(&N) -> IS,
    IS: IntoIterator<Item = (N, C)>,
    FP: FnMut(&N) -> IP,
    IP: IntoIterator<Item = (N, C)>,
{
    if start == end {
        return Some((vec![start.clone()], Zero::zero()));
    }

    let mut forward: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    forward.insert(start.clone(), (usize::MAX, Zero::zero()));
    let mut forward_queue = BinaryHeap::new();
    forward_queue.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });
    let mut forward_settled: FxHashSet<N> = FxHashSet::default();

    let mut backward: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    backward.insert(end.clone(), (usize::MAX, Zero::zero()));
    let mut backward_queue = BinaryHeap::new();
    backward_queue.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });
    let mut backward_settled: FxHashSet<N> = FxHashSet::default();

    // Best complete path found so far, as (total cost, meeting node). The meeting node is present
    // in both the `forward` and `backward` parent maps, so the full path can be rebuilt from it.
    let mut best: Option<(C, N)> = None;

    while !forward_queue.is_empty() && !backward_queue.is_empty() {
        if expand_bidirectional(
            &mut forward_queue,
            &mut forward,
            &mut forward_settled,
            &backward,
            &backward_settled,
            &mut successors,
            &mut best,
        ) {
            break;
        }
        if backward_queue.is_empty() {
            break;
        }
        if expand_bidirectional(
            &mut backward_queue,
            &mut backward,
            &mut backward_settled,
            &forward,
            &forward_settled,
            &mut predecessors,
            &mut best,
        ) {
            break;
        }
    }

    best.map(|(cost, meeting)| {
        // The forward half runs from `start` up to the meeting node.
        let meeting_index = forward.get_index_of(&meeting).unwrap();
        let mut path = reverse_path(&forward, |&(p, _)| p, meeting_index);
        // The backward half runs from the meeting node towards `end`, following backward parents.
        let mut parent = backward.get(&meeting).unwrap().0;
        while parent != usize::MAX {
            let (node, &(next, _)) = backward.get_index(parent).unwrap();
            path.push(node.clone());
            parent = next;
        }
        (path, cost)
    })
}

/// Perform a single expansion step of one side of a bidirectional Dijkstra search.
///
/// The next unsettled node with the smallest tentative cost is popped from `queue` and settled.
/// Its neighbours (given by `neighbours`) are relaxed into `parents`, and whenever a neighbour has
/// already been reached by the opposite search a complete path is available and recorded in `best`
/// if it improves on the current one.
///
/// Returns `true` when the popped node has already been settled by the opposite search, which means
/// the two frontiers have met and the best recorded path is optimal.
fn expand_bidirectional<N, C, FN, IN>(
    queue: &mut BinaryHeap<SmallestHolder<C>>,
    parents: &mut FxIndexMap<N, (usize, C)>,
    settled: &mut FxHashSet<N>,
    opposite: &FxIndexMap<N, (usize, C)>,
    opposite_settled: &FxHashSet<N>,
    neighbours: &mut FN,
    best: &mut Option<(C, N)>,
) -> bool
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let Some(SmallestHolder { cost, index }) = queue.pop() else {
        return false;
    };
    let node = {
        let (node, &(_, c)) = parents.get_index(index).unwrap();
        // A cheaper path to this node was found after this entry was queued: discard it.
        if cost > c {
            return false;
        }
        node.clone()
    };
    if !settled.insert(node.clone()) {
        // Already settled through an equal-cost entry.
        return false;
    }
    if opposite_settled.contains(&node) {
        // Both searches have settled this node: the best recorded path is optimal.
        return true;
    }
    for (neighbour, move_cost) in neighbours(&node) {
        let new_cost = cost + move_cost;
        let n;
        match parents.entry(neighbour) {
            Vacant(e) => {
                n = e.index();
                e.insert((index, new_cost));
            }
            Occupied(mut e) => {
                if e.get().1 > new_cost {
                    n = e.index();
                    e.insert((index, new_cost));
                } else {
                    continue;
                }
            }
        }
        queue.push(SmallestHolder {
            cost: new_cost,
            index: n,
        });
        // If the opposite search has already reached this neighbour, the two halves form a
        // complete path; keep it if it is the cheapest one seen so far.
        let neighbour = parents.get_index(n).unwrap().0;
        if let Some(&(_, opposite_cost)) = opposite.get(neighbour) {
            let total = new_cost + opposite_cost;
            let improved = match best {
                Some((current, _)) => total < *current,
                None => true,
            };
            if improved {
                *best = Some((total, neighbour.clone()));
            }
        }
    }
    false
}

/// Determine all reachable nodes from a starting point as well as the
/// minimum cost to reach them and a possible optimal parent node
/// using the [Dijkstra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
///   from the node to the successor.
///
/// The result is a map where every reachable node (not including `start`) is associated with
/// an optimal parent node and a cost from the start node.
///
/// The [`build_path`] function can be used to build a full path from the starting point to one
/// of the reachable targets.
///
/// # Example
///
/// We use a graph of integer nodes from 1 to 9, each node leading to its double and the value
/// after it with a cost of 10 at every step.
///
/// ```
/// use pathfinding::prelude::dijkstra_all;
///
/// fn successors(&n: &u32) -> Vec<(u32, usize)> {
///   if n <= 4 { vec![(n*2, 10), (n*2+1, 10)] } else { vec![] }
/// }
///
/// let reachables = dijkstra_all(&1, successors);
/// assert_eq!(reachables.len(), 8);
/// assert_eq!(reachables[&2], (1, 10));  // 1 -> 2
/// assert_eq!(reachables[&3], (1, 10));  // 1 -> 3
/// assert_eq!(reachables[&4], (2, 20));  // 1 -> 2 -> 4
/// assert_eq!(reachables[&5], (2, 20));  // 1 -> 2 -> 5
/// assert_eq!(reachables[&6], (3, 20));  // 1 -> 3 -> 6
/// assert_eq!(reachables[&7], (3, 20));  // 1 -> 3 -> 7
/// assert_eq!(reachables[&8], (4, 30));  // 1 -> 2 -> 4 -> 8
/// assert_eq!(reachables[&9], (4, 30));  // 1 -> 2 -> 4 -> 9
/// ```
pub fn dijkstra_all<N, C, FN, IN>(start: &N, successors: FN) -> HashMap<N, (N, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    dijkstra_partial(start, successors, |_| false).0
}

/// Determine some reachable nodes from a starting point as well as the minimum cost to
/// reach them and a possible optimal parent node
/// using the [Dijkstra search algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
///   from the node to the successor.
/// - `stop` is a function which is called every time a node is examined (including `start`).
///   A `true` return value will stop the algorithm.
///
/// The result is a map where every node examined before the algorithm stopped (not including
/// `start`) is associated with an optimal parent node and a cost from the start node, as well
/// as the node which caused the algorithm to stop if any.
///
/// The [`build_path`] function can be used to build a full path from the starting point to one
/// of the reachable targets.
#[expect(clippy::missing_panics_doc)]
pub fn dijkstra_partial<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut stop: FS,
) -> (HashMap<N, (N, C)>, Option<N>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let (parents, reached) = run_dijkstra(start, &mut successors, &mut stop);
    (
        parents
            .iter()
            .skip(1)
            .map(|(n, (p, c))| (n.clone(), (parents.get_index(*p).unwrap().0.clone(), *c))) // unwrap() cannot fail
            .collect(),
        reached.map(|i| parents.get_index(i).unwrap().0.clone()),
    )
}

fn run_dijkstra<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    stop: &mut FS,
) -> (FxIndexMap<N, (usize, C)>, Option<usize>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));
    let mut target_reached = None;
    while let Some(SmallestHolder { cost, index }) = to_see.pop() {
        let successors = {
            let (node, &(_, c)) = parents.get_index(index).unwrap();
            if stop(node) {
                target_reached = Some(index);
                break;
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
            let n;
            match parents.entry(successor) {
                Vacant(e) => {
                    n = e.index();
                    e.insert((index, new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        n = e.index();
                        e.insert((index, new_cost));
                    } else {
                        continue;
                    }
                }
            }

            to_see.push(SmallestHolder {
                cost: new_cost,
                index: n,
            });
        }
    }
    (parents, target_reached)
}

/// Build a path leading to a target according to a parents map, which must
/// contain no loop. This function can be used after [`dijkstra_all`] or
/// [`dijkstra_partial`] to build a path from a starting point to a reachable target.
///
/// - `target` is reachable target.
/// - `parents` is a map containing an optimal parent (and an associated
///   cost which is ignored here) for every reachable node.
///
/// This function returns a vector with a path from the farthest parent up to
/// `target`, including `target` itself.
///
/// # Panics
///
/// If the `parents` map contains a loop, this function will attempt to build
/// a path of infinite length and panic when memory is exhausted.
///
/// # Example
///
/// We will use a `parents` map to indicate that each integer from 2 to 100
/// parent is its integer half (2 -> 1, 3 -> 1, 4 -> 2, etc.)
///
/// ```
/// use pathfinding::prelude::build_path;
///
/// let parents = (2..=100).map(|n| (n, (n/2, 1))).collect();
/// assert_eq!(vec![1, 2, 4, 9, 18], build_path(&18, &parents));
/// assert_eq!(vec![1], build_path(&1, &parents));
/// assert_eq!(vec![101], build_path(&101, &parents));
/// ```
#[expect(clippy::implicit_hasher)]
pub fn build_path<N, C>(target: &N, parents: &HashMap<N, (N, C)>) -> Vec<N>
where
    N: Eq + Hash + Clone,
{
    let mut rev = vec![target.clone()];
    let mut next = target.clone();
    while let Some((parent, _)) = parents.get(&next) {
        rev.push(parent.clone());
        next = parent.clone();
    }
    rev.reverse();
    rev
}

struct SmallestHolder<K> {
    cost: K,
    index: usize,
}

impl<K: PartialEq> PartialEq for SmallestHolder<K> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<K: PartialEq> Eq for SmallestHolder<K> {}

impl<K: Ord> PartialOrd for SmallestHolder<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> Ord for SmallestHolder<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

/// Struct returned by [`dijkstra_reach`].
pub struct DijkstraReachable<N, C, FN> {
    to_see: BinaryHeap<SmallestHolder<C>>,
    seen: FxHashSet<usize>,
    parents: FxIndexMap<N, (usize, C)>,
    successors: FN,
}

/// Information about a node reached by [`dijkstra_reach`].
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DijkstraReachableItem<N, C> {
    /// The node that was reached by [`dijkstra_reach`].
    pub node: N,
    /// The previous node that the current node came from.
    /// If the node is the first node, there will be no parent.
    pub parent: Option<N>,
    /// The total cost from the starting node.
    pub total_cost: C,
}

impl<N, C, FN, IN> Iterator for DijkstraReachable<N, C, FN>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    type Item = DijkstraReachableItem<N, C>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(SmallestHolder { cost, index }) = self.to_see.pop() {
            if !self.seen.insert(index) {
                continue;
            }
            let item;
            let successors = {
                let (node, &(parent_index, total_cost)) = self.parents.get_index(index).unwrap();
                item = Some(DijkstraReachableItem {
                    node: node.clone(),
                    parent: self.parents.get_index(parent_index).map(|x| x.0.clone()),
                    total_cost,
                });
                (self.successors)(node)
            };
            for (successor, move_cost) in successors {
                let new_cost = cost + move_cost;
                let n;
                match self.parents.entry(successor) {
                    Vacant(e) => {
                        n = e.index();
                        e.insert((index, new_cost));
                    }
                    Occupied(mut e) => {
                        if e.get().1 > new_cost {
                            n = e.index();
                            e.insert((index, new_cost));
                        } else {
                            continue;
                        }
                    }
                }

                self.to_see.push(SmallestHolder {
                    cost: new_cost,
                    index: n,
                });
            }
            return item;
        }

        None
    }
}

/// Visit all nodes that are reachable from a start node. The node
/// will be visited in order of cost, with the closest nodes first.
///
/// The `successors` function receives the current node, and returns
/// an iterator of successors associated with their move cost.
pub fn dijkstra_reach<N, C, FN, IN>(start: &N, successors: FN) -> DijkstraReachable<N, C, FN>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });

    let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));

    let seen = FxHashSet::default();

    DijkstraReachable {
        to_see,
        seen,
        parents,
        successors,
    }
}
