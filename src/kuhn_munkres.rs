use ndarray::Array2;
use num_traits::{Signed, Zero};
use fixedbitset::FixedBitSet;
use std::iter::Sum;

/// Compute the maximum matching between two disjoints sets of vertices
/// using the
/// [Kuhn-Munkres algorithm](https://en.wikipedia.org/wiki/Hungarian_algorithm)
/// (also known as Hungarian algorithm).
///
/// The weights between the first and second sets are given into the
/// `weights` square matrix. The first axis is indexed by the X set,
/// and the second axis by the Y set. The return value is a pair with
/// the total assignments weight, and a vector containing indices in
/// the Y set for every vertex in the X set.
///
/// This algorithm executes in O(n³) where n is the cardinality of the sets.
///
/// # Panics
///
/// This function will panic if the `weights` matrix is not a square matrix.

pub fn kuhn_munkres<C>(weights: &Array2<C>) -> (C, Vec<usize>)
where
    C: Sum<C> + Zero + Signed + Ord + Copy,
{
    // We call x the rows and y the columns. n is the size of the matrix.
    let n = weights.shape()[0];
    assert_eq!(n, weights.shape()[1]);
    // xy represents matchings for x, yz matchings for y
    let mut xy: Vec<Option<usize>> = vec![None; n];
    let mut yx: Vec<Option<usize>> = vec![None; n];
    // Start with an initial assignment for every x node satisfying its maximum
    // labelling. Also, count the assigned nodes. Once it reaches n, we have our
    // solution. Labelling for y nodes will start at 0.
    let mut lx: Vec<C> = vec![Zero::zero(); n];
    let mut assigned = 0;
    for (x, row) in weights.outer_iter().enumerate() {
        let (y, l) = row.into_iter().enumerate().max_by_key(|&v| v.1).unwrap();
        lx[x] = *l;
        if yx[y].is_none() {
            yx[y] = Some(x);
            xy[x] = Some(y);
            assigned += 1;
        }
    }
    let mut ly: Vec<C> = vec![Zero::zero(); n];
    // s, augmenting, and slack will be reset every time they are reused. augmenting
    // contains Some(prev) when the corresponding node belongs to the augmenting path.
    let mut s = FixedBitSet::with_capacity(n);
    let mut alternating = Vec::with_capacity(n);
    let mut slack = vec![(Zero::zero(), 0); n];
    for root in 0..n {
        if assigned == n {
            break;
        }
        if xy[root].is_some() {
            continue;
        }
        alternating.clear();
        alternating.resize(n, None);
        // Find y such that the path is augmented. This will be set when breaking for the
        // loop below. Above the loop is some code to initialize the search.
        let mut y = {
            s.clear();
            s.insert(root);
            // Slack for a vertex y is, initially, the margin between the
            // sum of the labels of root and y, and the weight between root and y.
            // As we add x nodes to the alternating path, we update the slack to
            // represent the smallest margin between one of the x nodes and y.
            for y in 0..n {
                slack[y] = (lx[root] + ly[y] - weights[[root, y]], root);
            }
            Some(loop {
                // Select one of the smallest slack delta and its edge (x, y)
                // for y not in the alternating path already.
                let ((delta, x), y) = (0..n)
                    .filter(|&y| alternating[y].is_none())
                    .map(|y| (slack[y], y))
                    .min()
                    .unwrap();
                debug_assert!(s.contains(x));
                // If some slack has been found, remove it from x nodes in the
                // alternating path, and add it to y nodes in the alternating path.
                // The slack of y nodes outside the alternating path will be reduced
                // by this minimal slack as well.
                if delta > Zero::zero() {
                    for x in 0..n {
                        if s.contains(x) {
                            lx[x] = lx[x] - delta;
                        }
                    }
                    for y in 0..n {
                        if alternating[y].is_some() {
                            ly[y] = ly[y] + delta;
                        } else {
                            let (val, arg) = slack[y];
                            slack[y] = (val - delta, arg);
                        }
                    }
                }
                debug_assert!(lx[x] + ly[y] == weights[[x, y]]);
                // Add (x, y) to the alternating path.
                alternating[y] = Some(x);
                if yx[y].is_none() {
                    // We have found an augmenting path.
                    break y;
                }
                // This y node had a predecessor, add it to the set of x nodes
                // in the augmenting path.
                let x = yx[y].unwrap();
                debug_assert!(!s.contains(x));
                s.insert(x);
                // Update slack because of the added vertex in s might contain a
                // greater slack than with previously inserted x nodes in the augmenting
                // path.
                for y in 0..n {
                    if alternating[y].is_none() {
                        let alternate_slack = (lx[x] + ly[y] - weights[[x, y]], x);
                        if slack[y] > alternate_slack {
                            slack[y] = alternate_slack;
                        }
                    }
                }
            })
        };
        // Inverse edges along the augmenting path.
        while y.is_some() {
            let x = alternating[y.unwrap()].unwrap();
            let prec = xy[x];
            yx[y.unwrap()] = Some(x);
            xy[x] = y;
            y = prec;
        }
        assigned += 1;
    }
    (
        lx.into_iter().sum::<C>() + ly.into_iter().sum(),
        xy.into_iter().map(|v| v.unwrap()).collect::<Vec<_>>(),
    )
}

/// Compute the minimum matching between two disjoints sets of vertices
/// using the
/// [Kuhn-Munkres algorithm](https://en.wikipedia.org/wiki/Hungarian_algorithm)
/// (also known as Hungarian algorithm).
///
/// The weights between the first and second sets are given into the
/// `weights` square matrix. The first axis is indexed by the X set,
/// and the second axis by the Y set. The return value is a pair with
/// the total assignments weight, and a vector containing indices in
/// the Y set for every vertex in the X set.
///
/// This algorithm executes in O(n³) where n is the cardinality of the sets.
///
/// # Panics
///
/// This function will panic if the `weights` matrix is not a square matrix.

pub fn kuhn_munkres_min<C>(weights: &Array2<C>) -> (C, Vec<usize>)
where
    C: Sum<C> + Zero + Signed + Ord + Copy,
{
    let (total, assignments) = kuhn_munkres(&-weights.clone());
    (-total, assignments)
}
