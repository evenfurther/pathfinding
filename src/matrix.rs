use num_traits::Signed;
use std::ops::{Index, IndexMut, Neg};

/// Matrix of an arbitrary type
#[derive(Clone, Debug)]
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
            rows: rows,
            columns: columns,
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
            rows: rows,
            columns: columns,
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

    fn idx(&self, i: &(usize, usize)) -> usize {
        i.0 * self.columns + i.1
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
