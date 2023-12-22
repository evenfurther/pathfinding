# pathfinding

[![Current Version](https://img.shields.io/crates/v/pathfinding.svg)](https://crates.io/crates/pathfinding)
[![Documentation](https://docs.rs/pathfinding/badge.svg)](https://docs.rs/pathfinding)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/pathfinding.svg)](#license)

This crate implements several pathfinding, flow, and graph algorithms in [Rust][Rust].

## Algorithms

The algorithms are generic over their arguments.

### Directed graphs

- [A*][A*]: find the shortest path in a weighted graph using an heuristic to guide the process.
- [BFS][BFS]: explore nearest successors first, then widen the search.
- [Brent][Brent]: find a cycle in an infinite sequence.
- [DFS][DFS]: explore a graph by going as far as possible, then backtrack.
- [Dijkstra][Dijkstra]: find the shortest path in a weighted graph.
- [Edmonds Karp][Edmonds Karp]: find the maximum flow in a weighted graph.
- [Floyd][Floyd]: find a cycle in an infinite sequence.
- [Fringe][Fringe]: find the shortest path in a weighted graph using an heuristic to guide the process.
- [IDA*][IDA*]: explore longer and longer paths in a weighted graph at the cost of multiple similar examinations.
- [IDDFS][IDDFS]: explore longer and longer paths in an unweighted graph at the cost of multiple similar examinations.
- [strongly connected components][Strongly connected components]: find strongly connected components in a directed graph.
- [topological sorting][Topological sorting]: find an acceptable topological order in a directed graph.
- [Yen][Yen]: find k-shortest paths using Dijkstra.

### Undirected graphs

- [connected components][Connected components]: find disjoint connected sets of vertices.
- [Kruskal][Kruskal]: find a minimum-spanning-tree.

### Matching

- [Kuhn-Munkres][Kuhn-Munkres] (Hungarian algorithm): find the maximum (or minimum) matching
in a weighted bipartite graph.

### Miscellaneous structures

- A `Grid` type representing a rectangular grid in which vertices can be added or removed,
  with automatic creation of edges between adjacent vertices.
- A `Matrix` type to store data of arbitrary types, with neighbour-aware methods.

## Using this crate

In your `Cargo.toml`, put:

``` ini
[dependencies]
pathfinding = "4.7.0"
```

You can then pull your preferred algorithm (BFS in this example) using:

``` rust
use pathfinding::prelude::bfs;
```

## Example

We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
moves.

``` rust
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn successors(&self) -> Vec<Pos> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
  }
}

static GOAL: Pos = Pos(4, 6);
let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
assert_eq!(result.expect("no path found").len(), 5);
```

## Note on floating-point types

Several algorithms require that the numerical types used to describe
edge weights implement `Ord`. If you wish to use Rust built-in
floating-point types (such as `f32`) that implement `PartialOrd`
in this context, you can wrap them into compliant types using the
[ordered-float](https://crates.io/crates/ordered-float) crate.

The minimum supported Rust version (MSRV) is Rust 1.70.0.

## License

This code is released under a dual Apache 2.0 / MIT free software license.

## Contributing

You are welcome to contribute by opening [issues](https://github.com/evenfurther/pathfinding/issues)
or submitting [pull requests](https://github.com/evenfurther/pathfinding/pulls). Please open an issue
before implementing a new feature, in case it is a work in progress already or it is fit for this
repository.

In order to pass the continuous integration tests, your code must be formatted using the latest
`rustfmt` with the nightly rust toolchain, and pass `cargo clippy` and [`pre-commit`](https://pre-commit.com/) checks. Those will run automatically when you submit a pull request.

This repository uses the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) commit message style, such as:

- feat(matrix): add `Matrix::transpose()`
- fix(tests): remove unused imports

If a pull-request should automatically close an open issue, please
include "Fix #xxx# or "Close #xxx" in the pull-request cover-letter.

[A*]: https://en.wikipedia.org/wiki/A*_search_algorithm
[BFS]: https://en.wikipedia.org/wiki/Breadth-first_search
[Brent]: https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
[Connected components]: https://en.wikipedia.org/wiki/Connected_component_(graph_theory)
[DFS]: https://en.wikipedia.org/wiki/Depth-first_search
[Dijkstra]: https://en.wikipedia.org/wiki/Dijkstra's_algorithm
[Edmonds Karp]: https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm
[Floyd]: https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare
[Fringe]: https://en.wikipedia.org/wiki/Fringe_search
[Kruskal]: https://en.wikipedia.org/wiki/Kruskal's_algorithm
[IDA*]: https://en.wikipedia.org/wiki/Iterative_deepening_A*
[IDDFS]: https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search
[Kuhn-Munkres]: https://en.wikipedia.org/wiki/Hungarian_algorithm
[Rust]: https://rust-lang.org/
[Strongly connected components]: https://en.wikipedia.org/wiki/Strongly_connected_component
[Topological sorting]: https://en.wikipedia.org/wiki/Topological_sorting
[Yen]: https://en.wikipedia.org/wiki/Yen's_algorithm
