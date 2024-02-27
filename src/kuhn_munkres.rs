//! Compute a maximum weight maximum matching between two disjoints sets of
//! vertices using the
//! [Kuhn-Munkres algorithm](https://en.wikipedia.org/wiki/Hungarian_algorithm)
//! (also known as Hungarian algorithm).

use crate::{matrix::Matrix, FxIndexSet};
use num_traits::{Bounded, Signed, Zero};
use std::iter::Sum;

/// Adjacency matrix for weights.
pub trait Weights<C> {
    /// Return the number of rows.
    #[must_use]
    fn rows(&self) -> usize;

    /// Return the number of columns.
    #[must_use]
    fn columns(&self) -> usize;

    /// Return the element at position.
    #[must_use]
    fn at(&self, row: usize, col: usize) -> C;

    /// Return the negated weights.
    #[must_use]
    fn neg(&self) -> Self
    where
        Self: Sized,
        C: Signed;
}

impl<C: Copy> Weights<C> for Matrix<C> {
    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }

    fn at(&self, row: usize, col: usize) -> C {
        self[(row, col)]
    }

    fn neg(&self) -> Self
    where
        C: Signed,
    {
        -self.clone()
    }
}

/// Compute a maximum weight maximum matching between two disjoints sets of
/// vertices using the
/// [Kuhn-Munkres algorithm](https://en.wikipedia.org/wiki/Hungarian_algorithm)
/// (also known as Hungarian algorithm).
///
/// The weights between the first and second sets are given into the
/// `weights` adjacency matrix. The return value is a pair with
/// the total assignments weight, and a vector containing the column
/// corresponding the every row.
///
/// For this reason, the number of rows must not be larger than the number of
/// columns as no row will be left unassigned.
///
/// This algorithm executes in O(n³) where n is the cardinality of the sets.
///
/// # Example
///
/// Three people in an art gallery are interested by the same three pieces.
/// Each of them has a budget of $300, and as if the world was perfect they
/// each determine that this corresponds to what they are ready to pay for
/// every piece:
///
/// - Ann estimates the Cloth to $100, the Statue to $110, and the Painting to $90
/// - Bernard estimates the Cloth to $95, the Statue to $130, and the Painting to $75
/// - Claude estimates the Cloth to $95, the Statue to $140 and the Painting to $65
///
/// The gallery owner doesn't want to totally frustrate any customer and decides to
/// sell one piece of art to each one of them. What would be the best way to maximize
/// the cash flow?
///
/// ```
/// use pathfinding::prelude::{kuhn_munkres, Matrix, Weights};
///
/// // Assign weights to everybody choices
/// let weights = Matrix::from_rows(vec![
///     //   Cloth, Statue, Painting
///     vec![100,   110,    90],        // Ann
///     vec![95,    130,    75],        // Bernard
///     vec![95,    140,    65],        // Claude
/// ]).unwrap();
///
/// // Find the maximum bipartite matching
/// let (cash_flow, assignments) = kuhn_munkres(&weights);
///
/// // Gallery owner receives $325, this is the maximum they can get
/// assert_eq!(cash_flow, 325);
///
/// // Ann gets the Painting for $90, Bernard gets the Cloth for $95,
/// // Claude gets the Statue for $140, which makes a total of $325
/// assert_eq!(assignments, vec![2, 0, 1]);
/// ```
///
/// # See also
///
/// To minimize the sum of weights instead of maximizing it, use the
/// [`kuhn_munkres_min()`] function.
///
/// # Panics
///
/// This function panics if the number of rows is larger than the number of
/// columns, or if the total assignments weight overflows or underflows.
///
/// Also, using indefinite values such as positive or negative infinity or
/// NaN can cause this function to loop endlessly.
pub fn kuhn_munkres<C, W>(weights: &W) -> (C, Vec<usize>)
where
    C: Bounded + Sum<C> + Signed + Zero + Ord + Copy,
    W: Weights<C>,
{
    // We call x the rows and y the columns. (nx, ny) is the size of the matrix.
    let nx = weights.rows();
    let ny = weights.columns();
    assert!(
        nx <= ny,
        "number of rows must not be larger than number of columns"
    );
    // xy represents matching for x, yz matching for y
    let mut xy: Vec<Option<usize>> = vec![None; nx];
    let mut yx: Vec<Option<usize>> = vec![None; ny];
    // lx is the labelling for x nodes, ly the labelling for y nodes. We start
    // with an acceptable labelling with the maximum possible values for lx
    // and 0 for ly.
    let mut lx: Vec<C> = (0..nx)
        .map(|row| (0..ny).map(|col| weights.at(row, col)).max().unwrap())
        .collect::<Vec<_>>();
    let mut ly: Vec<C> = vec![Zero::zero(); ny];
    // s, augmenting, and slack will be reset every time they are reused. augmenting
    // contains Some(prev) when the corresponding node belongs to the augmenting path.
    let mut s = FxIndexSet::<usize>::default();
    let mut alternating = Vec::with_capacity(ny);
    let mut slack = vec![Zero::zero(); ny];
    let mut slackx = Vec::with_capacity(ny);
    for root in 0..nx {
        alternating.clear();
        alternating.resize(ny, None);
        // Find y such that the path is augmented. This will be set when breaking for the
        // loop below. Above the loop is some code to initialize the search.
        let mut y = {
            s.clear();
            s.insert(root);
            // Slack for a vertex y is, initially, the margin between the
            // sum of the labels of root and y, and the weight between root and y.
            // As we add x nodes to the alternating path, we update the slack to
            // represent the smallest margin between one of the x nodes and y.
            for y in 0..ny {
                slack[y] = lx[root] + ly[y] - weights.at(root, y);
            }
            slackx.clear();
            slackx.resize(ny, root);
            Some(loop {
                let mut delta = Bounded::max_value();
                let mut x = 0;
                let mut y = 0;
                // Select one of the smallest slack delta and its edge (x, y)
                // for y not in the alternating path already.
                for yy in 0..ny {
                    if alternating[yy].is_none() && slack[yy] < delta {
                        delta = slack[yy];
                        x = slackx[yy];
                        y = yy;
                    }
                }
                // If some slack has been found, remove it from x nodes in the
                // alternating path, and add it to y nodes in the alternating path.
                // The slack of y nodes outside the alternating path will be reduced
                // by this minimal slack as well.
                if delta > Zero::zero() {
                    for &x in &s {
                        lx[x] = lx[x] - delta;
                    }
                    for y in 0..ny {
                        if alternating[y].is_some() {
                            ly[y] = ly[y] + delta;
                        } else {
                            slack[y] = slack[y] - delta;
                        }
                    }
                }
                // Add (x, y) to the alternating path.
                alternating[y] = Some(x);
                if yx[y].is_none() {
                    // We have found an augmenting path.
                    break y;
                }
                // This y node had a predecessor, add it to the set of x nodes
                // in the augmenting path.
                let x = yx[y].unwrap();
                s.insert(x);
                // Update slack because of the added vertex in s might contain a
                // greater slack than with previously inserted x nodes in the augmenting
                // path.
                for y in 0..ny {
                    if alternating[y].is_none() {
                        let alternate_slack = lx[x] + ly[y] - weights.at(x, y);
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
        xy.into_iter().map(Option::unwrap).collect::<Vec<_>>(),
    )
}

/// Compute a minimum weight maximum matching between two disjoints sets of
/// vertices using the
/// [Kuhn-Munkres algorithm](https://en.wikipedia.org/wiki/Hungarian_algorithm)
/// (also known as Hungarian algorithm).
///
/// The weights between the first and second sets are given into the
/// `weights` adjacency matrix. The return value is a pair with
/// the total assignments weight, and a vector containing the column
/// corresponding the every row.
///
/// For this reason, the number of rows must not be larger than the number of
/// columns as no row will be left unassigned.
///
/// This algorithm executes in O(n³) where n is the cardinality of the sets.
///
/// # See also
///
/// To maximize the sum of weights instead of minimizing it, use the
/// [`kuhn_munkres()`] function.
///
/// # Panics
///
/// This function panics if the number of rows is larger than the number of
/// columns, or if the total assignments weight overflows or underflows.
///
/// Also, using indefinite values such as positive or negative infinity or
/// NaN can cause this function to loop endlessly.
pub fn kuhn_munkres_min<C, W>(weights: &W) -> (C, Vec<usize>)
where
    C: Bounded + Sum<C> + Zero + Signed + Ord + Copy,
    W: Weights<C>,
{
    let (total, assignments) = kuhn_munkres(&weights.neg());
    (-total, assignments)
}
