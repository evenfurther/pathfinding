use ndarray::{Array1, Array2, indices_of};
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
    let size = capacities.shape()[0];
    assert_eq!(capacities.shape()[0], capacities.shape()[1]);
    if source >= size || sink >= size {
        return (vec![], Zero::zero());
    }
    // Permanent evoluting data structures.
    let mut flows = Array2::<C>::zeros((size, size));
    let mut total_capacity = Zero::zero();
    let mut to_see = VecDeque::new();

    // Data which will be cleared for every path but
    // is allocated once.
    let mut parents = Array1::from_elem(size, None);
    let mut path_capacity = Array1::from_elem(size, C::max_value());

    // Repeatidly look for an augmenting path.
    'augment: loop {
        to_see.clear();
        to_see.push_back(source);
        while let Some(node) = to_see.pop_front() {
            let capacity_so_far = path_capacity[node];
            for neighbour in 0..size {
                let residual = capacities[[node, neighbour]] - flows[[node, neighbour]];
                if neighbour == source || residual <= Zero::zero() || parents[neighbour].is_some() {
                    continue;
                }
                parents[neighbour] = Some(node);
                path_capacity[neighbour] = if capacity_so_far < residual {
                    capacity_so_far
                } else {
                    residual
                };
                if neighbour == sink {
                    let mut n = sink;
                    while n != source {
                        let p = parents[n].unwrap();
                        flows[[p, n]] = flows[[p, n]] + path_capacity[sink];
                        flows[[n, p]] = flows[[n, p]] - path_capacity[sink];
                        n = p;
                    }
                    total_capacity = total_capacity + path_capacity[sink];
                    parents.fill(None);
                    path_capacity.fill(C::max_value());
                    continue 'augment;
                }
                to_see.push_back(neighbour);
            }
        }
        break;
    }
    (
        indices_of(&flows)
            .into_iter()
            .filter_map(|(a, b)| {
                let f = flows[[a, b]];
                if f > Zero::zero() {
                    Some(((a, b), f))
                } else {
                    None
                }
            })
            .collect(),
        total_capacity,
    )
}
