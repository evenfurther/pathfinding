#![deny(missing_docs)]
//! This crate implements several pathfinding, flow, and graph algorithms.

extern crate fixedbitset;
extern crate indexmap;
#[macro_use]
extern crate itertools;
pub extern crate num_traits;

pub mod directed;
pub mod grid;
pub mod kuhn_munkres;
pub mod matrix;
pub mod undirected;
pub mod utils;

/// Export all public functions and structures for an easy access.
pub mod prelude {
    pub use crate::directed::astar::*;
    pub use crate::directed::bfs::*;
    pub use crate::directed::dfs::*;
    pub use crate::directed::dijkstra::*;
    pub use crate::directed::edmonds_karp::*;
    pub use crate::directed::fringe::*;
    pub use crate::directed::idastar::*;
    pub use crate::directed::iddfs::*;
    pub use crate::directed::strongly_connected_components::*;
    pub use crate::directed::topological_sort::*;
    pub use crate::grid::*;
    pub use crate::kuhn_munkres::*;
    pub use crate::matrix::*;
    pub use crate::undirected::connected_components::*;
    pub use crate::undirected::kruskal::*;
    pub use crate::utils::*;
}
