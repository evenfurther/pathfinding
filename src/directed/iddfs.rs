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
/// a dynamic solution instead of a fixed node.
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
    if depth == 0 {
        Path::NoneAtThisDepth
    } else if success(path.last().unwrap()) {
        Path::FoundOptimum
    } else {
        let successors_it = successors(path.last().unwrap());

        let mut best_result = Path::Impossible;

        for n in successors_it {
            if !path.contains(&n) {
                path.push(n);
                match step(path, successors, success, depth - 1) {
                    Path::FoundOptimum => return Path::FoundOptimum,
                    Path::NoneAtThisDepth => best_result = Path::NoneAtThisDepth,
                    Path::Impossible => (),
                }
                path.pop();
            }
        }

        best_result
    }
}
