use num::{Bounded, Zero};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::mem;

use super::reverse_path;

fn remove<T: Eq>(v: &mut VecDeque<T>, e: &T) -> bool {
    if let Some(index) = v.iter().position(|x| x == e) {
        v.remove(index);
        true
    } else {
        false
    }
}

/// Compute a shortest path using the [Fringe search
/// algorithm](https://en.wikipedia.org/wiki/Fringe_search).
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
/// use pathfinding::fringe;
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
/// let result = fringe(&Pos(1, 1), |p| p.neighbours(), |p| p.distance(&GOAL) / 3,
///                     |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::fringe;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = fringe(&(1, 1),
///                     |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                    (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                                .into_iter().map(|p| (p, 1)),
///                     |&(x, y)| ((x-GOAL.0).abs() + (y-GOAL.0).abs()) / 3,
///                     |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
pub fn fringe<N, C, FN, IN, FH, FS>(start: &N,
                                    neighbours: FN,
                                    heuristic: FH,
                                    success: FS)
                                    -> Option<(Vec<N>, C)>
    where N: Eq + Hash + Clone,
          C: Bounded + Zero + Ord + Copy,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = (N, C)>,
          FH: Fn(&N) -> C,
          FS: Fn(&N) -> bool
{
    let mut now = VecDeque::new();
    let mut later = VecDeque::new();
    let mut costs: HashMap<N, C> = HashMap::new();
    let mut parents: HashMap<N, N> = HashMap::new();
    let mut flimit = heuristic(start);
    now.push_back(start.clone());
    costs.insert(start.clone(), Zero::zero());

    loop {
        if now.is_empty() {
            return None;
        }
        let mut fmin = C::max_value();
        while let Some(node) = now.pop_front() {
            let g = costs[&node];
            let f = g + heuristic(&node);
            if f > flimit {
                if f < fmin {
                    fmin = f;
                }
                later.push_back(node);
                continue;
            }
            if success(&node) {
                return Some((reverse_path(parents, node), g));
            }
            for (neighbour, cost) in neighbours(&node) {
                let g_neighbour = g + cost;
                if let Some(&old_g) = costs.get(&neighbour) {
                    if old_g <= g_neighbour {
                        continue;
                    }
                }
                if !remove(&mut later, &neighbour) { remove(&mut now, &neighbour); }
                now.push_front(neighbour.clone());
                costs.insert(neighbour.clone(), g_neighbour);
                parents.insert(neighbour, node.clone());
            }
        }
        mem::swap(&mut now, &mut later);
        flimit = fmin;
    }
}
