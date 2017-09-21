use ndarray::Array2;
use num_traits::{Signed, Zero};
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
    // lx is the labelling for x nodes, ly the labelling for y nodes. We start
    // with an acceptable labelling with the maximum possible values for lx
    // and 0 for ly.
    let mut lx: Vec<C> = weights
        .outer_iter()
        .map(|row| row.into_iter().max().unwrap())
        .cloned()
        .collect::<Vec<_>>();
    let mut ly: Vec<C> = vec![Zero::zero(); n];
    // s, augmenting, and slack will be reset every time they are reused. augmenting
    // contains Some(prev) when the corresponding node belongs to the augmenting path.
    let mut s = Vec::with_capacity(n);
    let mut augmenting = Vec::with_capacity(n);
    let mut slack = vec![(Zero::zero(), 0); n];
    for root in 0..n {
        augmenting.clear();
        augmenting.resize(n, None);
        // Find y such that the path is augmented. This will be set when breaking for the
        // loop below. Above the loop is some code to initialize the search.
        let mut y = {
            s.clear();
            s.resize(n, false);
            s[root] = true;
            for y in 0..n {
                slack[y] = (lx[root] + ly[y] - weights[[root, y]], root);
            }
            Some(loop {
                let ((delta, x), y) = (0..n)
                    .filter(|&y| augmenting[y].is_none())
                    .map(|y| (slack[y], y))
                    .min()
                    .unwrap();
                debug_assert!(s[x]);
                if delta > Zero::zero() {
                    for x in 0..n {
                        if s[x] {
                            lx[x] = lx[x] - delta;
                        }
                    }
                    for y in 0..n {
                        if augmenting[y].is_some() {
                            ly[y] = ly[y] + delta;
                        } else {
                            let (val, arg) = slack[y];
                            slack[y] = (val - delta, arg);
                        }
                    }
                }
                debug_assert!(lx[x] + ly[y] == weights[[x, y]]);
                augmenting[y] = Some(x);
                if yx[y].is_none() {
                    // We have found an augmenting path.
                    break y;
                }
                // Add x to the set s.
                let x = yx[y].unwrap();
                debug_assert!(!s[x]);
                s[x] = true;
                // Update slack because of the added vertex in s.
                for y in 0..n {
                    if augmenting[y].is_none() {
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
            let x = augmenting[y.unwrap()].unwrap();
            let prec = xy[x];
            yx[y.unwrap()] = Some(x);
            xy[x] = y;
            y = prec;
        }
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
