//! Rectangular grid in which vertices can be added or removed, with or
//! without diagonal links.

use indexmap::IndexSet;
use itertools::iproduct;
use std::fmt;
use std::iter::FromIterator;

use super::utils::absdiff;

#[derive(Clone)]
/// Representation of a rectangular grid in which vertices can be added
/// or removed. Edges are automatically created between adjacent vertices.
/// By default, only vertical and horizontal edges are created, unless
/// diagonal mode is enabled.
///
/// Internally, a Grid is represented either as a collection of vertices
/// or as a collection of absent vertices, depending on the density of
/// the grid. The switch between both representations is done automatically
/// when vertices are added or removed, or when the grid is resized.
pub struct Grid {
    /// The grid width.
    pub width: usize,
    /// The grid height.
    pub height: usize,
    diagonal_mode: bool,
    // `dense` is true if the grid is full by default and `exclusions`
    // contains absent vertices. It is false if the grid is empty by default
    // and `exclusions` contains the vertices.
    dense: bool,
    exclusions: IndexSet<(usize, usize)>,
}

impl Grid {
    /// Create a new empty grid object of the given dimensions, with
    /// diagonal mode disabled.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            diagonal_mode: false,
            dense: false,
            exclusions: IndexSet::new(),
        }
    }

    /// Check if a (possibly removed) vertex belongs to the grid or if it
    /// is located outside the grid.
    #[inline]
    #[must_use]
    pub fn is_inside(&self, vertex: &(usize, usize)) -> bool {
        vertex.0 < self.width && vertex.1 < self.height
    }

    /// Enable diagonal mode. Diagonal edges will be created between
    /// adjacent vertices.
    pub fn enable_diagonal_mode(&mut self) {
        self.diagonal_mode = true;
    }

    /// Disable diagonal mode. Only horizontal and vertical edges will
    /// be created between adjacent vertices.
    pub fn disable_diagonal_mode(&mut self) {
        self.diagonal_mode = false;
    }

    /// Resize the grid to the given dimensions. Return `true` if this
    /// caused any existing vertex to be discarded.
    pub fn resize(&mut self, width: usize, height: usize) -> bool {
        let mut truncated = false;
        if width < self.width {
            truncated |= iproduct!(width..self.width, 0..self.height).any(|c| self.has_vertex(&c));
        }
        if height < self.height {
            truncated |= iproduct!(0..self.width, height..self.height).any(|c| self.has_vertex(&c));
        }
        self.exclusions.retain(|&(x, y)| x < width && y < height);
        if self.dense {
            for vertex in iproduct!(self.width..width, 0..height) {
                self.exclusions.insert(vertex);
            }
            for vertex in iproduct!(0..self.width.min(width), self.height..height) {
                self.exclusions.insert(vertex);
            }
        }
        self.width = width;
        self.height = height;
        self.rebalance();
        truncated
    }

    /// Return the number of positions in this grid.
    #[must_use]
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    /// Return the number of vertices.
    #[must_use]
    pub fn vertices_len(&self) -> usize {
        if self.dense {
            self.size() - self.exclusions.len()
        } else {
            self.exclusions.len()
        }
    }

    /// Add a new vertex. Return `true` if the vertex did not previously
    /// exist and has been added.
    pub fn add_vertex(&mut self, vertex: (usize, usize)) -> bool {
        if !self.is_inside(&vertex) {
            return false;
        }
        let r = if self.dense {
            self.exclusions.remove(&vertex)
        } else {
            self.exclusions.insert(vertex)
        };
        self.rebalance();
        r
    }

    /// Remove a vertex. Return `true` if the vertex did previously exist
    /// and has been removed.
    pub fn remove_vertex(&mut self, vertex: &(usize, usize)) -> bool {
        if !self.is_inside(vertex) {
            return false;
        }
        let r = if self.dense {
            self.exclusions.insert(*vertex)
        } else {
            self.exclusions.remove(vertex)
        };
        self.rebalance();
        r
    }

    /// Return an iterator over the border vertices. The grid must not have
    /// a zero width or height.
    fn borders(&self) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        let height = self.height;
        (0..width)
            .flat_map(move |x| vec![(x, 0), (x, height - 1)].into_iter())
            .chain((1..height - 1).flat_map(move |y| vec![(0, y), (width - 1, y)].into_iter()))
    }

    /// Add the borders of the grid. Return the number of added vertices.
    pub fn add_borders(&mut self) -> usize {
        if self.width == 0 || self.height == 0 {
            return 0;
        }
        let count = if self.dense {
            self.borders().filter(|v| self.exclusions.remove(v)).count()
        } else {
            self.borders()
                .filter(|v| self.exclusions.insert(*v))
                .count()
        };
        self.rebalance();
        count
    }

    /// Remove the borders of the grid. Return the number of removed vertices.
    pub fn remove_borders(&mut self) -> usize {
        if self.width == 0 || self.height == 0 {
            return 0;
        }
        let count = if self.dense {
            self.borders()
                .filter(|v| self.exclusions.insert(*v))
                .count()
        } else {
            self.borders().filter(|v| self.exclusions.remove(v)).count()
        };
        self.rebalance();
        count
    }

    fn rebalance(&mut self) {
        if self.exclusions.len() > self.width * self.height / 2 {
            self.exclusions = iproduct!(0..self.width, 0..self.height)
                .filter(|v| !self.exclusions.contains(v))
                .collect();
            self.dense = !self.dense;
        }
    }

    /// Remove all vertices from the grid. Return `true` if the grid
    /// previously contained at least one vertex.
    pub fn clear(&mut self) -> bool {
        let r = !self.is_empty();
        self.dense = false;
        self.exclusions.clear();
        r
    }

    /// Fill the grid with all possible vertices. Return `true` if
    /// this caused the addition of at least one vertex.
    pub fn fill(&mut self) -> bool {
        let r = !self.is_full();
        self.clear();
        self.invert();
        r
    }

    /// Return `true` if the grid contains no vertices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        if self.dense {
            self.exclusions.len() == self.size()
        } else {
            self.exclusions.is_empty()
        }
    }

    /// Return `true` if no additional vertices can be set
    /// (because they are all already set).
    #[must_use]
    pub fn is_full(&self) -> bool {
        if self.dense {
            self.exclusions.is_empty()
        } else {
            self.exclusions.len() == self.size()
        }
    }

    /// Remove every existing vertex, and add all absent vertices.
    /// If you see the grid as a black and white array, imagine that
    /// the color are exchanged.
    pub fn invert(&mut self) {
        self.dense = !self.dense;
    }

    /// Check if a vertex is present.
    #[must_use]
    pub fn has_vertex(&self, vertex: &(usize, usize)) -> bool {
        self.is_inside(vertex) && (self.exclusions.contains(vertex) ^ self.dense)
    }

    /// Check if an edge is present.
    #[must_use]
    pub fn has_edge(&self, v1: &(usize, usize), v2: &(usize, usize)) -> bool {
        if !self.has_vertex(v1) || !self.has_vertex(v2) {
            return false;
        }
        let x = absdiff(v1.0, v2.0);
        let y = absdiff(v1.1, v2.1);
        x + y == 1 || (x == 1 && y == 1 && self.diagonal_mode)
    }

    /// Iterate over edges.
    #[must_use]
    pub fn edges(&self) -> EdgesIterator {
        EdgesIterator {
            grid: self,
            x: 0,
            y: 0,
            i: 0,
        }
    }

    /// Return the list of neighbours of a given vertex. If `vertex` is absent
    /// from the grid, an empty list is returned. Only existing vertices will
    /// be returned.
    #[must_use]
    pub fn neighbours(&self, vertex: &(usize, usize)) -> Vec<(usize, usize)> {
        if !self.has_vertex(vertex) {
            return vec![];
        }
        let &(x, y) = vertex;
        let mut candidates = Vec::with_capacity(8);
        if x > 0 {
            candidates.push((x - 1, y));
            if self.diagonal_mode {
                if y > 0 {
                    candidates.push((x - 1, y - 1));
                }
                if y + 1 < self.height {
                    candidates.push((x - 1, y + 1));
                }
            }
        }
        if x + 1 < self.width {
            candidates.push((x + 1, y));
            if self.diagonal_mode {
                if y > 0 {
                    candidates.push((x + 1, y - 1));
                }
                if y + 1 < self.height {
                    candidates.push((x + 1, y + 1));
                }
            }
        }
        if y > 0 {
            candidates.push((x, y - 1));
        }
        if y + 1 < self.height {
            candidates.push((x, y + 1));
        }
        candidates.retain(|v| self.has_vertex(v));
        candidates
    }

    /// Iterate over vertices.
    #[must_use]
    pub fn iter(&self) -> GridIterator {
        self.into_iter()
    }

    /// Distance between two potential vertices. If diagonal mode is
    /// enabled, this is the maximum of both coordinates difference.
    /// If diagonal mode is disabled, this is the Manhattan distance.
    #[must_use]
    pub fn distance(&self, a: &(usize, usize), b: &(usize, usize)) -> usize {
        let (dx, dy) = (absdiff(a.0, b.0), absdiff(a.1, b.1));
        if self.diagonal_mode {
            dx.max(dy)
        } else {
            dx + dy
        }
    }
}

impl FromIterator<(usize, usize)> for Grid {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (usize, usize)>,
    {
        let vertices = iter.into_iter().collect();
        let mut width = 0;
        let mut height = 0;
        for &(x, y) in &vertices {
            if x + 1 > width {
                width = x + 1;
            }
            if y + 1 > height {
                height = y + 1;
            }
        }
        let mut grid = Self {
            width,
            height,
            diagonal_mode: false,
            dense: false,
            exclusions: vertices,
        };
        grid.rebalance();
        grid
    }
}

/// Iterator returned by calling `.into_iter()` on a grid.
pub struct GridIntoIterator {
    grid: Grid,
    x: usize,
    y: usize,
}

impl Iterator for GridIntoIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.dense {
            loop {
                if self.y == self.grid.height {
                    return None;
                }
                let r = if self.grid.has_vertex(&(self.x, self.y)) {
                    Some((self.x, self.y))
                } else {
                    None
                };
                self.x += 1;
                if self.x == self.grid.width {
                    self.x = 0;
                    self.y += 1;
                }
                if r.is_some() {
                    return r;
                }
            }
        } else {
            self.grid.exclusions.pop()
        }
    }
}

impl IntoIterator for Grid {
    type Item = (usize, usize);
    type IntoIter = GridIntoIterator;

    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

/// Iterator returned by calling `.iter()` on a grid.
pub struct GridIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.dense {
            loop {
                if self.y == self.grid.height {
                    return None;
                }
                let r = if self.grid.has_vertex(&(self.x, self.y)) {
                    Some((self.x, self.y))
                } else {
                    None
                };
                self.x += 1;
                if self.x == self.grid.width {
                    self.x = 0;
                    self.y += 1;
                }
                if r.is_some() {
                    return r;
                }
            }
        } else {
            self.grid
                .exclusions
                .get_index(self.x)
                .map(|v| {
                    self.x += 1;
                    v
                })
                .copied()
        }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize);
    type IntoIter = GridIterator<'a>;

    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

/// Iterator returned by calling `.edges()` on a grid.
pub struct EdgesIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    i: usize,
}

impl<'a> Iterator for EdgesIterator<'a> {
    type Item = ((usize, usize), (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.y == self.grid.height {
                return None;
            }
            let x = self.x;
            let y = self.y;
            let other = match self.i {
                0 => (x + 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y + 1),
                _ => (x - 1, y + 1),
            };
            self.i += 1;
            if (x == 0 && self.i == 3) || self.i == 4 {
                self.i = 0;
                self.x += 1;
                if self.x == self.grid.width {
                    self.x = 0;
                    self.y += 1;
                }
            }
            if self.grid.has_edge(&(x, y), &other) {
                return Some(((x, y), other));
            }
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.has_vertex(&(x, y)) { '#' } else { '.' })?;
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
