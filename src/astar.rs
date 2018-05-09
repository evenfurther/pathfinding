//! Compute a shortest path (or all shorted paths) using the [A* search
//! algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).

use indexmap::map::Entry::{Occupied, Vacant};
use indexmap::IndexMap;
use num_traits::Zero;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::usize;

use super::reverse_path;

/// Compute a shortest path using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned along with its total cost, in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `neighbours` returns a list of neighbours for a given node, along with the cost for moving
/// from the node to the neighbour.
/// - `heuristic` returns an approximation of the cost from a given node to the goal. The
/// approximation must not be greater than the real cost, or a wrong shortest path may be returned.
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
/// use pathfinding::astar::astar;
/// use pathfinding::utils::absdiff;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn distance(&self, other: &Pos) -> u32 {
///     (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
///   }
///
///   fn neighbours(&self) -> Vec<(Pos, u32)> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///          .into_iter().map(|p| (p, 1)).collect()
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = astar(&Pos(1, 1), |p| p.neighbours(), |p| p.distance(&GOAL) / 3,
///                    |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::astar::astar;
/// use pathfinding::utils::absdiff;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = astar(&(1, 1),
///                    |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                   (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                               .into_iter().map(|p| (p, 1)),
///                    |&(x, y)| absdiff(x, GOAL.0) + absdiff(y, GOAL.1),
///                    |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
pub fn astar<N, C, FN, IN, FH, FS>(
    start: &N,
    mut neighbours: FN,
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
        estimated_cost: heuristic(start),
        cost: Zero::zero(),
        payload: (Zero::zero(), 0),
    });
    let mut parents: IndexMap<N, (usize, C)> = IndexMap::new();
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));
    while let Some(SmallestCostHolder {
        payload: (cost, i), ..
    }) = to_see.pop()
    {
        let neighbours = {
            let (node, &(_, c)) = parents.get_index(i).unwrap();
            if success(node) {
                let path = reverse_path(&parents, |&(p, _)| p, i);
                return Some((path, cost));
            }
            // We may have inserted a node several time into the binary heap if we found
            // a better way to access it. Ensure that we are currently dealing with the
            // best path and discard the others.
            if cost > c {
                continue;
            }
            neighbours(node)
        };
        for (neighbour, move_cost) in neighbours {
            let new_cost = cost + move_cost;
            let h; // heuristic(&neighbour)
            let n; // index for neighbour
            match parents.entry(neighbour) {
                Vacant(e) => {
                    h = heuristic(e.key());
                    n = e.index();
                    e.insert((i, new_cost));
                }
                Occupied(mut e) => if e.get().1 > new_cost {
                    h = heuristic(e.key());
                    n = e.index();
                    e.insert((i, new_cost));
                } else {
                    continue;
                },
            }

            to_see.push(SmallestCostHolder {
                estimated_cost: new_cost + h,
                cost,
                payload: (new_cost, n),
            });
        }
    }
    None
}

/// Compute all shortest paths using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm). Whereas `astar`
/// (non-deterministic-ally) returns a single shortest path, `astar_bag` returns all shortest paths
/// (in a non-deterministic order).
///
/// The shortest paths starting from `start` up to a node for which `success` returns `true` are
/// computed and returned in an iterator along with the cost (which, by definition, is the same for
/// each shortest path), wrapped in a `Some`. If no paths are found, `None` is returned.
///
/// - `start` is the starting node.
/// - `neighbours` returns a list of neighbours for a given node, along with the cost for moving
/// from the node to the neighbour.
/// - `heuristic` returns an approximation of the cost from a given node to the goal. The
/// approximation must not be greater than the real cost, or a wrong shortest path may be returned.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
/// a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// Each path comprises both the start and an end node. Note that while every path shares the same
/// start node, different paths may have different end nodes.
pub fn astar_bag<N, C, FN, IN, FH, FS>(
    start: &N,
    mut neighbours: FN,
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
    let mut sinks = HashSet::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: heuristic(start),
        cost: Zero::zero(),
        payload: (Zero::zero(), 0),
    });
    let mut parents: IndexMap<N, (HashSet<usize>, C)> = IndexMap::new();
    parents.insert(start.clone(), (HashSet::new(), Zero::zero()));
    while let Some(SmallestCostHolder {
        payload: (cost, i),
        estimated_cost,
        ..
    }) = to_see.pop()
    {
        if let Some(min_cost) = min_cost {
            if estimated_cost > min_cost {
                break;
            }
        }
        let neighbours = {
            let (node, &(_, c)) = parents.get_index(i).unwrap();
            if success(node) {
                min_cost = Some(cost);
                sinks.insert(i);
            }
            // We may have inserted a node several time into the binary heap if we found
            // a better way to access it. Ensure that we are currently dealing with the
            // best path and discard the others.
            if cost > c {
                continue;
            }
            neighbours(node)
        };
        for (neighbour, move_cost) in neighbours {
            let new_cost = cost + move_cost;
            let h; // heuristic(&neighbour)
            let n; // index for neighbour
            match parents.entry(neighbour) {
                Vacant(e) => {
                    h = heuristic(e.key());
                    n = e.index();
                    let mut p = HashSet::new();
                    p.insert(i);
                    e.insert((p, new_cost));
                }
                Occupied(mut e) => if e.get().1 > new_cost {
                    h = heuristic(e.key());
                    n = e.index();
                    let s = e.get_mut();
                    s.0.clear();
                    s.0.insert(i);
                    s.1 = new_cost;
                } else {
                    if e.get().1 == new_cost {
                        // New parent with an identical cost, this is not
                        // considered as an insertion.
                        e.get_mut().0.insert(i);
                    }
                    continue;
                },
            }

            to_see.push(SmallestCostHolder {
                estimated_cost: new_cost + h,
                cost,
                payload: (new_cost, n),
            });
        }
    }

    min_cost.map(|cost| {
        let parents = parents
            .into_iter()
            .map(|(k, (ps, _))| (k, ps.into_iter().collect()))
            .collect();
        (
            AstarSolution {
                sinks: sinks.into_iter().collect(),
                parents,
                current: vec![],
                terminated: false,
            },
            cost,
        )
    })
}

/// Compute all shortest paths using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm). Whereas `astar`
/// (non-deterministic-ally) returns a single shortest path, `astar_bag` returns all shortest paths
/// (in a non-deterministic order).
///
/// This is a utility function which collects the results of the `astar_bag` function into a
/// vector. Most of the time, it is more appropriate to use `astar_bag` directly.
///
/// ### Warning
///
/// The number of results with the same value might be very large in some graphs. Use with caution.
pub fn astar_bag_collect<N, C, FN, IN, FH, FS>(
    start: &N,
    neighbours: FN,
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
    astar_bag(start, neighbours, heuristic, success)
        .map(|(solutions, cost)| (solutions.collect(), cost))
}

struct SmallestCostHolder<K, P> {
    estimated_cost: K,
    cost: K,
    payload: P,
}

impl<K: PartialEq, P> PartialEq for SmallestCostHolder<K, P> {
    fn eq(&self, other: &SmallestCostHolder<K, P>) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl<K: PartialEq, P> Eq for SmallestCostHolder<K, P> {}

impl<K: Ord, P> PartialOrd for SmallestCostHolder<K, P> {
    fn partial_cmp(&self, other: &SmallestCostHolder<K, P>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, P> Ord for SmallestCostHolder<K, P> {
    fn cmp(&self, other: &SmallestCostHolder<K, P>) -> Ordering {
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
    parents: Vec<(N, Vec<usize>)>,
    current: Vec<Vec<usize>>,
    terminated: bool,
}

unsafe impl<N: Send> Send for AstarSolution<N> {}

impl<N: Clone + Eq + Hash> AstarSolution<N> {
    fn complete(&mut self) {
        loop {
            let ps = match self.current.last() {
                None => self.sinks.clone(),
                Some(last) => {
                    let &top = last.last().unwrap();
                    self.parents(top).clone()
                }
            };
            if ps.is_empty() {
                break;
            }
            self.current.push(ps);
        }
    }

    fn next_vec(&mut self) {
        while self.current.last().map(|v| v.len()) == Some(1) {
            self.current.pop();
        }
        self.current.last_mut().map(|v| v.pop());
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
        if self.terminated {
            return None;
        }
        self.complete();
        let path = self.current
            .iter()
            .rev()
            .map(|v| v.last().cloned().unwrap())
            .map(|i| self.node(i).clone())
            .collect::<Vec<_>>();
        self.next_vec();
        self.terminated = self.current.is_empty();
        Some(path)
    }
}
