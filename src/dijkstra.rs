use astar::astar;
use num_traits::Zero;
use std::hash::Hash;

/// Compute a shortest path using the [Dijsktra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned along with its total cost, in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `neighbours` returns a list of neighbours for a given node, along with the cost for moving
/// from the node to the neighbour.
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
/// use pathfinding::dijkstra;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn neighbours(&self) -> Vec<(Pos, usize)> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///          .into_iter().map(|p| (p, 1)).collect()
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = dijkstra(&Pos(1, 1), |p| p.neighbours(), |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::dijkstra;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = dijkstra(&(1, 1),
///                       |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                      (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                                  .into_iter().map(|p| (p, 1)),
///                       |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
pub fn dijkstra<N, C, FN, IN, FS>(start: &N, neighbours: FN, success: FS) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: Fn(&N) -> bool,
{
    let zero = Zero::zero();
    astar(start, neighbours, |_| zero, success)
}
