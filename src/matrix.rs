//! Matrix of an arbitrary type and utilities to rotate, transpose, etc.

extern crate itertools;

use num_traits::Signed;
use std::ops::{Index, IndexMut, Neg, Range};

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

unsafe impl<C: Send> Send for Matrix<C> {}

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
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
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

    /// Return a copy of a square matrix rotated clock-wise
    /// a number of times.
    ///
    /// # Panics
    ///
    /// This function panics if the matrix is not square.
    pub fn rotated_cw(&self, times: usize) -> Matrix<C> {
        let mut copy = self.clone();
        copy.rotate_cw(times);
        copy
    }

    /// Return a copy of a square matrix rotated counter-clock-wise
    /// a number of times.
    ///
    /// # Panics
    ///
    /// This function panics if the matrix is not square.
    pub fn rotated_ccw(&self, times: usize) -> Matrix<C> {
        let mut copy = self.clone();
        copy.rotate_ccw(times);
        copy
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

    /// Check if a matrix is a square one.
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    /// Index in raw data of a given position.
    pub fn idx(&self, i: &(usize, usize)) -> usize {
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

impl<C> AsRef<[C]> for Matrix<C> {
    fn as_ref(&self) -> &[C] {
        &self.data
    }
}

impl<C> AsMut<[C]> for Matrix<C> {
    fn as_mut(&mut self) -> &mut [C] {
        &mut self.data
    }
}
