
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
