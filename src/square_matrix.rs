use num_traits::Signed;
use std::ops::{Index, IndexMut, Neg};

/// Square matrix of an arbitrary type
pub struct SquareMatrix<C> {
    /// Size of every dimension
    pub size: usize,
    data: Vec<C>,
}

impl<C: Clone> SquareMatrix<C> {
    /// Create new square matrix with an initial value.
    pub fn new(size: usize, value: C) -> SquareMatrix<C> {
        let mut v = Vec::with_capacity(size * size);
        v.resize(size * size, value);
        SquareMatrix {
            size: size,
            data: v,
        }
    }

    /// Fill with a known value.
    pub fn fill(&mut self, value: C) {
        self.data.clear();
        self.data.resize(self.size * self.size, value);
    }
}

impl<C: Clone> Clone for SquareMatrix<C> {
    fn clone(&self) -> SquareMatrix<C> {
        SquareMatrix {
            size: self.size,
            data: self.data.clone(),
        }
    }
}

impl<C: Clone + Signed> Neg for SquareMatrix<C> {
    type Output = SquareMatrix<C>;

    fn neg(self) -> SquareMatrix<C> {
        SquareMatrix {
            size: self.size,
            data: self.data.iter().map(|x| -x.clone()).collect::<Vec<_>>(),
        }
    }
}

impl<C> SquareMatrix<C> {
    fn idx(&self, i: &(usize, usize)) -> usize {
        i.0 + i.1 * self.size
    }
}

impl<'a, C> Index<&'a (usize, usize)> for SquareMatrix<C> {
    type Output = C;

    fn index(&self, index: &'a (usize, usize)) -> &C {
        &self.data[self.idx(&index)]
    }
}

impl<'a, C> IndexMut<&'a (usize, usize)> for SquareMatrix<C> {
    fn index_mut(&mut self, index: &'a (usize, usize)) -> &mut C {
        let i = self.idx(&index);
        &mut self.data[i]
    }
}
