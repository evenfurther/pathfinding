use num_traits::{Bounded, Signed, Zero};
use fixedbitset::FixedBitSet;
use square_matrix::SquareMatrix;
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

pub fn kuhn_munkres<C>(weights: &SquareMatrix<C>) -> (C, Vec<usize>)
where
    C: Bounded + Sum<C> + Zero + Signed + Ord + Copy,
{
    // We call x the rows and y the columns. n is the size of the matrix.
    let n = weights.size;
    // xy represents matchings for x, yz matchings for y
    let mut xy: Vec<Option<usize>> = vec![None; n];
    let mut yx: Vec<Option<usize>> = vec![None; n];
    // lx is the labelling for x nodes, ly the labelling for y nodes. We start
    // with an acceptable labelling with the maximum possible values for lx
    // and 0 for ly.
    let mut lx: Vec<C> = (0..n)
        .map(|row| (0..n).map(|col| weights[&(row, col)]).max().unwrap())
        .collect::<Vec<_>>();
    let mut ly: Vec<C> = vec![Zero::zero(); n];
    // s, augmenting, and slack will be reset every time they are reused. augmenting
    // contains Some(prev) when the corresponding node belongs to the augmenting path.
    let mut s = FixedBitSet::with_capacity(n);
    let mut alternating = Vec::with_capacity(n);
    let mut slack = vec![Zero::zero(); n];
    let mut slackx = Vec::with_capacity(n);
    for root in 0..n {
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
                slack[y] = lx[root] + ly[y] - weights[&(root, y)];
            }
            slackx.clear();
            slackx.resize(n, root);
            Some(loop {
                let mut delta = Bounded::max_value();
                let mut x = 0;
                let mut y = 0;
                // Select one of the smallest slack delta and its edge (x, y)
                // for y not in the alternating path already.
                for yy in 0..n {
                    if alternating[yy].is_none() && slack[yy] < delta {
                        delta = slack[yy];
                        x = slackx[yy];
                        y = yy;
                    }
                }
                debug_assert!(s.contains(x));
                // If some slack has been found, remove it from x nodes in the
                // alternating path, and add it to y nodes in the alternating path.
                // The slack of y nodes outside the alternating path will be reduced
                // by this minimal slack as well.
                if delta > Zero::zero() {
                    for x in s.ones() {
                        lx[x] = lx[x] - delta;
                    }
                    for y in 0..n {
                        if alternating[y].is_some() {
                            ly[y] = ly[y] + delta;
                        } else {
                            slack[y] = slack[y] - delta;
                        }
                    }
                }
                debug_assert!(lx[x] + ly[y] == weights[&(x, y)]);
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
                        let alternate_slack = lx[x] + ly[y] - weights[&(x, y)];
                        if slack[y] > alternate_slack {
                            slack[y] = alternate_slack;
                            slackx[y] = x;
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

pub fn kuhn_munkres_min<C>(weights: &SquareMatrix<C>) -> (C, Vec<usize>)
where
    C: Bounded + Sum<C> + Zero + Signed + Ord + Copy,
{
    let (total, assignments) = kuhn_munkres(&-weights.clone());
    (-total, assignments)
}
