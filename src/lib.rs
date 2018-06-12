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
    pub use directed::astar::*;
    pub use directed::bfs::*;
    pub use directed::dfs::*;
    pub use directed::dijkstra::*;
    pub use directed::edmonds_karp::*;
    pub use directed::fringe::*;
    pub use directed::idastar::*;
    pub use directed::topological_sort::*;
    pub use grid::*;
    pub use kuhn_munkres::*;
    pub use matrix::*;
    pub use undirected::connected_components::*;
    pub use utils::*;
}
