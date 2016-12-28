fn step<N, FN, IN, FS>(path: &mut Vec<N>, neighbours: &FN, success: &FS) -> bool
    where N: Eq + Clone,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = N>,
          FS: Fn(&N) -> bool
{
    let node = path.last().unwrap().clone();
    if success(&node) {
        true
    } else {
        for n in neighbours(&node) {
            if !path.contains(&n) {
                path.push(n);
                if step(path, neighbours, success) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }
}

/// Compute a path using the [depth-first search
/// algorithm](https://en.wikipedia.org/wiki/Depth-first_search).
///
/// The path starts from `start` up to a node for which `success` returns `true` is computed and
/// returned along with its total cost, in a `Some`. If no path can be found, `None` is returned
/// instead.
///
/// - `start` is the starting node.
/// - `neighbours` returns a list of neighbours for a given node, which will be tried in order.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
/// a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node.
///
/// # Example
///
/// We will search a way to get from 1 to 17 while only adding 1 or multiplying the number by
/// itself.
///
/// If we put the adder first, an adder-only solution will be found:
///
/// ```
/// use pathfinding::dfs;
///
/// assert_eq!(dfs(&1, |&n| vec![n+1, n*n].into_iter().filter(|&x| x <= 17), |&n| n == 17),
///            Some((1..18).collect()));
/// ```
///
/// However, if we put the multiplier first, a shorter solution will be explored first:
///
/// ```
/// use pathfinding::dfs;
///
/// assert_eq!(dfs(&1, |&n| vec![n*n, n+1].into_iter().filter(|&x| x <= 17), |&n| n == 17),
///            Some(vec![1, 2, 4, 16, 17]));
/// ```
pub fn dfs<N, FN, IN, FS>(start: &N, neighbours: FN, success: FS) -> Option<Vec<N>>
    where N: Eq + Clone,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = N>,
          FS: Fn(&N) -> bool
{
    let mut path = vec![start.clone()];
    if step(&mut path, &neighbours, &success) {
        Some(path)
    } else {
        None
    }
}
