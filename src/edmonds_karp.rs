use num_traits::{Bounded, Signed, Zero};
use square_matrix::SquareMatrix;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

type EKFlows<N, C> = (Vec<((N, N), C)>, C);

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
/// Note that the capacity type C must be signed as the algorithm has to deal with
/// negative residual capacities.
///
/// By creating an `EdmondsKarp` structure, it is possible to adjust the capacities
/// after computing the maximum flow and rerun the algorithm without starting from
/// scratch.

pub fn edmonds_karp<N, C, IC>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Bounded + Signed + PartialOrd + Copy,
    IC: IntoIterator<Item = ((N, N), C)>,
{
    // Build a correspondance between N and 0..vertices.len() so that we can
    // work with matrices more easily.
    let size = vertices.len();
    let reverse = (0..size)
        .into_iter()
        .map(|i| (vertices[i].clone(), i))
        .collect::<HashMap<_, _>>();
    let mut capacities = SquareMatrix::new(size, Zero::zero());
    for ((from, to), capacity) in caps {
        capacities[&(reverse[&from], reverse[&to])] = capacity;
    }
    let (paths, max) = edmonds_karp_matrix(reverse[source], reverse[sink], &capacities);
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
/// The graph is described via an adjacency matrix.
///
/// A maximum flow going from `source` to `sink` will be computed, and the various
/// flow values along with the total will be returned.
///
/// - `source` is the source node (the origin of the flow).
/// - `sink` is the sink node (the target of the flow).
/// - `capacities` is a matrix describing the capacities. `capacities[i][j]`
///   represents the non-negative capacity from `i` to `j`.
///
/// Note that the capacity type C must be signed as the algorithm has to deal with
/// negative residual capacities.
///
/// # Panics
///
/// This function will panic if the `capacities` matrix is not a square matrix.

pub fn edmonds_karp_matrix<C>(
    source: usize,
    sink: usize,
    capacities: &SquareMatrix<C>,
) -> EKFlows<usize, C>
where
    C: Zero + Signed + Bounded + PartialOrd + Copy,
{
    let mut ek = EdmondsKarp::new(capacities.size, source, sink);
    ek.augment(capacities)
}

/// Structure holding Edmonds-Karp algorithm internal variables. This is
/// not supposed to be manipulated from outside and must be treated as
/// opaque.
pub struct EdmondsKarp<C> {
    source: usize,
    sink: usize,
    size: usize,
    flows: SquareMatrix<C>,
    total_capacity: C,
}

impl<C> EdmondsKarp<C>
where
    C: Zero + Signed + Bounded + PartialOrd + Copy,
{
    /// Create a new `EdmondsKarp` structure.
    /// - `size` is the size of each dimension of the the capacities
    ///   square matrix.
    /// - `source` is the source node (the origin of the flow).
    /// - `sink` is the sink node (the target of the flow).
    pub fn new(size: usize, source: usize, sink: usize) -> EdmondsKarp<C> {
        EdmondsKarp {
            source: source,
            sink: sink,
            size: size,
            flows: SquareMatrix::new(size, Zero::zero()),
            total_capacity: Zero::zero(),
        }
    }

    /// Set a capacity and return `true` if this caused the existing flows
    /// to be reset.
    pub fn set_capacity(
        &mut self,
        capacities: &mut SquareMatrix<C>,
        from: usize,
        to: usize,
        capacity: C,
    ) -> bool {
        capacities[&(from, to)] = capacity;
        self.reset_after_change(capacities, from, to)
    }

    /// Reset all flows unconditionally because capacities have changed.
    pub fn reset_flows(&mut self) {
        self.flows.fill(Zero::zero());
        self.total_capacity = Zero::zero();
    }

    /// Reset all flows in case capacities have been reduced to below the
    /// existing flow. Return `true` if a reset has been performed since
    /// the last computation.
    pub fn reset_if_needed(&mut self, capacities: &SquareMatrix<C>) -> bool {
        (0..self.size).any(|from| {
            (0..self.size).any(|to| self.reset_after_change(capacities, from, to))
        })
    }

    /// Reset all flows in case the given capacity has been reduced to below
    /// the existing flow value. Return `true` if a reset has been performed
    /// since the last computation.
    pub fn reset_after_change(
        &mut self,
        capacities: &SquareMatrix<C>,
        from: usize,
        to: usize,
    ) -> bool {
        if self.total_capacity == Zero::zero() {
            true
        } else if capacities[&(from, to)] < self.flows[&(from, to)] {
            self.reset_flows();
            true
        } else {
            false
        }
    }

    /// Reset all flows in case any of the given capacity has been reduced to below
    /// the existing flow value. Return `true` if a reset has been performed since
    /// the last computation.
    pub fn reset_after_changes(
        &mut self,
        capacities: &SquareMatrix<C>,
        changes: &[(usize, usize)],
    ) -> bool {
        changes
            .iter()
            .any(|&(from, to)| self.reset_after_change(capacities, from, to))
    }

    /// Compute the maximum flow.
    pub fn augment(&mut self, capacities: &SquareMatrix<C>) -> EKFlows<usize, C> {
        assert_eq!(capacities.size, self.size);
        if self.source >= self.size || self.sink >= self.size {
            return (vec![], Zero::zero());
        }
        let mut parents = Vec::with_capacity(self.size);
        parents.resize(self.size, None);
        let mut path_capacity = Vec::with_capacity(self.size);
        path_capacity.resize(self.size, C::max_value());
        let mut to_see = VecDeque::new();
        'augment: loop {
            to_see.clear();
            to_see.push_back(self.source);
            while let Some(node) = to_see.pop_front() {
                let capacity_so_far = path_capacity[node];
                for neighbour in 0..self.size {
                    let residual = capacities[&(node, neighbour)] - self.flows[&(node, neighbour)];
                    if neighbour == self.source || residual <= Zero::zero()
                        || parents[neighbour].is_some()
                    {
                        continue;
                    }
                    parents[neighbour] = Some(node);
                    path_capacity[neighbour] = if capacity_so_far < residual {
                        capacity_so_far
                    } else {
                        residual
                    };
                    if neighbour == self.sink {
                        let mut n = self.sink;
                        while n != self.source {
                            let p = parents[n].unwrap();
                            self.flows[&(p, n)] = self.flows[&(p, n)] + path_capacity[self.sink];
                            self.flows[&(n, p)] = self.flows[&(n, p)] - path_capacity[self.sink];
                            n = p;
                        }
                        self.total_capacity = self.total_capacity + path_capacity[self.sink];
                        parents.clear();
                        parents.resize(self.size, None);
                        path_capacity.clear();
                        path_capacity.resize(self.size, C::max_value());
                        continue 'augment;
                    }
                    to_see.push_back(neighbour);
                }
            }
            break;
        }
        (
            iproduct!(0..self.flows.size, 0..self.flows.size)
                .filter_map(|(a, b)| {
                    let f = self.flows[&(a, b)];
                    if f > Zero::zero() {
                        Some(((a, b), f))
                    } else {
                        None
                    }
                })
                .collect(),
            self.total_capacity,
        )
    }
}
