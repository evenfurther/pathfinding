#![deny(missing_docs)]

//! This crate implements several pathfinding, flow, and graph algorithms.

extern crate fixedbitset;
#[macro_use]
extern crate itertools;
pub extern crate num_traits;

mod astar;
mod bfs;
mod dfs;
mod dijkstra;
mod edmonds_karp;
mod fringe;
mod idastar;
mod kuhn_munkres;
mod matrix;

pub use astar::*;
pub use bfs::*;
pub use dfs::*;
pub use dijkstra::*;
pub use edmonds_karp::*;
pub use fringe::*;
pub use idastar::*;
pub use kuhn_munkres::*;
pub use matrix::*;

use std::collections::HashMap;
use std::hash::Hash;

fn reverse_path<N: Eq + Hash + Clone>(parents: &HashMap<N, N>, start: N) -> Vec<N> {
    let mut path = vec![start];
    while let Some(parent) = parents.get(path.last().unwrap()).cloned() {
        path.push(parent);
    }
    path.into_iter().rev().collect()
}
