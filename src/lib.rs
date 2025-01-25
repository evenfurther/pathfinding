#![forbid(missing_docs)]
//! # pathfinding
//!
//! [![Current Version](https://img.shields.io/crates/v/pathfinding.svg)](https://crates.io/crates/pathfinding)
//! [![Documentation](https://docs.rs/pathfinding/badge.svg)](https://docs.rs/pathfinding)
//! [![License: Apache-2.0/MIT](https://img.shields.io/crates/l/pathfinding.svg)](#license)
//!
//! This crate implements several pathfinding, flow, and graph algorithms in [Rust][Rust].
//!
//! ## Algorithms
//!
//! The algorithms are generic over their arguments.
//!
//! ### Directed graphs
//!
//! - [A*](directed/astar/index.html): find the shortest path in a weighted graph using an heuristic to guide the process ([⇒ Wikipedia][A*])
//! - [BFS](directed/bfs/index.html): explore nearest successors first, then widen the search ([⇒ Wikipedia][BFS])
//! - [Bidirectional search](directed/bfs/fn.bfs_bidirectional.html): simultaneously explore paths forwards from the start and backwards from the goal ([=> Wikipedia][Bidirectional search])
//! - [Brent](directed/cycle_detection/index.html): find a cycle in an infinite sequence ([⇒ Wikipedia][Brent])
//! - [DFS](directed/dfs/index.html): explore a graph by going as far as possible, then backtrack ([⇒ Wikipedia][DFS])
//! - [Dijkstra](directed/dijkstra/index.html): find the shortest path in a weighted graph ([⇒ Wikipedia][Dijkstra])
//! - [Edmonds Karp](directed/edmonds_karp/index.html): find the maximum flow in a weighted graph ([⇒ Wikipedia][Edmonds Karp])
//! - [Floyd](directed/cycle_detection/index.html): find a cycle in an infinite sequence ([⇒ Wikipedia][Floyd])
//! - [Fringe](directed/fringe/index.html): find the shortest path in a weighted graph using an heuristic to guide the process ([⇒ Wikipedia][Fringe])
//! - [IDA*](directed/idastar/index.html): explore longer and longer paths in a weighted graph at the cost of multiple similar examinations ([⇒ Wikipedia][IDA*])
//! - [IDDFS](directed/iddfs/index.html): explore longer and longer paths in an unweighted graph at the cost of multiple similar examinations ([⇒ Wikipedia][IDDFS])
//! - [paths counting](directed/count_paths/index.html): count the paths to the destination in an acyclic graph
//! - [strongly connected components](directed/strongly_connected_components/index.html): find strongly connected components in a directed graph ([⇒ Wikipedia][Strongly connected components])
//! - [topological sorting](directed/topological_sort/index.html): find an acceptable topological order in a directed graph ([⇒ Wikipedia][Topological sorting])
//! - [Yen](directed/yen/index.html): find k-shortest paths using Dijkstra ([⇒ Wikipedia][Yen])
//!
//! ### Undirected graphs
//!
//! - [connected components](undirected/connected_components/index.html): find disjoint connected sets of vertices ([⇒ Wikipedia][Connected components])
//! - [Kruskal](undirected/kruskal/index.html): find a minimum-spanning-tree ([⇒ Wikipedia][Kruskal])
//! - [Prim](undirected/prim/index.html): find a minimum-spanning-tree ([⇒ Wikipedia][Prim])
//! - [cliques]: find maximum cliques in a graph ([= Wikipedia][BronKerbosch])
//!
//! ### Matching
//!
//! - [Kuhn-Munkres](kuhn_munkres/index.html) (Hungarian algorithm): find the maximum (or minimum) matching in a weighted bipartite graph ([⇒ Wikipedia][Kuhn-Munkres])
//!
//! ### Miscellaneous structures
//!
//! - A [`Grid`](grid/index.html) type representing a rectangular grid in which vertices can be added or removed, with automatic creation of edges between adjacent vertices.
//! - A [`Matrix`](matrix/index.html) type to store data of arbitrary types, with neighbour-aware methods.
//!
//! ## Example
//!
//! We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
//! moves.
//!
//! ``` rust
//! use pathfinding::prelude::bfs;
//!
//! #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
//! struct Pos(i32, i32);
//!
//! impl Pos {
//!   fn successors(&self) -> Vec<Pos> {
//!     let &Pos(x, y) = self;
//!     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
//!          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
//!   }
//! }
//!
//! static GOAL: Pos = Pos(4, 6);
//! let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
//! assert_eq!(result.expect("no path found").len(), 5);
//! ```
//!
//! ## Note on floating-point types
//!
//! Several algorithms require that the numerical types used to describe
//! edge weights implement `Ord`. If you wish to use Rust built-in
//! floating-point types (such as `f32`) that implement `PartialOrd`
//! in this context, you can wrap them into compliant types using the
//! [ordered-float](https://crates.io/crates/ordered-float) crate.
//!
//! The minimum supported Rust version (MSRV) is Rust 1.77.2.
//!
//! [A*]: https://en.wikipedia.org/wiki/A*_search_algorithm
//! [BFS]: https://en.wikipedia.org/wiki/Breadth-first_search
//! [Bidirectional search]: https://en.wikipedia.org/wiki/Bidirectional_search
//! [Brent]: https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
//! [BronKerbosch]: https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
//! [Connected components]: https://en.wikipedia.org/wiki/Connected_component_(graph_theory)
//! [DFS]: https://en.wikipedia.org/wiki/Depth-first_search
//! [Dijkstra]: https://en.wikipedia.org/wiki/Dijkstra's_algorithm
//! [Edmonds Karp]: https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm
//! [Floyd]: https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare
//! [Fringe]: https://en.wikipedia.org/wiki/Fringe_search
//! [IDA*]: https://en.wikipedia.org/wiki/Iterative_deepening_A*
//! [IDDFS]: https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search
//! [Kruskal]: https://en.wikipedia.org/wiki/Kruskal's_algorithm
//! [Kuhn-Munkres]: https://en.wikipedia.org/wiki/Hungarian_algorithm
//! [Prim]: https://en.wikipedia.org/wiki/Prim's_algorithm
//! [Rust]: https://rust-lang.org/
//! [Strongly connected components]: https://en.wikipedia.org/wiki/Strongly_connected_component
//! [Topological sorting]: https://en.wikipedia.org/wiki/Topological_sorting
//! [Yen]: https://en.wikipedia.org/wiki/Yen's_algorithm

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
    pub use crate::undirected::cliques::*;
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
