use astar::astar;
use num::Zero;
use std::hash::Hash;

pub fn dijkstra<N, C, FN, IN, FS>(start: &N, neighbours: FN, success: FS) -> Option<(Vec<N>, C)>
    where N: Eq + Hash + Clone,
          C: Zero + Ord + Copy,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = (N, C)>,
          FS: Fn(&N) -> bool
{
    let zero = Zero::zero();
    astar(start, neighbours, |_| zero, success)
}
