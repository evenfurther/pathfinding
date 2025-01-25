# pathfinding

[![Current Version](https://img.shields.io/crates/v/pathfinding.svg)](https://crates.io/crates/pathfinding)
[![Documentation](https://docs.rs/pathfinding/badge.svg)](https://docs.rs/pathfinding)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/pathfinding.svg)](#license)

This crate implements several pathfinding, flow, and graph algorithms in [Rust](https://rust-lang.org/). The algorithms are generic over their arguments. See [the documentation](https://docs.rs/pathfinding) for more information about the various algorithms.

## Using this crate

In your `Cargo.toml`, put:

``` ini
[dependencies]
pathfinding = "4.14.0"
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

## License

This code is released under a dual Apache 2.0 / MIT free software license.

## Contributing

You are welcome to contribute by opening [issues](https://github.com/evenfurther/pathfinding/issues)
or submitting [pull requests](https://github.com/evenfurther/pathfinding/pulls). Please open an issue
before implementing a new feature, in case it is a work in progress already or it is fit for this
repository.

In order to pass the continuous integration tests, your code must be formatted using the latest
`rustfmt` with the nightly rust toolchain, and pass `cargo clippy` and [`pre-commit`](https://pre-commit.com/) checks.
Those will run automatically when you submit a pull request. You can install `pre-commit` to your
checked out version of the repository by running:

```bash
$ pre-commit install --hook-type commit-msg
```

This repository uses the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) commit message style, such as:

- feat(matrix): add `Matrix::transpose()`
- fix(tests): remove unused imports

Each commit must be self-sufficient and clean. If during inspection or code review you need to make further changes to a commit, please squash it. You may use `git rebase -i`, or more convenient tools such as [`jj`](https://martinvonz.github.io/jj/latest/) or [`git-branchless`](https://github.com/arxanas/git-branchless), in order to manipulate your git commits.

If a pull-request should automatically close an open issue, please
include "Fix #xxx# or "Close #xxx" in the pull-request cover-letter.
