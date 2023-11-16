#![doc = include_str!("../README.md")]

use deprecate_until::deprecate_until;
pub use num_traits;

pub mod directed;
pub mod grid;
pub mod kuhn_munkres;
pub mod matrix;
pub mod undirected;
pub mod utils;

use indexmap::{IndexMap, IndexSet};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;
type FxIndexSet<K> = IndexSet<K, BuildHasherDefault<FxHasher>>;

/// Export all public functions and structures for an easy access.
pub mod prelude {
    pub use crate::directed::astar::*;
    pub use crate::directed::bfs::*;
    pub use crate::directed::count_paths::*;
    pub use crate::directed::cycle_detection::*;
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

/// Deprecated: moved into the `directed` module.
#[deprecate_until(
    note = "use directed::cycle_detection or the prelude instead",
    since = "4.3.1",
    remove = "> 4.x"
)]
pub mod cycle_detection {
    pub use crate::directed::cycle_detection::*;
}
