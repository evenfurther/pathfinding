use num_traits::{Bounded, Signed, Zero};
use square_matrix::SquareMatrix;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hash::Hash;
use std::mem::swap;

/// Type alias for Edmonds-Karp result.
pub type EKFlows<N, C> = (Vec<((N, N), C)>, C);

/// Compute the maximum flow that can go through a directed graph using the
/// [Edmonds Karp algorithm](https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm).
///
/// A maximum flow going from `source` to `sink` will be computed, and the various
/// flow values along with the total will be returned. A dense matrix will be used
/// to represent the edges.
///
/// - `vertices` is the collection of vertices in the graph.
/// - `source` is the source node (the origin of the flow).
/// - `sink` is the sink node (the target of the flow).
/// - `caps` is an iterator-like object describing the positive capacities between the
///   nodes.
///
/// Note that the capacity type C must be signed as the algorithm has to deal with
/// negative residual capacities.
///
/// By creating an `EdmondsKarp` structure, it is possible to adjust the capacities
/// after computing the maximum flow and rerun the algorithm without starting from
/// scratch.
///
/// # Panics
///
/// This function panics if `source` or `sink` is not found in `vertices`.
pub fn edmonds_karp_dense<N, C, IC>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Bounded + Signed + PartialOrd + Clone,
    IC: IntoIterator<Item = ((N, N), C)>,
{
    // Build a correspondance between N and 0..vertices.len() so that we can
    // work with matrices more easily.
    let size = vertices.len();
    let reverse = (0..size)
        .into_iter()
        .map(|i| (vertices[i].clone(), i))
        .collect::<HashMap<_, _>>();
    let mut capacities = DenseCapacity::new(size, reverse[source], reverse[sink]);
    for ((from, to), capacity) in caps {
        capacities.set_capacity(reverse[&from], reverse[&to], capacity);
    }
    let (paths, max) = capacities.augment();
    (
        paths
            .into_iter()
            .map(|((a, b), c)| {
                ((vertices[a].clone(), vertices[b].clone()), c)
            })
            .collect(),
        max,
    )
}

/// Compute the maximum flow that can go through a directed graph using the
/// [Edmonds Karp algorithm](https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm).
///
/// A maximum flow going from `source` to `sink` will be computed, and the various
/// flow values along with the total will be returned. A sparse representation will be
/// used to represent the edges.
///
/// - `vertices` is the collection of vertices in the graph.
/// - `source` is the source node (the origin of the flow).
/// - `sink` is the sink node (the target of the flow).
/// - `caps` is an iterator-like object describing the positive capacities between the
///   nodes.
///
/// Note that the capacity type C must be signed as the algorithm has to deal with
/// negative residual capacities.
///
/// By creating an `EdmondsKarp` structure, it is possible to adjust the capacities
/// after computing the maximum flow and rerun the algorithm without starting from
/// scratch.
///
/// # Panics
///
/// This function panics if `source` or `sink` is not found in `vertices`.
pub fn edmonds_karp_sparse<N, C, IC>(
    vertices: &[N],
    source: &N,
    sink: &N,
    caps: IC,
) -> EKFlows<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Bounded + Signed + PartialOrd + Clone + Eq,
    IC: IntoIterator<Item = ((N, N), C)>,
{
    // Build a correspondance between N and 0..vertices.len() so that we can
    // work with matrices more easily.
    let size = vertices.len();
    let reverse = (0..size)
        .into_iter()
        .map(|i| (vertices[i].clone(), i))
        .collect::<HashMap<_, _>>();
    let mut capacities = SparseCapacity::new(size, reverse[source], reverse[sink]);
    for ((from, to), capacity) in caps {
        capacities.set_capacity(reverse[&from], reverse[&to], capacity);
    }
    let (paths, max) = capacities.augment();
    (
        paths
            .into_iter()
            .map(|((a, b), c)| {
                ((vertices[a].clone(), vertices[b].clone()), c)
            })
            .collect(),
        max,
    )
}

/// Representation of capacity and flow data.
pub trait EdmondsKarp<C: Clone + Zero + Signed + PartialOrd + Bounded> {
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

    /// List of neighbours with positive residual capacity and this capacity.
    fn residual_neighbours(&self, from: usize) -> Vec<(usize, C)>;

    /// Residual capacity between two nodes.
    fn residual_capacity(&self, from: usize, to: usize) -> C;

    /// Flow between two nodes.
    fn flow(&self, from: usize, to: usize) -> C;

    /// All flows between nodes.
    fn flows(&self) -> Vec<((usize, usize), C)>;

    /// Set capacity between two nodes. This might trigger a reset
    /// of the already computed flows.
    fn set_capacity(&mut self, from: usize, to: usize, capacity: C);

    /// Add a given flow between two nodes. This should not be used
    /// directly.
    fn add_flow(&mut self, from: usize, to: usize, capacity: C);

    /// Reset the flows if needed.
    fn reset_if_needed(&mut self);

    /// Get total capacity.
    fn total_capacity(&self) -> C {
        self.common().total_capacity.clone()
    }

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
        self.reset_if_needed();
        let mut parents = Vec::with_capacity(size);
        parents.resize(size, None);
        let mut path_capacity = Vec::with_capacity(size);
        path_capacity.resize(size, C::max_value());
        let mut to_see = VecDeque::new();
        'augment: loop {
            to_see.clear();
            to_see.push_back(source);
            while let Some(node) = to_see.pop_front() {
                let capacity_so_far = path_capacity[node].clone();
                for (neighbour, residual) in self.residual_neighbours(node).iter().cloned() {
                    if neighbour == source || parents[neighbour].is_some() {
                        continue;
                    }
                    parents[neighbour] = Some(node);
                    path_capacity[neighbour] = if capacity_so_far.clone() < residual.clone() {
                        capacity_so_far.clone()
                    } else {
                        residual
                    };
                    if neighbour == sink {
                        let mut n = sink;
                        while n != source {
                            let p = parents[n].unwrap();
                            self.add_flow(p, n, path_capacity[sink].clone());
                            n = p;
                        }
                        let total = self.total_capacity();
                        self.set_total_capacity(total + path_capacity[sink].clone());
                        parents.clear();
                        parents.resize(size, None);
                        path_capacity.clear();
                        path_capacity.resize(size, C::max_value());
                        continue 'augment;
                    }
                    to_see.push_back(neighbour);
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
}

/// Common fields.
#[derive(Debug)]
pub struct Common<C> {
    size: usize,
    source: usize,
    sink: usize,
    total_capacity: C,
    needs_reset: bool,
    detailed_flows: bool,
}

/// Sparse capacity and flow data.
#[derive(Debug)]
pub struct SparseCapacity<C> {
    common: Common<C>,
    flows: BTreeMap<usize, BTreeMap<usize, C>>,
    residuals: BTreeMap<usize, BTreeMap<usize, C>>,
}

impl<C: Clone + Eq + Zero + Signed + Bounded + PartialOrd> SparseCapacity<C> {
    /// Create a new sparse structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than `size`.
    pub fn new(size: usize, source: usize, sink: usize) -> SparseCapacity<C> {
        assert!(source < size, "source is greater or equal than size");
        assert!(sink < size, "sink is greater or equal than size");
        SparseCapacity {
            common: Common {
                size: size,
                source: source,
                sink: sink,
                total_capacity: Zero::zero(),
                needs_reset: false,
                detailed_flows: true,
            },
            flows: BTreeMap::new(),
            residuals: BTreeMap::new(),
        }
    }

    /// Create a new sparse structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows in the `capacities` matrix.
    pub fn from_matrix(
        source: usize,
        sink: usize,
        capacities: SquareMatrix<C>,
    ) -> SparseCapacity<C> {
        let size = capacities.size;
        assert!(source < size, "source is greater or equal than matrix side");
        assert!(sink < size, "sink is greater or equal than matrix side");
        let mut result = Self::new(size, source, sink);
        for from in 0..size {
            for to in 0..size {
                let capacity = capacities[&(from, to)].clone();
                if capacity > Zero::zero() {
                    result.set_capacity(from, to, capacity);
                }
            }
        }
        result
    }

    /// Create a new sparse structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows of the newly created square capacities matrix, or when the
    /// data is not square.
    pub fn from_vec(source: usize, sink: usize, data: Vec<C>) -> SparseCapacity<C> {
        Self::from_matrix(source, sink, SquareMatrix::from_vec(data))
    }

    fn set_value(data: &mut BTreeMap<usize, BTreeMap<usize, C>>, from: usize, to: usize, value: C) {
        let to_remove = {
            let sub = data.entry(from).or_insert(BTreeMap::new());
            if value != Zero::zero() {
                sub.insert(to, value.clone());
            } else {
                sub.remove(&to);
            }
            sub.is_empty()
        };
        if to_remove {
            data.remove(&from);
        }
    }

    fn get_value(data: &BTreeMap<usize, BTreeMap<usize, C>>, from: usize, to: usize) -> C {
        data.get(&from)
            .and_then(|ns| ns.get(&to).cloned())
            .unwrap_or(Zero::zero())
    }

    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C) {
        let new_capacity = self.residual_capacity(from, to) + capacity;
        Self::set_value(&mut self.residuals, from, to, new_capacity);
    }
}

impl<C: Clone + Zero + Signed + Eq + PartialOrd + Bounded> EdmondsKarp<C> for SparseCapacity<C> {
    fn common(&self) -> &Common<C> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut Common<C> {
        &mut self.common
    }

    fn residual_neighbours(&self, from: usize) -> Vec<(usize, C)> {
        self.residuals
            .get(&from)
            .map(|ns| {
                ns.clone()
                    .into_iter()
                    .filter(|&(_, ref c)| c.clone() > Zero::zero())
                    .collect()
            })
            .unwrap_or(Vec::new())
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

    fn set_capacity(&mut self, from: usize, to: usize, capacity: C) {
        let flow = self.flow(from, to);
        Self::set_value(
            &mut self.residuals,
            from,
            to,
            capacity.clone() - flow.clone(),
        );
        if capacity < flow {
            self.common.needs_reset = true;
        }
    }

    fn add_flow(&mut self, from: usize, to: usize, capacity: C) {
        let direct = self.flow(from, to) + capacity.clone();
        Self::set_value(&mut self.flows, from, to, direct.clone());
        Self::set_value(&mut self.flows, to, from, -direct);
        self.add_residual_capacity(from, to, -capacity.clone());
        self.add_residual_capacity(to, from, capacity);
    }

    fn reset_if_needed(&mut self) {
        if self.common.needs_reset {
            let mut flows = BTreeMap::new();
            swap(&mut self.flows, &mut flows);
            for (from, ns) in flows {
                for (to, capacity) in ns {
                    self.add_residual_capacity(from, to, capacity);
                }
            }
            self.common.total_capacity = Zero::zero();
            self.common.needs_reset = false;
        }
    }
}

/// Dense capacity and flow data.
#[derive(Debug)]
pub struct DenseCapacity<C> {
    common: Common<C>,
    capacities: SquareMatrix<C>,
    flows: SquareMatrix<C>,
}

impl<C: Clone + Zero> DenseCapacity<C> {
    /// Create a new dense structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than `size`.
    pub fn new(size: usize, source: usize, sink: usize) -> DenseCapacity<C> {
        assert!(source < size, "source is greater or equal than size");
        assert!(sink < size, "sink is greater or equal than size");
        DenseCapacity {
            common: Common {
                size: size,
                source: source,
                sink: sink,
                total_capacity: Zero::zero(),
                needs_reset: false,
                detailed_flows: true,
            },
            capacities: SquareMatrix::new(size, Zero::zero()),
            flows: SquareMatrix::new(size, Zero::zero()),
        }
    }

    /// Create a new dense structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows in the `capacities` matrix.
    pub fn from_matrix(
        source: usize,
        sink: usize,
        capacities: SquareMatrix<C>,
    ) -> DenseCapacity<C> {
        let size = capacities.size;
        assert!(source < size, "source is greater or equal than matrix side");
        assert!(sink < size, "sink is greater or equal than matrix side");
        DenseCapacity {
            common: Common {
                size: size,
                source: source,
                sink: sink,
                total_capacity: Zero::zero(),
                needs_reset: false,
                detailed_flows: true,
            },
            capacities: capacities,
            flows: SquareMatrix::new(size, Zero::zero()),
        }
    }

    /// Create a new dense structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows of the newly created square capacities matrix, or when the
    /// data is not square.
    pub fn from_vec(source: usize, sink: usize, data: Vec<C>) -> DenseCapacity<C> {
        Self::from_matrix(source, sink, SquareMatrix::from_vec(data))
    }
}

impl<C: Clone + Zero + Signed + PartialOrd + Bounded> EdmondsKarp<C> for DenseCapacity<C> {
    fn common(&self) -> &Common<C> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut Common<C> {
        &mut self.common
    }

    fn residual_neighbours(&self, from: usize) -> Vec<(usize, C)> {
        (0..self.capacities.size)
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
        self.capacities[&(from, to)].clone() - self.flows[&(from, to)].clone()
    }

    fn flow(&self, from: usize, to: usize) -> C {
        self.flows[&(from, to)].clone()
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

    fn set_capacity(&mut self, from: usize, to: usize, capacity: C) {
        self.capacities[&(from, to)] = capacity.clone();
        if capacity < self.flows[&(from, to)].clone() {
            self.common.needs_reset = true;
        }
    }

    fn add_flow(&mut self, from: usize, to: usize, capacity: C) {
        self.flows[&(from, to)] = self.flows[&(from, to)].clone() + capacity.clone();
        self.flows[&(to, from)] = self.flows[&(to, from)].clone() - capacity.clone();
    }

    fn reset_if_needed(&mut self) {
        if self.common.needs_reset {
            self.flows.fill(Zero::zero());
            self.common.total_capacity = Zero::zero();
            self.common.needs_reset = false;
        }
    }
}
