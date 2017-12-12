#![deny(missing_docs)]

//! This crate implements several pathfinding, flow, and graph algorithms.

extern crate fixedbitset;
#[macro_use]
extern crate itertools;
extern crate ordermap;
pub extern crate num_traits;

mod astar;
mod bfs;
mod connected_components;
mod dfs;
mod dijkstra;
mod edmonds_karp;
mod fringe;
mod idastar;
mod kuhn_munkres;
mod matrix;
mod topological_sort;

pub use astar::*;
pub use bfs::*;
pub use connected_components::*;
pub use dfs::*;
pub use dijkstra::*;
pub use edmonds_karp::*;
pub use fringe::*;
pub use idastar::*;
pub use kuhn_munkres::*;
pub use matrix::*;
pub use topological_sort::*;

use ordermap::OrderMap;
use std::hash::Hash;

fn reverse_path<N, V, F>(parents: &OrderMap<N, V>, parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: Fn(&V) -> usize,
{
    let path = itertools::unfold(start, |i| {
        parents.get_index(*i).map(|(node, value)| {
            *i = parent(value);
            node
        })
    }).collect::<Vec<&N>>();

    path.into_iter().rev().cloned().collect()
}
