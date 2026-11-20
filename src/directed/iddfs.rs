//! Compute a shortest path using the [iterative deepening depth-first search
//! algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search).

/// Compute a shortest path using the [iterative deepening depth-first search
/// algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
///   a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node. Note that the start node ownership
/// is taken by `iddfs` as no clones are made.
///
/// # Example
///
/// We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
/// moves.
///
/// The first version uses an explicit type `Pos` on which the required traits are derived.
///
/// ```
/// use pathfinding::prelude::iddfs;
///
/// #[derive(Eq, PartialEq)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn successors(&self) -> Vec<Pos> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = iddfs(Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").len(), 5);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::iddfs;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = iddfs((1, 1),
///                  |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                 (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)],
///                  |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").len(), 5);
/// ```
pub fn iddfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    let mut path = vec![start];

    let mut current_max_depth: usize = 1;

    loop {
        match step(&mut path, &mut successors, &mut success, current_max_depth) {
            Path::FoundOptimum => return Some(path),
            Path::NoneAtThisDepth => current_max_depth += 1,
            Path::Impossible => return None,
        }
    }
}

#[derive(Debug)]
enum Path {
    FoundOptimum,
    Impossible,
    NoneAtThisDepth,
}

/// Explore every path of at most `depth` nodes starting with `path`, which holds a single
/// node, extending `path` up to the goal if one is found.
///
/// This is a depth-first search, written with an explicit stack rather than recursively so that
/// a long path cannot overflow the call stack. `path` and `stack` are kept in step: `stack[i]`
/// holds the successors of `path[i]` left to explore, and the result found below it so far.
fn step<N, FN, IN, FS>(
    path: &mut Vec<N>,
    successors: &mut FN,
    success: &mut FS,
    depth: usize,
) -> Path
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    let mut stack: Vec<(IN::IntoIter, Path)> = Vec::new();
    loop {
        // Enter the last node of `path`, the `path.len()`-th node of the path. Its result is
        // known straight away unless it has successors to explore.
        let mut result = if path.len() > depth {
            Path::NoneAtThisDepth
        } else if success(path.last().unwrap()) {
            return Path::FoundOptimum;
        } else {
            let successors = successors(path.last().unwrap()).into_iter();
            stack.push((successors, Path::Impossible));
            match next_node(path, &mut stack) {
                Some(n) => {
                    path.push(n);
                    continue;
                }
                None => stack.pop().unwrap().1, // Cannot fail: a frame was just pushed
            }
        };
        // Leave every node whose successors are exhausted, until one has a successor to enter.
        loop {
            let Some((_, parent_result)) = stack.last_mut() else {
                return result;
            };
            path.pop();
            if matches!(result, Path::NoneAtThisDepth) {
                *parent_result = Path::NoneAtThisDepth;
            }
            if let Some(n) = next_node(path, &mut stack) {
                path.push(n);
                break;
            }
            result = stack.pop().unwrap().1; // Cannot fail: the stack was just checked
        }
    }
}

/// Next successor of the node on top of `stack` not already on `path`, if any.
fn next_node<N, I>(path: &[N], stack: &mut [(I, Path)]) -> Option<N>
where
    N: Eq,
    I: Iterator<Item = N>,
{
    // Cannot fail: callers always have a frame on the stack.
    stack.last_mut().unwrap().0.find(|n| !path.contains(n))
}
