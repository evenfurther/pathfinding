use num_traits::Zero;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cmp::Ordering;
use std::hash::Hash;
use std::rc::Rc;
use std::borrow::Borrow;

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
/// use pathfinding::astar;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn distance(&self, other: &Pos) -> usize {
///     ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
///   }
///
///   fn neighbours(&self) -> Vec<(Pos, usize)> {
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
/// use pathfinding::astar;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = astar(&(1, 1),
///                    |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                   (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                               .into_iter().map(|p| (p, 1)),
///                    |&(x, y)| ((x-GOAL.0).abs() + (y-GOAL.0).abs()) / 3,
///                    |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```

pub fn astar<N, C, FN, IN, FH, FS>(
    start: &N,
    neighbours: FN,
    heuristic: FH,
    success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: Fn(&N) -> C,
    FS: Fn(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: heuristic(start),
        cost: Zero::zero(),
        payload: (Zero::zero(), start.clone()),
    });
    let mut parents: HashMap<N, (N, C)> = HashMap::new();
    while let Some(SmallestCostHolder {
        payload: (cost, node),
        ..
    }) = to_see.pop()
    {
        if success(&node) {
            let parents = parents.into_iter().map(|(n, (p, _))| (n, p)).collect();
            return Some((reverse_path(&parents, node), cost));
        }
        // We may have inserted a node several time into the binary heap if we found
        // a better way to access it. Ensure that we are currently dealing with the
        // best path and discard the others.
        if let Some(&(_, c)) = parents.get(&node) {
            if cost > c {
                continue;
            }
        }
        for (neighbour, move_cost) in neighbours(&node) {
            let new_cost = cost + move_cost;
            if neighbour != *start {
                let mut inserted = true;
                match parents.entry(neighbour.clone()) {
                    Vacant(e) => {
                        e.insert((node.clone(), new_cost));
                    }
                    Occupied(mut e) => if e.get().1 > new_cost {
                        e.insert((node.clone(), new_cost));
                    } else {
                        inserted = false;
                    },
                };
                if inserted {
                    let new_predicted_cost = new_cost + heuristic(&neighbour);
                    to_see.push(SmallestCostHolder {
                        estimated_cost: new_predicted_cost,
                        cost: cost,
                        payload: (new_cost, neighbour),
                    });
                }
            }
        }
    }
    None
}

#[derive(Eq, Hash, PartialEq)]
struct PathNode<N>
where
    N: Clone,
{
    node: Rc<N>,
    parent: Option<Rc<PathNode<N>>>,
}

/// Compute all shortest paths using the [A* search
/// algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm). Whereas `astar`
/// (non-deterministically) returns a single shortest path, `astar_bag` returns all shortest paths
/// (in a non-deterministic order).
///
/// The shortest paths starting from `start` up to a node for which `success` returns `true` are
/// computed and returned in a `Vec` along with the cost (which, by definition, is the same for
/// each shortest path). If no paths are found, the vector will be empty.
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
    neighbours: FN,
    heuristic: FH,
    success: FS,
) -> (Vec<Vec<N>>, C)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: Fn(&N) -> C,
    FS: Fn(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    let mut out = Vec::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: heuristic(start),
        cost: Zero::zero(),
        payload: Rc::new(PathNode {
            node: Rc::new(start.clone()),
            parent: None,
        }),
    });
    let mut lowest_cost = HashMap::new();
    let mut min_cost = None;
    while let Some(SmallestCostHolder {
        cost,
        estimated_cost,
        payload,
    }) = to_see.pop()
    {
        let pn: &PathNode<N> = Rc::borrow(&payload);
        if let Some(mc) = min_cost {
            if estimated_cost > mc {
                break;
            }
        }
        if success(pn.node.borrow()) {
            min_cost = Some(cost);
            out.push(mk_path(&payload));
            continue;
        }
        // We may have inserted the same node several times into the binary heap: if we found a
        // lower-cost way of accessing it then there's no point in exploring higher-cost versions
        // (since we will already have explored the lower-cost versions).
        if let Some(c) = lowest_cost.get(&pn.node) {
            if cost > *c {
                continue;
            }
        }
        for (neighbour, move_cost) in neighbours(pn.node.borrow()) {
            let new_cost = cost + move_cost;
            if neighbour != *start {
                let new_predicted_cost = new_cost + heuristic(&neighbour);

                let nrc = Rc::new(neighbour);
                match lowest_cost.entry(Rc::clone(&nrc)) {
                    Vacant(e) => {
                        e.insert(new_cost);
                    }
                    Occupied(mut e) => if *e.get() > new_cost {
                        e.insert(new_cost);
                    },
                }

                to_see.push(SmallestCostHolder {
                    estimated_cost: new_predicted_cost,
                    cost: new_cost,
                    payload: Rc::new(PathNode {
                        node: nrc,
                        parent: Some(Rc::clone(&payload)),
                    }),
                });
            }
        }
    }
    (out, min_cost.unwrap_or_else(Zero::zero))
}

fn mk_path<N>(mut pl: &Rc<PathNode<N>>) -> Vec<N>
where
    N: Clone,
{
    let mut path = Vec::new();
    loop {
        let cur: &PathNode<N> = Rc::borrow(pl);
        let n: &N = cur.node.borrow();
        path.push(n.clone());
        match cur.parent {
            Some(ref p) => pl = p,
            None => break,
        }
    }
    path.reverse();
    path
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
