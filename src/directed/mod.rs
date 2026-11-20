//! Algorithms for directed graphs.

use super::FxIndexMap;
use indexmap::map::Entry::{Occupied, Vacant};
use std::hash::Hash;

pub mod astar;
pub mod bfs;
pub mod count_paths;
pub mod cycle_detection;
pub mod dfs;
pub mod dijkstra;
pub mod edmonds_karp;
pub mod fringe;
pub mod idastar;
pub mod iddfs;
pub mod strongly_connected_components;
pub mod topological_sort;
pub mod yen;

/// Outcome of offering a node to [`relax`].
#[derive(Clone, Copy)]
enum Relaxed {
    /// The node was seen for the first time, and stored at this index.
    New(usize),
    /// A cheaper path to the node, stored at this index, was found.
    Improved(usize),
}

impl Relaxed {
    #[inline]
    const fn index(self) -> usize {
        match self {
            Self::New(index) | Self::Improved(index) => index,
        }
    }
}

/// Record that `node` can be reached at `cost` through the node at index `parent` in
/// `parents`, unless it is already known to be reachable at no greater cost.
///
/// This is the edge relaxation step shared by the best-first searches, where `parents`
/// maps every node seen so far to the index of its parent and the cost of reaching it.
#[inline]
fn relax<N, C>(
    parents: &mut FxIndexMap<N, (usize, C)>,
    node: N,
    parent: usize,
    cost: C,
) -> Option<Relaxed>
where
    N: Eq + Hash,
    C: Ord,
{
    match parents.entry(node) {
        Vacant(e) => {
            let index = e.index();
            e.insert((parent, cost));
            Some(Relaxed::New(index))
        }
        Occupied(mut e) => {
            if e.get().1 > cost {
                let index = e.index();
                e.insert((parent, cost));
                Some(Relaxed::Improved(index))
            } else {
                None
            }
        }
    }
}

fn reverse_path<N, V, F>(parents: &FxIndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let mut path = Vec::new();
    let mut i = start;
    while let Some((node, value)) = parents.get_index(i) {
        path.push(node.clone());
        i = parent(value);
    }
    path.reverse();
    path
}
