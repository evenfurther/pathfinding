//! Compute a shortest path using the [IDA* search
//! algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_A*).

use num_traits::Zero;

/// Compute a shortest path using the [IDA* search
/// algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_A*).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned along with its total cost, in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
/// from the node to the successor.
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
/// use pathfinding::prelude::{absdiff, idastar};
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn distance(&self, other: &Pos) -> u32 {
///     (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
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
/// use pathfinding::prelude::{absdiff, idastar};
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = idastar(&(1, 1),
///                    |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                   (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                               .into_iter().map(|p| (p, 1)),
///                    |&(x, y)| (absdiff(x, GOAL.0) + absdiff(y, GOAL.1)) / 3,
///                    |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
pub fn idastar<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut bound = heuristic(start);
    let mut path = vec![start.clone()];
    loop {
        match search(
            &mut path,
            Zero::zero(),
            bound,
            &mut successors,
            &mut heuristic,
            &mut success,
        ) {
            Path::Found(path, cost) => return Some((path, cost)),
            Path::Minimum(min) => {
                if bound == min {
                    return None;
                }
                bound = min;
            }
            Path::Impossible => return None,
        }
    }
}

enum Path<N, C> {
    Found(Vec<N>, C),
    Minimum(C),
    Impossible,
}

fn search<N, C, FN, IN, FH, FS>(
    path: &mut Vec<N>,
    cost: C,
    bound: C,
    successors: &mut FN,
    heuristic: &mut FH,
    success: &mut FS,
) -> Path<N, C>
where
    N: Eq + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let neighbs = {
        let start = &path[path.len() - 1];
        let f = cost + heuristic(start);
        if f > bound {
            return Path::Minimum(f);
        }
        if success(start) {
            return Path::Found(path.clone(), f);
        }
        let mut neighbs = successors(start)
            .into_iter()
            .filter_map(|(n, c)| {
                if path.contains(&n) {
                    None
                } else {
                    let h = heuristic(&n);
                    Some((n, c, c + h))
                }
            })
            .collect::<Vec<_>>();
        neighbs.sort_unstable_by_key(|&(_, _, c)| c);
        neighbs
    };
    let mut min = None;
    for (node, extra, _) in neighbs {
        path.push(node);
        match search(path, cost + extra, bound, successors, heuristic, success) {
            found_path @ Path::Found(_, _) => return found_path,
            Path::Minimum(m) => match min {
                None => min = Some(m),
                Some(n) if m < n => min = Some(m),
                Some(_) => (),
            },
            Path::Impossible => (),
        }
        path.pop();
    }
    match min {
        Some(m) => Path::Minimum(m),
        None => Path::Impossible,
    }
}
