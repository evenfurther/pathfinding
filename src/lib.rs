#![deny(missing_docs)]

//! This crate implements several pathfinding, flow, and graph algorithms.

extern crate fixedbitset;
extern crate indexmap;
#[macro_use]
extern crate itertools;
pub extern crate num_traits;

pub mod astar;
pub mod bfs;
pub mod connected_components;
pub mod dfs;
pub mod dijkstra;
pub mod edmonds_karp;
pub mod fringe;
pub mod grid;
pub mod idastar;
pub mod kuhn_munkres;
pub mod matrix;
pub mod topological_sort;
pub mod utils;

/// Export all public functions and structures for an easy access.
pub mod prelude {
    pub use astar::*;
    pub use bfs::*;
    pub use connected_components::*;
    pub use dfs::*;
    pub use dijkstra::*;
    pub use edmonds_karp::*;
    pub use fringe::*;
    pub use grid::*;
    pub use idastar::*;
    pub use kuhn_munkres::*;
    pub use matrix::*;
    pub use topological_sort::*;
    pub use utils::*;
}

use indexmap::IndexMap;
use std::hash::Hash;

fn reverse_path<N, V, F>(parents: &IndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let path = itertools::unfold(start, |i| {
        parents.get_index(*i).map(|(node, value)| {
            *i = parent(value);
            node
        })
    }).collect::<Vec<&N>>();

    path.into_iter().rev().cloned().collect()
}
