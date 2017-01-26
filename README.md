# pathfinding

[![Unix build Status](https://travis-ci.org/samueltardieu/pathfinding.svg?branch=master)](https://travis-ci.org/samueltardieu/pathfinding)
[![Windows build Status](https://ci.appveyor.com/api/projects/status/github/samueltardieu/pathfinding?branch=master&svg=true)](https://ci.appveyor.com/project/samueltardieu/pathfinding)
[![Current Version](https://img.shields.io/crates/v/pathfinding.svg)](https://crates.io/crates/pathfinding)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/pathfinding.svg)](#license)

This crate implements several pathfinding algorithms in [Rust]():

- [A*](): find the shortest path in a weighted graph using an heuristic to guide the process.
- [breadth-first search (BFS)](): explore nearest neighbours first, then widen the search.
- [depth-first search (DFS)](): explore a graph by going as far as possible, then backtrack.
- [Dijkstra](): find the shortest path in a weighted graph.
- [Fringe](): find the shortest path in a weighted graph using an heuristic to guide the process.

Those algorithms are generic over their arguments.

## Using this crate

In your `Cargo.toml`, put:

``` ini
[dependencies]
pathfinding = "0.1"
```

You can then pull your preferred algorithm (BFS in this example) using:

``` rust
extern crate pathfinding;

use pathfinding::bfs;;
```

## Example


We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
moves.

The first version uses an explicit type `Pos` on which the required traits are derived.

``` rust
use pathfinding::bfs;

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

The second version does not declare a `Pos` type, makes use of more closures,
and is thus shorter.

``` rust
use pathfinding::bfs;

static GOAL: (i32, i32) = (4, 6);
let result = bfs(&(1, 1),
                 |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
                                (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)],
                 |&p| p == GOAL);
assert_eq!(result.expect("no path found").len(), 5);
```

## License

This code is released under a dual Apache 2.0 / MIT free software license.

[A*]: https://en.wikipedia.org/wiki/A*_search_algorithm
[BFS]: https://en.wikipedia.org/wiki/Breadth-first_search
[DFS]: https://en.wikipedia.org/wiki/Depth-first_search
[Dijkstra]: https://en.wikipedia.org/wiki/Dijkstra's_algorithm
[Fringe]: https://en.wikipedia.org/wiki/Fringe_search
[Rust]: https://rust-lang.org/
