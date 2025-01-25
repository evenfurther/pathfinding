
v4.14.0 / 2025-01-25
==================

  * feat: implement bidirectional BFS
  * fix(dijkstra)!: remove unneeded partial cost parameter. This is
    a BREAKING CHANGE in `dijkstra_reach()` parameters, but fortunately
    very easy to fix for users.
  * chore: reorganize `rand` imports

v4.13.1 / 2025-01-15
==================

  * fix(perf): back out commit 808b951c5a9eb5dd25adbd46a5887525d0a0913d which causes a severe performance regression in Dijkstra algorithm
  * style: use `usize::div_ceil()`
  * feat(gitignore): ignore flame graph files
  * docs: cleanup some first line
  * chore(deps): update rust crate itertools to 0.14.0
  * fix(clippy): put test module last in file
  * feat: accept `&(usize, usize)` as `Matrix` index
  * fix(style): use a less convoluted test

v4.13.0 / 2024-12-29
==================

  * feat: implement Bron-Kerbosch algorithm, thanks to @gzsombor
  * fix(style): use `unwrap_or_else` when appropriate
  * fix(style): use `Self` instead of the type name
  * fix(style): make some functions `const`

v4.12.0 / 2024-12-10
==================

  * fix(doc): reference `count_paths` from top-level documentation
  * fix(tests): remove `test_` prefix in tests
  * chore(Cargo): update fake regex dependency message
  * fix(deps): update rust crate thiserror to v2
  * fix: use proper pattern binding
  * fix(deps): update codspeed-criterion-compat to get rid of advisory
  * fix(tests): new test for `utils` module
  * fix(ui): adapt UI tests to Rust 1.84
  * chore(gitignore): ignore `cargo mutants` output
  * fix(tests): `gen` will be a keyword in Rust 2024
  * fix(kruskal): accept owned data into the method
  * fix: remove or move `allow` attributes
  * style(matrix): remove unneeded bounds on `DoubleEndedIterator` impl
  * style: replace `let _ =` by `_ =`
  * docs: remove empty lines in comments
  * Add test for Yen's algorithm
  * Add precision on development process
  * feat(tests): test Edmonds-Karp failure in sparse mode
  * feat(dfs): use a non-recursive version
  * fix(benches): reinstate regular benches for DFS
  * fix!(dfs): never visit the same node twice
  * feat(benches): add restricted DFS benchmarks
  * style: use Iterator::inspect() when the value does not change
  * chore(deps): update rust crate codspeed-criterion-compat to v2
  * fix!(msrv): update MSRV to 1.77.2
  * Generic variant of connected_components
  * fix(doc): refer to `usize::MAX` instead of `std::usize::MAX`
  * chore(Cargo.toml): allow `clippy::too_long_first_doc_paragraph`

v4.11.0 / 2024-08-31
==================

  * feat(prim): add Prim's algorithm for finding MST
  * docs(astar): add documentation for SmallestCostHolder
  * fix(README): Broken link in the README.md
  * test: add more tests for `Grid` and `Matrix`
  * fix(cargo-deny): update configuration

v4.10.0 / 2024-06-18
==================

  * feat: replace `FixedBitSet` by `IndexSet` for better performances
  * chore(deps): update many dependencies for better performances
  * feat(tests): add new aoc-2023-17 test
  * fix(tests): do not build useless vector
  * fix: remove redundant imports

v4.9.1 / 2024-02-12
==================

  * fix(README): inline documentation to fix inner links to modules
  * fix(deps): update rust crate indexmap to 2.2.3
  * fix(deps): update rust crate thiserror to 1.0.57

v4.9.0 / 2024-02-11
==================

  * feat(matrix): add in-place matrix transposition for non-square matrix
  * feat(bench): add bench for matrix transposition
  * feat(tests): add a test for transposing an empty matrix
  * fix(deps): add priority to clippy lints for lint_groups_priority
  * chore(grid): replace deprecated IndexMap remove() method by swap_remove()
  * fix(deps): update rust crate num-traits to 0.2.18
  * fix(deps): update rust crate indexmap to 2.2.2
  * chore(deps): update rust crate itertools to 0.12.1

v4.8.2 / 2024-01-14
==================

  * fix(dfs_reach): visit nodes in the documented order

v4.8.1 / 2024-01-07
==================

  * fix(yen): revert "Routes are already sorted by cost and path len"
  * test(yen): add test for checking Yen algorithm output ordering
  * chore(pre-commit): add conventional commit check
  * chore: use deprecate_until attribute instead of deprecated

v4.8.0 / 2023-12-22
==================

  * feat(matrix): add `Matrix::transpose()`
  * feat(matrix): add `Matrix::column_iter()`

v4.7.0 / 2023-12-21
==================

  * feat(grid): add `Grid::constrain()`
  * feat(matrix): add `Matrix::constrain()`
  * feat(utils): add `constrain()`

v4.6.0 / 2023-12-14
==================

  * feat(matrix): implement DoubleEndedIterator for RowIterator

v4.5.0 / 2023-12-14
==================

  * feat(matrix): add swap method
  * chore(msrv): update minimum required Rust version to 1.70.0
  * chore: use bool::is_some_and

v4.4.0 / 2023-11-30
==================

  * feat: new `dijkstra_reach()` function
  * fix(doc): remove useless explicit links

v4.3.4 / 2023-11-29
==================

  * fix(edmondskarp): better panic messages
  * fix(matrix): better panic messages
  * fix(style): apply clippy fixes
  * fix(doc): typo

v4.3.3 / 2023-11-13
==================

  * fix(yen): return all loopless paths
  * chore(cargo deny): fix warning in configuration file
  * chore(deps): update rust crate indexmap to 2.1.0
  * chore(deps): update rust crate thiserror to 1.0.50
  * chore(deps): update rust crate regex to 1.10.2
  * chore(deps): update rust crate num-traits to 0.2.17

v4.3.2 / 2023-09-22
==================

  * New remaining_low_bounds() method for {Bfs,Dfs}Reachable
  * Migrate to the evenfurther GitHub organization
  * fix(deps): update rust crate thiserror to 1.0.48
  * Use or_default() in test

v4.3.1 / 2023-08-02
==================

  * Move `cycle_detection` module into `directed` and deprecate the former
  * Update indexmap requirement from 1.9.2 to 2.0.0
  * Style: use `or_default()` rather than `or_insert_with()` with default value
  * Style: do not use `bool::then()` in `filter_map()`
  * Style: make `partial_cmp` use `cmp`
  * Style: reformat with let/else support
  * Use codspeed-criterion-compat everywhere, do not require criterion

v4.3.0 / 2023-05-30
==================

  * Allow creating a Matrix based on a function from position to value
  * Make method cancel_flow of edmondskarp only cancel the minimum amount of flow among all edges along a path, instead of the maximum, in order to avoid negative flows
  * Use sort_unstable_by() instead of sort_unstable_by_key()
  * New Grid example for from_coordinates() method
  * Use RemSP and path splitting
  * Remove optimization which gives worst benchmark results
  * Integrate CodSpeed
  * Update criterion requirement from 0.4.0 to 0.5.1
  * Make Kuhn-Munkres benchmarks reproducible

v4.2.1 / 2023-01-17
==================

  * Document that A*/Dijkstra/Fringe/idA* costs must be non-negative
  * Upgrade dependencies
  * Use new clippy lint name
  * Add bench for separate_components
  * Bench Kuhn-Munkres algorithm
  * Remove itertools dependency
  * Remove unnecessary .into_iter() in tests

v4.2.0 / 2022-12-25
==================

  * Add Grid::from_coordinates()
  * Add the possibility to display the grid with reversed line order
  * Add more Grid documentation

v4.1.1 / 2022-12-14
==================

  * Better performances in Grid, Kruskal and Edmonds-Karp

v4.1.0 / 2022-12-14
==================

  * Add Matrix::items() and Matrix::items_mut()
  * Rename Matrix::indices() as Matrix::keys() and deprecate Matrix::indices()
  * Clarify the ordering of coordinate tuples in Matrix
  * Add more Grid documentation
  * Enable clippy pedantic mode by default

v4.0.1 / 2022-12-12
==================

  * Improve bfs performance
  * Add documentation for possible errors and panics

v4.0.0 / 2022-11-30
==================

  * Add move_in_direction and in_direction to utils
  * Make some function const
  * Cleanups
  * Count paths
  * Add minimum_cut capability to EdmondsKarp
  * Bump MSRV to 1.65.0
  * Update dependencies

v3.0.14 / 2022-10-03
==================

  * Use into_keys() where appropriate
  * Add fake regex dev dependency
  * Use boolean::then_some()
  * Update criterion requirement from 0.3.4 to 0.4.0
  * Optimize Yen's algorithm
  * Routes are already sorted by cost and path len

v3.0.13 / 2022-06-16
==================

  * Document possibility of looping endlessly in kuhn_munkres related functions
  * Use matches!() to simplify expression

v3.0.12 / 2022-04-13
==================

  * Add two algorithms (Floyd and Brent) to detect cycles
  * Deprecate absdiff() in favor of Rust 1.60 abs_diff()
  * Remove double must-use

v3.0.11 / 2022-03-11
==================

  * Introduce `Grid::{bfs,dfs}_reachable()` and `deprecate Grid::reachable()`
  * Remove `Copy` bound on predicate of `Matrix::{bfs,dfs}_reachable()`
  * Use anonymous lifetimes when appropriate
  * Add example for `kuhn_munkres()`

v3.0.10 / 2022-02-14
====================

  * Remove unused `Matrix::uninit`/`Matrix::assume_init()`
  * Remove remaining `debug_assert!()` calls

v3.0.9 / 2022-02-02
===================

  * Add conversion from `Matrix<bool>` to `Grid`
  * Add `Grid` equality
  * Add `Matrix::map()`

v3.0.8 / 2022-01-24
===================

  * Add `Matrix::new_uninit()` and `Matrix::assume_init()`
  * Forbid all missing or partially missing docs
  * Mark iterators as fused

v3.0.7 / 2022-01-23
===================

  * Deprecate `Matrix::reachable()` for `Matrix::bfs_reachable(`) and
    `Matrix::dfs_reachable()`
  * Add `dfs_reach()`
  * Use an enumeration to represent `MatrixFormatError`

v3.0.6 / 2022-01-12
===================

  * Add MSRV and check for consistency
  * Add `#[must_use]` on `Weights` trait
  * Use thiserror crate to build `MatrixFormatError`
  * Add an example for `Grid` as `Debug`

v3.0.5 / 2021-12-13
===================

  * Alternate `Grid` debug mode

v3.0.4 / 2021-12-12
===================

  * Add `Grid::reachable()`
  * Add `Matrix::get()` and `Matrix::get_mut()`

v3.0.3 / 2021-12-09
===================

  * Add `Matrix::reachable()`
  * Better `Matrix` corner cases documentation

v3.0.2 / 2021-12-09
===================

  * Remove references in `Grid` methods
  * Remove more references in `Matrix` methods

v3.0.1 / 2021-12-09
===================

  * Remove unnecessary `Clone` bounds

v3.0.0 / 2021-12-09
===================

  * Use tuples instead of tuples reference for `Matrix` index
