#![forbid(missing_docs)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

pub use num_traits;

pub mod cycle_detection;
pub mod directed;
pub mod grid;
pub mod kuhn_munkres;
pub mod matrix;
pub mod undirected;
pub mod utils;

/// Export all public functions and structures for an easy access.
pub mod prelude {
    pub use crate::cycle_detection::*;
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
    pub use crate::directed::yen::*;
    pub use crate::grid::*;
    pub use crate::kuhn_munkres::*;
    pub use crate::matrix::*;
    pub use crate::undirected::connected_components::*;
    pub use crate::undirected::kruskal::*;
    pub use crate::utils::*;
}
