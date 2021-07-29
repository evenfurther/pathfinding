//! Compute the maximum flow that can go through a directed graph using the
//! [Edmonds Karp algorithm](https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm).
//!
//! This module contains several functions helping compute the flow on a given
//! graph, as well as a structure which allows iterative modifications of the
//! network. When the network is modified, the flow is recomputed and tries to
//! take advantage of computations already performed on unchanged or augmented
//! edges.

use indexmap::IndexSet;
use itertools::iproduct;
use num_traits::{Bounded, Signed, Zero};
use std::collections::{BTreeMap, VecDeque};
use std::hash::Hash;

use super::bfs::bfs;
use crate::matrix::Matrix;

/// Type alias for Edmonds-Karp result.
#[allow(clippy::upper_case_acronyms)]
pub type EKFlows<N, C> = (Vec<((N, N), C)>, C);

/// Compute the maximum flow that can go through a directed graph using the
/// [Edmonds Karp algorithm](https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm).
///
/// A maximum flow going from `source` to `sink` will be computed, and the various
/// flow values along with the total will be returned.
///
/// - `vertices` is the collection of vertices in the graph.
/// - `source` is the source node (the origin of the flow).
/// - `sink` is the sink node (the target of the flow).
/// - `caps` is an iterator-like object describing the positive capacities between the
///   nodes.
///
/// Note that the capacity type `C` must be signed as the algorithm has to deal with
/// negative residual capacities.
///
/// By creating an [`EdmondsKarp`]() structure, it is possible to adjust the capacities
/// after computing the maximum flow and rerun the algorithm without starting from
/// scratch. This function is a helper function that remaps the `N` node type to
/// appropriate indices.
///
/// # Panics
///
/// This function panics if `source` or `sink` is not found in `vertices`.
pub fn edmonds_karp<N, C, IC, EK>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Bounded + Signed + Ord + Copy,
    IC: IntoIterator<Item = ((N, N), C)>,
    EK: EdmondsKarp<C>,
{
    // Build a correspondence between N and 0..vertices.len() so that we can
    // work with matrices more easily.
    let reverse = vertices.iter().collect::<IndexSet<_>>();
    let mut capacities = EK::new(
        vertices.len(),
        reverse.get_index_of(source).unwrap(),
        reverse.get_index_of(sink).unwrap(),
    );
    for ((from, to), capacity) in caps {
        capacities.set_capacity(
            reverse.get_index_of(&from).unwrap(),
            reverse.get_index_of(&to).unwrap(),
            capacity,
        );
    }
    let (paths, max) = capacities.augment();
    (
        paths
            .into_iter()
            .map(|((a, b), c)| ((vertices[a], vertices[b]), c))
            .collect(),
        max,
    )
}

/// Helper for the `edmonds_karp` function using an adjacency matrix for dense graphs.
pub fn edmonds_karp_dense<N, C, IC>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Bounded + Signed + Ord + Copy,
    IC: IntoIterator<Item = ((N, N), C)>,
{
    edmonds_karp::<N, C, IC, DenseCapacity<C>>(vertices, source, sink, caps)
}

/// Helper for the `edmonds_karp` function using adjacency maps for sparse graphs.
pub fn edmonds_karp_sparse<N, C, IC>(
    vertices: &[N],
    source: &N,
    sink: &N,
    caps: IC,
) -> EKFlows<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Bounded + Signed + Ord + Copy,
    IC: IntoIterator<Item = ((N, N), C)>,
{
    edmonds_karp::<N, C, IC, SparseCapacity<C>>(vertices, source, sink, caps)
}

/// Representation of capacity and flow data.
pub trait EdmondsKarp<C: Copy + Zero + Signed + Ord + Bounded> {
    /// Create a new empty structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than `size`.
    fn new(size: usize, source: usize, sink: usize) -> Self
    where
        Self: Sized;

    /// Create a new populated structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows in the `capacities` matrix, or it the matrix is not
    /// a square one.
    fn from_matrix(source: usize, sink: usize, capacities: Matrix<C>) -> Self
    where
        Self: Sized;

    /// Create a new populated structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows in the square matrix created from the `capacities` vector.
    #[must_use]
    fn from_vec(source: usize, sink: usize, capacities: Vec<C>) -> Self
    where
        Self: Sized,
    {
        Self::from_matrix(source, sink, Matrix::square_from_vec(capacities).unwrap())
    }

    /// Common data.
    fn common(&self) -> &Common<C>;

    /// Mutable common data.
    fn common_mut(&mut self) -> &mut Common<C>;

    /// Number of nodes.
    fn size(&self) -> usize {
        self.common().size
    }

    /// Source.
    fn source(&self) -> usize {
        self.common().source
    }

    /// Sink.
    fn sink(&self) -> usize {
        self.common().sink
    }

    /// List of successors with positive residual capacity and this capacity.
    fn residual_successors(&self, from: usize) -> Vec<(usize, C)>;

    /// Residual capacity between two nodes.
    fn residual_capacity(&self, from: usize, to: usize) -> C;

    /// Flow between two nodes.
    fn flow(&self, from: usize, to: usize) -> C;

    /// All positive flows starting from a node.
    fn flows_from(&self, from: usize) -> Vec<usize>;

    /// All flows between nodes.
    fn flows(&self) -> Vec<((usize, usize), C)>;

    /// Set capacity between two nodes.
    fn set_capacity(&mut self, from: usize, to: usize, capacity: C) {
        let flow = self.flow(from, to);
        let delta = capacity - (self.residual_capacity(from, to) + flow);
        if capacity < flow {
            let to_cancel = flow - capacity;
            self.add_flow(to, from, to_cancel);
            let source = self.source();
            self.cancel_flow(source, from, to_cancel);
            let sink = self.sink();
            self.cancel_flow(to, sink, to_cancel);
            self.common_mut().total_capacity = self.common().total_capacity - to_cancel;
        }
        self.add_residual_capacity(from, to, delta);
    }

    /// Add a given flow between two nodes. This should not be used
    /// directly.
    fn add_flow(&mut self, from: usize, to: usize, capacity: C);

    /// Get total capacity.
    fn total_capacity(&self) -> C {
        self.common().total_capacity
    }

    /// Add some residual capacity.
    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C);

    /// Set total capacity.
    fn set_total_capacity(&mut self, capacity: C) {
        self.common_mut().total_capacity = capacity;
    }

    /// Do not request the detailed flows as a result. The returned
    /// flows will be an empty vector.
    fn omit_detailed_flows(&mut self) {
        self.common_mut().detailed_flows = false;
    }

    /// Are detailed flows requested?
    fn detailed_flows(&self) -> bool {
        self.common().detailed_flows
    }

    /// Compute the maximum flow.
    fn augment(&mut self) -> EKFlows<usize, C> {
        let size = self.size();
        let source = self.source();
        let sink = self.sink();
        let mut parents = Vec::with_capacity(size);
        parents.resize(size, None);
        let mut path_capacity = Vec::with_capacity(size);
        path_capacity.resize(size, C::max_value());
        let mut to_see = VecDeque::new();
        'augment: loop {
            to_see.clear();
            to_see.push_back(source);
            while let Some(node) = to_see.pop_front() {
                let capacity_so_far = path_capacity[node];
                for (successor, residual) in self.residual_successors(node).iter().copied() {
                    if successor == source || parents[successor].is_some() {
                        continue;
                    }
                    parents[successor] = Some(node);
                    path_capacity[successor] = if capacity_so_far < residual {
                        capacity_so_far
                    } else {
                        residual
                    };
                    if successor == sink {
                        let mut n = sink;
                        while n != source {
                            let p = parents[n].unwrap();
                            self.add_flow(p, n, path_capacity[sink]);
                            n = p;
                        }
                        let total = self.total_capacity();
                        self.set_total_capacity(total + path_capacity[sink]);
                        parents.clear();
                        parents.resize(size, None);
                        path_capacity.clear();
                        path_capacity.resize(size, C::max_value());
                        continue 'augment;
                    }
                    to_see.push_back(successor);
                }
            }
            break;
        }
        if self.detailed_flows() {
            (self.flows(), self.total_capacity())
        } else {
            (Vec::new(), self.total_capacity())
        }
    }

    /// Internal: cancel a flow capacity between two nodes.
    fn cancel_flow(&mut self, from: usize, to: usize, mut capacity: C) {
        if from == to {
            return;
        }
        while capacity > Zero::zero() {
            if let Some(path) = bfs(&from, |&n| self.flows_from(n).into_iter(), |&n| n == to) {
                let path = path
                    .clone()
                    .into_iter()
                    .zip(path.into_iter().skip(1))
                    .collect::<Vec<_>>();
                let mut max_cancelable = path
                    .iter()
                    .map(|&(src, dst)| self.flow(src, dst))
                    .max()
                    .unwrap();
                if max_cancelable > capacity {
                    max_cancelable = capacity;
                }
                for (src, dst) in path {
                    self.add_flow(dst, src, max_cancelable);
                }
                capacity = capacity - max_cancelable;
            } else {
                unreachable!("no flow to cancel");
            }
        }
    }
}

/// Common fields.
#[derive(Clone, Debug)]
pub struct Common<C> {
    size: usize,
    source: usize,
    sink: usize,
    total_capacity: C,
    detailed_flows: bool,
}

/// Sparse capacity and flow data.
#[derive(Clone, Debug)]
pub struct SparseCapacity<C> {
    common: Common<C>,
    flows: BTreeMap<usize, BTreeMap<usize, C>>,
    residuals: BTreeMap<usize, BTreeMap<usize, C>>,
}

unsafe impl<C: Send> Send for SparseCapacity<C> {}

impl<C: Copy + Eq + Zero + Signed + Bounded + Ord> SparseCapacity<C> {
    fn set_value(data: &mut BTreeMap<usize, BTreeMap<usize, C>>, from: usize, to: usize, value: C) {
        let to_remove = {
            let sub = data.entry(from).or_insert_with(BTreeMap::new);
            if value == Zero::zero() {
                sub.remove(&to);
            } else {
                sub.insert(to, value);
            }
            sub.is_empty()
        };
        if to_remove {
            data.remove(&from);
        }
    }

    fn get_value(data: &BTreeMap<usize, BTreeMap<usize, C>>, from: usize, to: usize) -> C {
        data.get(&from)
            .and_then(|ns| ns.get(&to).copied())
            .unwrap_or_else(Zero::zero)
    }
}

impl<C: Copy + Zero + Signed + Eq + Ord + Bounded> EdmondsKarp<C> for SparseCapacity<C> {
    fn new(size: usize, source: usize, sink: usize) -> Self {
        assert!(source < size, "source is greater or equal than size");
        assert!(sink < size, "sink is greater or equal than size");
        Self {
            common: Common {
                size,
                source,
                sink,
                total_capacity: Zero::zero(),
                detailed_flows: true,
            },
            flows: BTreeMap::new(),
            residuals: BTreeMap::new(),
        }
    }

    #[must_use]
    fn from_matrix(source: usize, sink: usize, capacities: Matrix<C>) -> Self {
        assert!(
            capacities.is_square(),
            "capacities matrix is not a square one"
        );
        let size = capacities.rows;
        assert!(source < size, "source is greater or equal than matrix side");
        assert!(sink < size, "sink is greater or equal than matrix side");
        let mut result = Self::new(size, source, sink);
        for from in 0..size {
            for to in 0..size {
                let capacity = capacities[&(from, to)];
                if capacity > Zero::zero() {
                    result.set_capacity(from, to, capacity);
                }
            }
        }
        result
    }

    fn common(&self) -> &Common<C> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut Common<C> {
        &mut self.common
    }

    fn residual_successors(&self, from: usize) -> Vec<(usize, C)> {
        self.residuals.get(&from).map_or_else(Vec::new, |ns| {
            ns.iter()
                .filter_map(|(&n, &c)| if c > Zero::zero() { Some((n, c)) } else { None })
                .collect()
        })
    }

    fn residual_capacity(&self, from: usize, to: usize) -> C {
        Self::get_value(&self.residuals, from, to)
    }

    fn flow(&self, from: usize, to: usize) -> C {
        Self::get_value(&self.flows, from, to)
    }

    fn flows(&self) -> Vec<((usize, usize), C)> {
        self.flows
            .clone()
            .into_iter()
            .flat_map(|(k, vs)| {
                vs.into_iter().filter_map(move |(v, c)| {
                    if c > Zero::zero() {
                        Some(((k, v), c))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn add_flow(&mut self, from: usize, to: usize, capacity: C) {
        let direct = self.flow(from, to) + capacity;
        Self::set_value(&mut self.flows, from, to, direct);
        Self::set_value(&mut self.flows, to, from, -direct);
        self.add_residual_capacity(from, to, -capacity);
        self.add_residual_capacity(to, from, capacity);
    }

    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C) {
        let new_capacity = self.residual_capacity(from, to) + capacity;
        Self::set_value(&mut self.residuals, from, to, new_capacity);
    }

    fn flows_from(&self, n: usize) -> Vec<usize> {
        self.flows.get(&n).map_or_else(Vec::new, |ns| {
            ns.iter()
                .filter_map(|(&o, &c)| if c > Zero::zero() { Some(o) } else { None })
                .collect()
        })
    }
}

/// Dense capacity and flow data.
#[derive(Clone, Debug)]
pub struct DenseCapacity<C> {
    common: Common<C>,
    residuals: Matrix<C>,
    flows: Matrix<C>,
}

unsafe impl<C: Send> Send for DenseCapacity<C> {}

impl<C: Copy + Zero + Signed + Ord + Bounded> EdmondsKarp<C> for DenseCapacity<C> {
    #[must_use]
    fn new(size: usize, source: usize, sink: usize) -> Self {
        assert!(source < size, "source is greater or equal than size");
        assert!(sink < size, "sink is greater or equal than size");
        Self {
            common: Common {
                size,
                source,
                sink,
                total_capacity: Zero::zero(),
                detailed_flows: true,
            },
            residuals: Matrix::new(size, size, Zero::zero()),
            flows: Matrix::new(size, size, Zero::zero()),
        }
    }

    #[must_use]
    fn from_matrix(source: usize, sink: usize, capacities: Matrix<C>) -> Self {
        assert!(
            capacities.is_square(),
            "capacities matrix is not a square one"
        );
        let size = capacities.rows;
        assert!(source < size, "source is greater or equal than matrix side");
        assert!(sink < size, "sink is greater or equal than matrix side");
        Self {
            common: Common {
                size,
                source,
                sink,
                total_capacity: Zero::zero(),
                detailed_flows: true,
            },
            residuals: capacities,
            flows: Matrix::new(size, size, Zero::zero()),
        }
    }

    fn common(&self) -> &Common<C> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut Common<C> {
        &mut self.common
    }

    fn residual_successors(&self, from: usize) -> Vec<(usize, C)> {
        (0..self.common.size)
            .filter_map(|n| {
                let residual = self.residual_capacity(from, n);
                if residual > Zero::zero() {
                    Some((n, residual))
                } else {
                    None
                }
            })
            .collect()
    }

    fn residual_capacity(&self, from: usize, to: usize) -> C {
        self.residuals[&(from, to)]
    }

    fn flow(&self, from: usize, to: usize) -> C {
        self.flows[&(from, to)]
    }

    fn flows(&self) -> Vec<((usize, usize), C)> {
        iproduct!(0..self.size(), 0..self.size())
            .filter_map(|(from, to)| {
                let flow = self.flow(from, to);
                if flow > Zero::zero() {
                    Some(((from, to), flow))
                } else {
                    None
                }
            })
            .collect()
    }

    fn add_flow(&mut self, from: usize, to: usize, capacity: C) {
        self.flows[&(from, to)] = self.flows[&(from, to)] + capacity;
        self.flows[&(to, from)] = self.flows[&(to, from)] - capacity;
        self.residuals[&(from, to)] = self.residuals[&(from, to)] - capacity;
        self.residuals[&(to, from)] = self.residuals[&(to, from)] + capacity;
    }

    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C) {
        self.residuals[&(from, to)] = self.residual_capacity(from, to) + capacity;
    }

    fn flows_from(&self, from: usize) -> Vec<usize> {
        (0..self.common.size)
            .filter(|to| self.flow(from, *to) > Zero::zero())
            .collect()
    }
}
