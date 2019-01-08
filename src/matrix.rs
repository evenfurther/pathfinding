//! Matrix of an arbitrary type and utilities to rotate, transpose, etc.

use itertools::iproduct;
use num_traits::Signed;
use std::error::Error;
use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut, Neg, Range};

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
    pub fn new(rows: usize, columns: usize, value: C) -> Matrix<C> {
        let mut v = Vec::with_capacity(rows * columns);
        v.resize(rows * columns, value);
        Matrix {
            rows,
            columns,
            data: v,
        }
    }

    /// Create new square matrix with initial value.
    pub fn new_square(size: usize, value: C) -> Matrix<C> {
        Self::new(size, size, value)
    }

    /// Fill with a known value.
    pub fn fill(&mut self, value: C) {
        self.data.clear();
        self.data.resize(self.rows * self.columns, value);
    }

    /// Return a copy of a sub-matrix.
    #[allow(clippy::needless_pass_by_value)]
    pub fn slice(&self, rows: Range<usize>, columns: Range<usize>) -> Matrix<C> {
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
        Matrix::from_vec(height, width, v)
    }

    /// Return a copy of a matrix rotated clock-wise
    /// a number of times.
    pub fn rotated_cw(&self, times: usize) -> Matrix<C> {
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
    pub fn rotated_ccw(&self, times: usize) -> Matrix<C> {
        self.rotated_cw(4 - (times % 4))
    }

    /// Return a copy of the matrix flipped along the vertical axis.
    pub fn flipped_lr(&self) -> Matrix<C> {
        let mut copy = self.clone();
        copy.flip_lr();
        copy
    }

    /// Return a copy of the matrix flipped along the horizontal axis.
    pub fn flipped_ud(&self) -> Matrix<C> {
        let mut copy = self.clone();
        copy.flip_ud();
        copy
    }

    /// Return a copy of the matrix after transposition.
    pub fn transposed(&self) -> Matrix<C> {
        Matrix {
            rows: self.columns,
            columns: self.rows,
            data: iproduct!(0..self.columns, 0..self.rows)
                .map(|(c, r)| self.data[r * self.columns + c].clone())
                .collect(),
        }
    }

    /// Extend the matrix in place by adding one full row.
    ///
    /// # Panics
    ///
    /// This function panics if the row does not have the expected
    /// number of elements.
    pub fn extend(&mut self, row: &[C]) {
        assert_eq!(
            self.columns,
            row.len(),
            "new row has {} columns intead of expected {}",
            row.len(),
            self.columns
        );
        self.rows += 1;
        for e in row {
            self.data.push(e.clone());
        }
    }
}

impl<C: Copy> Matrix<C> {
    /// Replace a slice of the current matrix with the content of another one.
    pub fn set_slice(&mut self, pos: &(usize, usize), slice: &Matrix<C>) {
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
    type Output = Matrix<C>;

    fn neg(self) -> Matrix<C> {
        Matrix {
            rows: self.rows,
            columns: self.columns,
            data: self.data.iter().map(|x| -x.clone()).collect::<Vec<_>>(),
        }
    }
}

impl<C> Matrix<C> {
    /// Create new matrix from vector values. The first value
    /// will be assigned to index (0, 0), the second one to index (0, 1),
    /// and so on.
    ///
    /// # Panics
    ///
    /// This function will panic if the number of values does not correspond
    /// to the announced size.
    pub fn from_vec(rows: usize, columns: usize, values: Vec<C>) -> Matrix<C> {
        assert_eq!(
            rows * columns,
            values.len(),
            "length of vector does not correspond to announced dimensions"
        );
        Matrix {
            rows,
            columns,
            data: values,
        }
    }

    /// Create new square matrix from vector values. The first value
    /// will be assigned to index (0, 0), the second one to index (0, 1),
    /// and so on.
    ///
    /// # Panics
    ///
    /// This function will panic if the number of values is not a square number.
    pub fn square_from_vec(values: Vec<C>) -> Matrix<C> {
        let size = (values.len() as f32).sqrt().round() as usize;
        assert_eq!(
            size * size,
            values.len(),
            "length of vector is not a square number"
        );
        Self::from_vec(size, size, values)
    }

    /// Create new empty matrix with a predefined number of rows.
    /// This is useful to gradually build the matrix and extend it
    /// later using [extend][Matrix::extend] and does not require
    /// a filler element compared to [Matrix::new].
    pub fn new_empty(columns: usize) -> Matrix<C> {
        Matrix {
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
    pub fn from_rows<IR, IC>(rows: IR) -> Result<Matrix<C>, MatrixFormatError>
    where
        IR: IntoIterator<Item = IC>,
        IC: IntoIterator<Item = C>,
    {
        let mut rows = rows.into_iter();
        if let Some(first_row) = rows.next() {
            let mut data = first_row.into_iter().collect::<Vec<_>>();
            let ncols = data.len();
            let mut nrows = 1;
            for (i, row) in rows.enumerate() {
                nrows += 1;
                data.extend(row);
                if nrows * ncols != data.len() {
                    return Err(MatrixFormatError {
                        message: format!(
                            "data for row {} (starting at 0) has len {} instead of expected {}",
                            i + 1,
                            data.len() - (nrows - 1) * ncols,
                            ncols
                        ),
                    });
                }
            }
            Ok(Matrix::from_vec(nrows, ncols, data))
        } else {
            Ok(Matrix::new_empty(0))
        }
    }

    /// Check if a matrix is a square one.
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    /// Index in raw data of a given position.
    ///
    /// # Panics
    ///
    /// This function panics if the coordinates do not designated a cell.
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
        self.rotate_cw(4 - (times % 4))
    }

    /// Return an iterator on neighbours of a given matrix cell, with or
    /// without considering diagonals.
    pub fn neighbours(
        &self,
        index: &(usize, usize),
        diagonals: bool,
    ) -> impl Iterator<Item = (usize, usize)> {
        let &(r, c) = index;
        let min_dr = if r == 0 { 0 } else { -1 };
        let max_dr = if r == self.rows - 1 { 0 } else { 1 };
        let min_dc = if c == 0 { 0 } else { -1 };
        let max_dc = if c == self.columns - 1 { 0 } else { 1 };
        (min_dc..=max_dc)
            .flat_map(move |dc| (min_dr..=max_dr).map(move |dr| (dr, dc)))
            .filter(move |&(dr, dc)| (diagonals && dr != 0 && dc != 0) || dr.abs() + dc.abs() == 1)
            .map(move |(dr, dc)| ((r as isize + dr) as usize, (c as isize + dc) as usize))
    }
}

impl<'a, C> Index<&'a (usize, usize)> for Matrix<C> {
    type Output = C;

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
        m.extend(&$a);
        m
    }};
    ($a:expr, $($b: expr),+) => {{
        let mut m = matrix!($a);
        let mut r = 0;
        $(
            m.extend(&$b);
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
