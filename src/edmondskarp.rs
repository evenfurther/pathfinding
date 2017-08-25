use ndarray::{Array1, Array2, Axis, Zip, indices_of};
use num_traits::{Bounded, Zero};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::Sub;

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

pub fn edmondskarp<N, C, IC>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Clone,
    C: Sub<Output = C> + Zero + Bounded + PartialOrd + Copy,
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
    let (paths, max) = edmondskarp_matrix(reverse[source], reverse[sink], &capacities);
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
/// # Panics
///
/// This function will panic if the `capacities` matrix is not a square matrix.

pub fn edmondskarp_matrix<C>(
    source: usize,
    sink: usize,
    capacities: &Array2<C>,
) -> EKFlows<usize, C>
where
    C: Sub<Output = C> + Zero + Bounded + PartialOrd + Copy,
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
    let mut capacity = Array1::from_elem(size, C::max_value());
    let mut residuals = Array1::from_elem(size, Zero::zero());

    // Repeatidly look for an augmenting path.
    'augment: loop {
        to_see.clear();
        to_see.push_back(source);
        while let Some(node) = to_see.pop_front() {
            let capacity_so_far = capacity[node];
            Zip::from(&mut residuals)
                .and(capacities.subview(Axis(0), node))
                .and(flows.subview(Axis(0), node))
                .apply(|r, &c, &f| *r = c - f);
            for neighbour in 0..size {
                if residuals[neighbour] <= Zero::zero() || parents[neighbour].is_some() {
                    continue;
                }
                parents[neighbour] = Some(node);
                let cap = if capacity_so_far < residuals[neighbour] {
                    capacity_so_far
                } else {
                    residuals[neighbour]
                };
                if neighbour == sink {
                    let mut n = neighbour;
                    while n != source {
                        let p = parents[n].unwrap();
                        flows[[p, n]] = flows[[p, n]] + cap;
                        flows[[n, p]] = flows[[n, p]] - cap;
                        n = p;
                    }
                    total_capacity = total_capacity + cap;
                    parents.fill(None);
                    capacity.fill(C::max_value());
                    continue 'augment;
                }
                capacity[neighbour] = cap;
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
