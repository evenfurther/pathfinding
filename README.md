# pathfinding

[![Unix build Status](https://travis-ci.org/samueltardieu/pathfinding.svg?branch=master)](https://travis-ci.org/samueltardieu/pathfinding)
[![Windows build Status](https://ci.appveyor.com/api/projects/status/github/samueltardieu/pathfinding?branch=master&svg=true)](https://ci.appveyor.com/project/samueltardieu/pathfinding)
[![Current Version](https://img.shields.io/crates/v/pathfinding.svg)](https://crates.io/crates/pathfinding)
[![Documentation](https://docs.rs/pathfinding/badge.svg)](https://docs.rs/pathfinding)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/pathfinding.svg)](#license)

This crate implements several pathfinding, flow, and graph algorithms in [Rust][Rust].

## Algorithms

The algorithms are generic over their arguments.

### Directed graphs

- [A*][A*]: find the shortest path in a weighted graph using an heuristic to guide the process.
- [BFS][BFS]: explore nearest neighbours first, then widen the search.
- [DFS][DFS]: explore a graph by going as far as possible, then backtrack.
- [Dijkstra][Dijkstra]: find the shortest path in a weighted graph.
- [Edmonds Karp][Edmonds Karp]: find the maximum flow in a weighted graph.
- [Fringe][Fringe]: find the shortest path in a weighted graph using an heuristic to guide the process.
- [IDA*][IDA*]: explore longer and longer paths in a weighted graph at the cost of multiple similar examinations.
- [IDDFS][IDDFS]: explore longer and longer paths in an unweighted graph at the cost of multiple similar examinations.
- [strongly connected components][Strongly connected components]: find strongly connected components in a directed graph.
- [topological sorting][Topological sorting]: find an acceptable topological order in a directed graph.

### Undirected graphs

- [connected components][Connected components]: find disjoint connected sets of vertices.

### Matching

- [Kuhn-Munkres][Kuhn-Munkres] (Hungarian algorithm): find the maximum (or minimum) matching
in a weighted bipartite graph.

## Using this crate

In your `Cargo.toml`, put:

``` ini
[dependencies]
pathfinding = "0.8"
```

You can then pull your preferred algorithm (BFS in this example) using:

``` rust
extern crate pathfinding;

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
  fn neighbours(&self) -> Vec<Pos> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
  }
}

static GOAL: Pos = Pos(4, 6);
let result = bfs(&Pos(1, 1), |p| p.neighbours(), |p| *p == GOAL);
assert_eq!(result.expect("no path found").len(), 5);
```

## License

This code is released under a dual Apache 2.0 / MIT free software license.

## Contributing

You are welcome to contribute by opening [issues](https://github.com/samueltardieu/pathfinding/issues)
or submitting [pull requests](https://github.com/samueltardieu/pathfinding/pulls). Please open an issue
before implementing a new feature, in case it is a work in progress already or it is fit for this
repository.

In order to pass the continuous integration tests, your code must be formatted using the latest
`rustfmt` with the nightly rust toolchain (available as the `rustfmt-preview` component of `rustup`).

This repository use the imperative mode in commit messages, such as "Add IDDFS",
"Fix #xxx". This style is preferred over "Added IDDFS" or "Fixed #xxx".

[A*]: https://en.wikipedia.org/wiki/A*_search_algorithm
[BFS]: https://en.wikipedia.org/wiki/Breadth-first_search
[Connected components]: https://en.wikipedia.org/wiki/Connected_component_(graph_theory)
[DFS]: https://en.wikipedia.org/wiki/Depth-first_search
[Dijkstra]: https://en.wikipedia.org/wiki/Dijkstra's_algorithm
[Edmonds Karp]: https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm
[Fringe]: https://en.wikipedia.org/wiki/Fringe_search
[IDA*]: https://en.wikipedia.org/wiki/Iterative_deepening_A*
[IDDFS]: https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search
[Kuhn-Munkres]: https://en.wikipedia.org/wiki/Hungarian_algorithm
[Rust]: https://rust-lang.org/
[Strongly connected components]: https://en.wikipedia.org/wiki/Strongly_connected_component
[Topological sorting]: https://en.wikipedia.org/wiki/Topological_sorting
