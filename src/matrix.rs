//! Matrix of an arbitrary type and utilities to rotate, transpose, etc.

use crate::utils::uint_sqrt;
use itertools::iproduct;
use itertools::Itertools;
use num_traits::Signed;
use std::error::Error;
use std::fmt;
use std::ops::RangeInclusive;
use std::ops::{Deref, DerefMut, Index, IndexMut, Neg, Range};
use std::slice::{Iter, IterMut};

/// Matrix of an arbitrary type. Data are stored consecutively in
/// memory, by rows. Raw data can be accessed using `as_ref()`
/// or `as_mut()`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Matrix<C> {
    /// Rows
    pub rows: usize,
    /// Columns
    pub columns: usize,
    data: Vec<C>,
}

impl<C: Clone> Matrix<C> {
    /// Create new matrix with an initial value.
    pub fn new(rows: usize, columns: usize, value: C) -> Self {
        let mut v = Vec::with_capacity(rows * columns);
        v.resize(rows * columns, value);
        Self {
            rows,
            columns,
            data: v,
        }
    }

    /// Create new square matrix with initial value.
    pub fn new_square(size: usize, value: C) -> Self {
        Self::new(size, size, value)
    }

    /// Fill with a known value.
    pub fn fill(&mut self, value: C) {
        self.data.clear();
        self.data.resize(self.rows * self.columns, value);
    }

    /// Return a copy of a sub-matrix, or return an error if the
    /// ranges are outside the original matrix.
    #[allow(clippy::needless_pass_by_value)]
    pub fn slice(
        &self,
        rows: Range<usize>,
        columns: Range<usize>,
    ) -> Result<Self, MatrixFormatError> {
        if rows.end > self.rows || columns.end > self.columns {
            return Err(MatrixFormatError {
                message: "slice far end points outside the matrix".to_owned(),
            });
        }
        let height = rows.end - rows.start;
        let width = columns.end - columns.start;
        let mut v = Vec::with_capacity(height * width);
        for r in rows {
            v.extend(
                self.data[r * self.columns + columns.start..r * self.columns + columns.end]
                    .iter()
                    .cloned(),
            );
        }
        Self::from_vec(height, width, v)
    }

    /// Return a copy of a matrix rotated clock-wise
    /// a number of times.
    #[must_use]
    pub fn rotated_cw(&self, times: usize) -> Self {
        if self.is_square() {
            let mut copy = self.clone();
            copy.rotate_cw(times);
            copy
        } else {
            match times % 4 {
                0 => self.clone(),
                1 => {
                    let mut copy = self.transposed();
                    copy.flip_lr();
                    copy
                }
                2 => {
                    let mut copy = self.clone();
                    copy.data.reverse();
                    copy
                }
                _ => {
                    let mut copy = self.transposed();
                    copy.flip_ud();
                    copy
                }
            }
        }
    }

    /// Return a copy of a matrix rotated counter-clock-wise
    /// a number of times.
    #[must_use]
    pub fn rotated_ccw(&self, times: usize) -> Self {
        self.rotated_cw(4 - (times % 4))
    }

    /// Return a copy of the matrix flipped along the vertical axis.
    #[must_use]
    pub fn flipped_lr(&self) -> Self {
        let mut copy = self.clone();
        copy.flip_lr();
        copy
    }

    /// Return a copy of the matrix flipped along the horizontal axis.
    #[must_use]
    pub fn flipped_ud(&self) -> Self {
        let mut copy = self.clone();
        copy.flip_ud();
        copy
    }

    /// Return a copy of the matrix after transposition.
    #[must_use]
    pub fn transposed(&self) -> Self {
        Self {
            rows: self.columns,
            columns: self.rows,
            data: iproduct!(0..self.columns, 0..self.rows)
                .map(|(c, r)| self.data[r * self.columns + c].clone())
                .collect(),
        }
    }

    /// Extend the matrix in place by adding one full row. An error
    /// is returned if the row does not have the expected number of
    /// elements.
    pub fn extend(&mut self, row: &[C]) -> Result<(), MatrixFormatError> {
        if self.columns != row.len() {
            return Err(MatrixFormatError {
                message: format!(
                    "new row has {} columns intead of expected {}",
                    row.len(),
                    self.columns
                ),
            });
        }
        self.rows += 1;
        for e in row {
            self.data.push(e.clone());
        }
        Ok(())
    }
}

impl<C: Copy> Matrix<C> {
    /// Replace a slice of the current matrix with the content of another one.
    /// Only the relevant cells will be extracted if the slice goes outside the
    /// original matrix.
    pub fn set_slice(&mut self, pos: &(usize, usize), slice: &Self) {
        let &(ref row, ref column) = pos;
        let height = (self.rows - row).min(slice.rows);
        let width = (self.columns - column).min(slice.columns);
        for r in 0..height {
            self.data[(row + r) * self.columns + column..(row + r) * self.columns + column + width]
                .copy_from_slice(&slice.data[r * slice.columns..r * slice.columns + width]);
        }
    }
}

impl<C: Clone + Signed> Neg for Matrix<C> {
    type Output = Self;

    #[must_use]
    fn neg(self) -> Self {
        Self {
            rows: self.rows,
            columns: self.columns,
            data: self.data.iter().map(|x| -x.clone()).collect::<Vec<_>>(),
        }
    }
}

impl<C> Matrix<C> {
    /// Create new matrix from vector values. The first value
    /// will be assigned to index (0, 0), the second one to index (0, 1),
    /// and so on. An error is returned instead if data length does not
    /// correspond to the announced size.
    pub fn from_vec(
        rows: usize,
        columns: usize,
        values: Vec<C>,
    ) -> Result<Self, MatrixFormatError> {
        if rows * columns != values.len() {
            return Err(MatrixFormatError { message: format!("length of vector does not correspond to announced dimensions ({} instead of {}×{}={})", values.len(), rows, columns, rows*columns)});
        }
        Ok(Self {
            rows,
            columns,
            data: values,
        })
    }

    /// Create new square matrix from vector values. The first value
    /// will be assigned to index (0, 0), the second one to index (0, 1),
    /// and so on. An error is returned if the number of values is not a
    /// square number.
    pub fn square_from_vec(values: Vec<C>) -> Result<Self, MatrixFormatError> {
        if let Some(size) = uint_sqrt(values.len()) {
            Self::from_vec(size, size, values)
        } else {
            Err(MatrixFormatError {
                message: format!("length of vector ({}) is not a square number", values.len()),
            })
        }
    }

    /// Create new empty matrix with a predefined number of columns.
    /// This is useful to gradually build the matrix and extend it
    /// later using [`extend`](Matrix::extend) and does not require
    /// a filler element compared to [`Matrix::new`].
    #[must_use]
    pub fn new_empty(columns: usize) -> Self {
        Self {
            rows: 0,
            columns,
            data: vec![],
        }
    }

    /// Create a matrix from something convertible to an iterator on rows,
    /// each row being convertible to an iterator on columns.
    ///
    /// An error will be returned if length of rows differ.
    ///
    /// ```
    /// use pathfinding::matrix::*;
    ///
    /// # fn main() -> Result<(), MatrixFormatError> {
    /// let input = "abc\ndef";
    /// let matrix = Matrix::from_rows(input.lines().map(|l| l.chars()))?;
    /// assert_eq!(matrix.rows, 2);
    /// assert_eq!(matrix.columns, 3);
    /// assert_eq!(matrix[&(1, 1)], 'e');
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_rows<IR, IC>(rows: IR) -> Result<Self, MatrixFormatError>
    where
        IR: IntoIterator<Item = IC>,
        IC: IntoIterator<Item = C>,
    {
        let mut rows = rows.into_iter();
        if let Some(first_row) = rows.next() {
            let mut data = first_row.into_iter().collect::<Vec<_>>();
            let number_of_columns = data.len();
            let mut number_of_rows = 1;
            for (i, row) in rows.enumerate() {
                number_of_rows += 1;
                data.extend(row);
                if number_of_rows * number_of_columns != data.len() {
                    return Err(MatrixFormatError {
                        message: format!(
                            "data for row {} (starting at 0) has len {} instead of expected {}",
                            i + 1,
                            data.len() - (number_of_rows - 1) * number_of_columns,
                            number_of_columns
                        ),
                    });
                }
            }
            Self::from_vec(number_of_rows, number_of_columns, data)
        } else {
            Ok(Self::new_empty(0))
        }
    }

    /// Check if a matrix is a square one.
    #[must_use]
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    /// Index in raw data of a given position.
    ///
    /// # Panics
    ///
    /// This function panics if the coordinates do not designated a cell.
    #[must_use]
    pub fn idx(&self, i: &(usize, usize)) -> usize {
        assert!(
            i.0 < self.rows,
            "trying to access row {} (max {})",
            i.0,
            self.rows - 1
        );
        assert!(
            i.1 < self.columns,
            "trying to access column {} (max {})",
            i.1,
            self.columns - 1
        );
        i.0 * self.columns + i.1
    }

    /// Flip the matrix around the vertical axis.
    pub fn flip_lr(&mut self) {
        for r in 0..self.rows {
            self.data[r * self.columns..(r + 1) * self.columns].reverse();
        }
    }

    /// Flip the matrix around the horizontal axis.
    pub fn flip_ud(&mut self) {
        for r in 0..self.rows / 2 {
            for c in 0..self.columns {
                self.data
                    .swap(r * self.columns + c, (self.rows - 1 - r) * self.columns + c);
            }
        }
    }

    /// Rotate a square matrix clock-wise a number of times.
    ///
    /// # Panics
    ///
    /// This function panics if the matrix is not square.
    pub fn rotate_cw(&mut self, times: usize) {
        assert!(
            self.rows == self.columns,
            "attempt to rotate a non-square matrix"
        );
        match times % 4 {
            0 => (),
            2 => self.data.reverse(),
            n => {
                for r in 0..self.rows / 2 {
                    for c in 0..(self.columns + 1) / 2 {
                        // i1 … i2
                        // …  …  …
                        // i4 … i3
                        let i1 = r * self.columns + c;
                        let i2 = c * self.columns + self.columns - 1 - r;
                        let i3 = (self.rows - 1 - r) * self.columns + self.columns - 1 - c;
                        let i4 = (self.rows - 1 - c) * self.columns + r;
                        if n == 1 {
                            // i1 … i2      i4 … i1
                            // …  …  …  =>  …  …  …
                            // i4 … i3      i3 … i2
                            self.data.swap(i1, i2);
                            self.data.swap(i1, i4);
                            self.data.swap(i3, i4);
                        } else {
                            // i1 … i2      i2 … i3
                            // …  …  …  =>  …  …  …
                            // i4 … i3      i1 … i4
                            self.data.swap(i3, i4);
                            self.data.swap(i1, i4);
                            self.data.swap(i1, i2);
                        }
                    }
                }
            }
        }
    }

    /// Rotate a square matrix counter-clock-wise a number of times.
    ///
    /// # Panics
    ///
    /// This function panics if the matrix is not square.
    pub fn rotate_ccw(&mut self, times: usize) {
        self.rotate_cw(4 - (times % 4));
    }

    /// Return an iterator on neighbours of a given matrix cell, with or
    /// without considering diagonals.
    pub fn neighbours(
        &self,
        &(r, c): &(usize, usize),
        diagonals: bool,
    ) -> impl Iterator<Item = (usize, usize)> {
        let row_range = RangeInclusive::new(r.saturating_sub(1), (self.rows - 1).min(r + 1));
        let col_range = RangeInclusive::new(c.saturating_sub(1), (self.columns - 1).min(c + 1));
        row_range
            .cartesian_product(col_range)
            .filter(move |&(rr, cc)| (rr != r || cc != c) && (diagonals || rr == r || cc == c))
    }

    /// Return the next cells in a given direction starting from
    /// a given cell. Any direction (including with values greater than 1) can be
    /// given. `(0, 0)` is not a valid direction.
    ///
    /// # Examples
    ///
    /// Starting from square `(1, 1)` in a 8×8 chessboard, move like a knight
    /// by steps of two rows down and one column right:
    ///
    /// ```
    /// use pathfinding::prelude::Matrix;
    /// let m = Matrix::new_square(8, '.');
    /// assert_eq!(m.move_in_direction(&(1, 1), (2, 1)), Some((3, 2)));
    /// ```
    #[must_use]
    pub fn move_in_direction(
        &self,
        index: &(usize, usize),
        direction: (isize, isize),
    ) -> Option<(usize, usize)> {
        move_in_direction(index, direction, self.rows, self.columns)
    }

    /// Return an iterator of cells in a given direction starting from
    /// a given cell. Any direction (including with values greater than 1) can be
    /// given. The starting cell is not included in the results.
    ///
    /// # Examples
    ///
    /// Starting from square `(1, 1)` in a 8×8 chessboard, move like a knight
    /// by steps of two rows down and one column right:
    ///
    /// ```
    /// use pathfinding::prelude::Matrix;
    /// let m = Matrix::new_square(8, '.');
    /// assert_eq!(m.in_direction(&(1, 1), (2, 1)).collect::<Vec<_>>(),
    ///            vec![(3, 2), (5, 3), (7, 4)]);
    /// ```
    ///
    /// Starting from square `(3, 2)` in the same chessboard, move diagonally in
    /// the North-West direction:
    ///
    /// ```
    /// use pathfinding::prelude::{Matrix, directions};
    /// let m = Matrix::new_square(8, '.');
    /// assert_eq!(m.in_direction(&(3, 2), directions::NW).collect::<Vec<_>>(),
    ///            vec![(2, 1), (1, 0)]);
    /// ```
    pub fn in_direction(
        &self,
        index: &(usize, usize),
        direction: (isize, isize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let (rows, columns) = (self.rows, self.columns);
        itertools::unfold(*index, move |current| {
            move_in_direction(current, direction, rows, columns).map(|next| {
                *current = next;
                next
            })
        })
    }

    /// Return an iterator on rows of the matrix.
    #[must_use]
    pub fn iter(&self) -> RowIterator<C> {
        self.into_iter()
    }

    /// Return an iterator on the Matrix indices, first row first.
    pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
        let columns = self.columns;
        (0..self.rows).flat_map(move |r| (0..columns).map(move |c| (r, c)))
    }

    /// Return an iterator on values, first row first.
    #[must_use]
    pub fn values(&self) -> Iter<C> {
        self.data.iter()
    }

    /// Return a mutable iterator on values, first row first.
    #[must_use]
    pub fn values_mut(&mut self) -> IterMut<C> {
        self.data.iter_mut()
    }
}

impl<'a, C> Index<&'a (usize, usize)> for Matrix<C> {
    type Output = C;

    #[must_use]
    fn index(&self, index: &'a (usize, usize)) -> &C {
        &self.data[self.idx(index)]
    }
}

impl<'a, C> IndexMut<&'a (usize, usize)> for Matrix<C> {
    fn index_mut(&mut self, index: &'a (usize, usize)) -> &mut C {
        let i = self.idx(index);
        &mut self.data[i]
    }
}

impl<C> Deref for Matrix<C> {
    type Target = [C];

    #[must_use]
    fn deref(&self) -> &[C] {
        &self.data
    }
}

impl<C> DerefMut for Matrix<C> {
    fn deref_mut(&mut self) -> &mut [C] {
        &mut self.data
    }
}

/// The matrix! macro allows the declaration of a Matrix from static data.
/// All rows must have the same number of columns. The data will be copied
/// into the matrix.
///
/// # Panics
///
/// This macro panics if the rows have an inconsistent number of columns.
///
/// # Example
///
/// ```
/// use pathfinding::matrix;
///
/// let m = matrix![[10, 20, 30], [40, 50, 60]];
///
/// assert_eq!(m.columns, 3);
/// assert_eq!(m.rows, 2);
/// ```
#[macro_export]
macro_rules! matrix {
    ($a:expr) => {{
        let mut m = pathfinding::matrix::Matrix::new_empty($a.len());
        m.extend(&$a).unwrap();
        m
    }};
    ($a:expr, $($b: expr),+) => {{
        let mut m = matrix!($a);
        let mut r = 0;
        $(
            m.extend(&$b).unwrap();
        )+
        m
    }};
    ($a:expr, $($b: expr),+, ) => (matrix!($a, $($b),+))
}

/// Format error encountered while attempting to build a Matrix.
#[derive(Debug)]
pub struct MatrixFormatError {
    message: String,
}

impl Error for MatrixFormatError {}

impl fmt::Display for MatrixFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "matrix format error: {}", self.message)
    }
}

/// Row iterator returned by `iter()` on a matrix.
pub struct RowIterator<'a, C> {
    matrix: &'a Matrix<C>,
    row: usize,
}

impl<'a, C> Iterator for RowIterator<'a, C> {
    type Item = &'a [C];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.matrix.rows {
            let r = Some(
                &self.matrix.data
                    [self.row * self.matrix.columns..(self.row + 1) * self.matrix.columns],
            );
            self.row += 1;
            r
        } else {
            None
        }
    }
}

impl<'a, C> IntoIterator for &'a Matrix<C> {
    type IntoIter = RowIterator<'a, C>;
    type Item = &'a [C];

    #[must_use]
    fn into_iter(self) -> RowIterator<'a, C> {
        RowIterator {
            matrix: self,
            row: 0,
        }
    }
}

/// Directions usable for [`Matrix::in_direction()`] second argument.
pub mod directions {
    /// East
    pub const E: (isize, isize) = (0, 1);

    /// South
    pub const S: (isize, isize) = (1, 0);

    /// West
    pub const W: (isize, isize) = (0, -1);

    /// North
    pub const N: (isize, isize) = (-1, 0);

    /// North-East
    pub const NE: (isize, isize) = (-1, 1);

    /// South-East
    pub const SE: (isize, isize) = (1, 1);

    /// North-West
    pub const NW: (isize, isize) = (-1, -1);

    /// South-West
    pub const SW: (isize, isize) = (1, -1);

    /// Four main directions
    pub const DIRECTIONS_4: [(isize, isize); 4] = [E, S, W, N];

    /// Eight main directions with diagonals
    pub const DIRECTIONS_8: [(isize, isize); 8] = [NE, E, SE, S, SW, W, NW, N];
}

#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn move_in_direction(
    index: &(usize, usize),
    direction: (isize, isize),
    rows: usize,
    columns: usize,
) -> Option<(usize, usize)> {
    let &(row, col) = index;
    if row >= rows || col >= columns || direction == (0, 0) {
        return None;
    }
    let (new_row, new_col) = (row as isize + direction.0, col as isize + direction.1);
    if new_row < 0 || new_col < 0 {
        return None;
    }
    let (new_row, new_col) = (new_row as usize, new_col as usize);
    (new_row < rows && new_col < columns).then(|| (new_row as usize, new_col as usize))
}
