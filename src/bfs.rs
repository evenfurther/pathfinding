use dijkstra::dijkstra;
use std::hash::Hash;

pub fn bfs<N, FN, IN, FS>(start: &N, neighbours: FN, success: FS) -> Option<Vec<N>>
    where N: Eq + Hash + Clone,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = N>,
          FS: Fn(&N) -> bool
{
    dijkstra(start, |n| neighbours(n).into_iter().map(|n| (n, 1)), success).map(|(path, _)| path)
}
