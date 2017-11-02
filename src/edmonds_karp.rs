use ndarray::{indices_of, Array1, Array2};
use num_traits::{Bounded, Signed, Zero};
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
    let mut capacities = Array2::<C>::zeros((size, size));
    for ((from, to), capacity) in caps {
        capacities[[reverse[&from], reverse[&to]]] = capacity;
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
    capacities: &Array2<C>,
) -> EKFlows<usize, C>
where
    C: Zero + Signed + Bounded + PartialOrd + Copy,
{
    let mut ek = EdmondsKarp::new(source, sink, capacities);
    ek.augment()
}

/// Structure holding Edmonds-Karp algorithm internal variables. Only
/// `capacities` may be manipulated from outside in order to gradually
/// change the capacities.
pub struct EdmondsKarp<'a, C: 'a> {
    source: usize,
    sink: usize,
    size: usize,
    capacities: &'a Array2<C>,
    flows: Array2<C>,
    total_capacity: C,
    parents: Array1<Option<usize>>,
    path_capacity: Array1<C>,
}

impl<'a, C> EdmondsKarp<'a, C>
where
    C: 'a + Zero + Signed + Bounded + PartialOrd + Copy,
{
    /// Create a new `EdmondsKarp` structure.
    pub fn new(source: usize, sink: usize, capacities: &'a Array2<C>) -> EdmondsKarp<'a, C> {
        let size = capacities.shape()[0];
        assert_eq!(capacities.shape()[0], capacities.shape()[1]);
        EdmondsKarp {
            source: source,
            sink: sink,
            size: size,
            capacities: capacities,
            flows: Array2::<C>::zeros((size, size)),
            total_capacity: Zero::zero(),
            parents: Array1::from_elem(size, None),
            path_capacity: Array1::from_elem(size, C::max_value()),
        }
    }

    /// Reset all flows because capacities have changed.
    pub fn reset_flows(&mut self) {
        self.flows.fill(Zero::zero());
        self.total_capacity = Zero::zero();
    }

    /// Reset all flows in case capacities have been reduced to below the
    /// existing flow. Return `true` if a reset has been performed.
    pub fn reset_if_needed(&mut self) -> bool {
        (0..self.size).any(|from| {
            (0..self.size).any(|to| self.reset_after_change(from, to))
        })
    }

    /// Reset all flows in case the given capacity has been reduced to below
    /// the existing flow value. Return `true` if a reset has been performed.
    pub fn reset_after_change(&mut self, from: usize, to: usize) -> bool {
        if self.capacities[[from, to]] < self.flows[[from, to]] {
            self.reset_flows();
            true
        } else {
            false
        }
    }

    /// Reset all flows in case any of the given capacity has been reduced to below
    /// the existing flow value. Return `true` if a reset has been performed.
    pub fn reset_after_changes(&mut self, changes: &[(usize, usize)]) -> bool {
        changes
            .iter()
            .any(|&(from, to)| self.reset_after_change(from, to))
    }

    /// Augment paths so that the flow is maximal.
    pub fn augment(&mut self) -> EKFlows<usize, C> {
        if self.source >= self.size || self.sink >= self.size {
            return (vec![], Zero::zero());
        }
        let mut to_see = VecDeque::new();
        'augment: loop {
            to_see.clear();
            to_see.push_back(self.source);
            while let Some(node) = to_see.pop_front() {
                let capacity_so_far = self.path_capacity[node];
                for neighbour in 0..self.size {
                    let residual =
                        self.capacities[[node, neighbour]] - self.flows[[node, neighbour]];
                    if neighbour == self.source || residual <= Zero::zero()
                        || self.parents[neighbour].is_some()
                    {
                        continue;
                    }
                    self.parents[neighbour] = Some(node);
                    self.path_capacity[neighbour] = if capacity_so_far < residual {
                        capacity_so_far
                    } else {
                        residual
                    };
                    if neighbour == self.sink {
                        let mut n = self.sink;
                        while n != self.source {
                            let p = self.parents[n].unwrap();
                            self.flows[[p, n]] = self.flows[[p, n]] + self.path_capacity[self.sink];
                            self.flows[[n, p]] = self.flows[[n, p]] - self.path_capacity[self.sink];
                            n = p;
                        }
                        self.total_capacity = self.total_capacity + self.path_capacity[self.sink];
                        self.parents.fill(None);
                        self.path_capacity.fill(C::max_value());
                        continue 'augment;
                    }
                    to_see.push_back(neighbour);
                }
            }
            self.parents.fill(None);
            self.path_capacity.fill(C::max_value());
            break;
        }
        (
            indices_of(&self.flows)
                .into_iter()
                .filter_map(|(a, b)| {
                    let f = self.flows[[a, b]];
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
