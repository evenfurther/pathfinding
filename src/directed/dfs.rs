//! Compute a path using the [depth-first search
//! algorithm](https://en.wikipedia.org/wiki/Depth-first_search).

/// Compute a path using the [depth-first search
/// algorithm](https://en.wikipedia.org/wiki/Depth-first_search).
/// The path starts from `start` up to a node for which `success` returns `true` is computed and
/// returned along with its total cost, in a `Some`. If no path can be found, `None` is returned
/// instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, which will be tried in order.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
/// a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node. Note that the start node ownership
/// is taken by `dfs` as no clones are made.
///
/// # Example
///
/// We will search a way to get from 1 to 17 while only adding 1 or multiplying the number by
/// itself.
///
/// If we put the adder first, an adder-only solution will be found:
///
/// ```
/// use pathfinding::prelude::dfs;
///
/// assert_eq!(dfs(1, |&n| vec![n+1, n*n].into_iter().filter(|&x| x <= 17), |&n| n == 17),
///            Some((1..18).collect()));
/// ```
///
/// However, if we put the multiplier first, a shorter solution will be explored first:
///
/// ```
/// use pathfinding::prelude::dfs;
///
/// assert_eq!(dfs(1, |&n| vec![n*n, n+1].into_iter().filter(|&x| x <= 17), |&n| n == 17),
///            Some(vec![1, 2, 4, 16, 17]));
/// ```
pub fn dfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    let mut path = vec![start];
    if step(&mut path, &mut successors, &mut success) {
        Some(path)
    } else {
        None
    }
}

fn step<N, FN, IN, FS>(path: &mut Vec<N>, successors: &mut FN, success: &mut FS) -> bool
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    if success(path.last().unwrap()) {
        true
    } else {
        let successors_it = successors(path.last().unwrap());
        for n in successors_it {
            if !path.contains(&n) {
                path.push(n);
                if step(path, successors, success) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }
}
