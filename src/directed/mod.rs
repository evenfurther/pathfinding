//! Algorithms for directed graphs.

use super::FxIndexMap;
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
pub mod theta_star;
pub mod topological_sort;
pub mod yen;

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
