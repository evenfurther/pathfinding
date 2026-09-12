//! Instruction-count benchmarks for the search algorithms.
//!
//! Each search runs inside an `#[inline(never)]` helper whose result is returned from the
//! benchmark. Both halves matter. Callgrind only counts instructions while collection is
//! toggled on around the benchmark function, so a search that gets inlined into the harness
//! and folded away is reported as a few hundred instructions rather than a few million; and
//! because that depends on inlining, it changes with unrelated edits and turns into a
//! spurious regression in the CI comparison. Keeping the work behind a call the optimiser
//! cannot see through, and consuming the result, keeps the measurement honest.

use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use pathfinding::prelude::{astar, bfs, bfs_bidirectional, dfs, dijkstra, fringe, idastar, iddfs};
use std::hint::black_box;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pt {
    x: u16,
    y: u16,
}

impl Pt {
    const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    #[inline]
    const fn heuristic(p: &Self) -> usize {
        (128 - p.x - p.y) as usize
    }
}

#[inline]
fn successors(pt: &Pt) -> Vec<Pt> {
    let mut ret = Vec::with_capacity(4);
    if 0 < pt.x {
        ret.push(Pt::new(pt.x - 1, pt.y));
    }
    if pt.x < 64 {
        ret.push(Pt::new(pt.x + 1, pt.y));
    }
    if 0 < pt.y {
        ret.push(Pt::new(pt.x, pt.y - 1));
    }
    if pt.y < 64 {
        ret.push(Pt::new(pt.x, pt.y + 1));
    }
    ret
}

#[inline]
fn weighted(pt: &Pt) -> impl Iterator<Item = (Pt, usize)> + use<> {
    successors(pt).into_iter().map(|n| (n, 1))
}

const fn at_corner(n: &Pt) -> bool {
    n.x == 64 && n.y == 64
}

const fn never(_: &Pt) -> bool {
    false
}

const fn at_five(n: &Pt) -> bool {
    n.x == 5 && n.y == 5
}

const fn one(_: &Pt) -> usize {
    1
}

type Path = Option<(Vec<Pt>, usize)>;

#[inline(never)]
fn run_astar(start: &Pt, success: fn(&Pt) -> bool, heuristic: fn(&Pt) -> usize) -> Path {
    astar(start, weighted, heuristic, success)
}

#[inline(never)]
fn run_dijkstra(start: &Pt, success: fn(&Pt) -> bool) -> Path {
    dijkstra(start, weighted, success)
}

#[inline(never)]
fn run_fringe(start: &Pt, success: fn(&Pt) -> bool, heuristic: fn(&Pt) -> usize) -> Path {
    fringe(start, weighted, heuristic, success)
}

#[inline(never)]
fn run_idastar(start: &Pt, success: fn(&Pt) -> bool) -> Path {
    idastar(start, weighted, Pt::heuristic, success)
}

#[inline(never)]
fn run_bfs(start: &Pt, success: fn(&Pt) -> bool) -> Option<Vec<Pt>> {
    bfs(start, successors, success)
}

#[inline(never)]
fn run_dfs(start: Pt, success: fn(&Pt) -> bool) -> Option<Vec<Pt>> {
    dfs(start, successors, success)
}

#[inline(never)]
fn run_iddfs(start: Pt, success: fn(&Pt) -> bool) -> Option<Vec<Pt>> {
    iddfs(start, successors, success)
}

#[inline(never)]
fn run_bfs_bidirectional(start: &Pt, end: &Pt) -> Option<Vec<Pt>> {
    bfs_bidirectional(start, end, successors, successors)
}

/// The no-path case searches backwards from an unreachable node that has no successors.
#[inline(never)]
fn run_bfs_bidirectional_no_path(start: &Pt, end: &Pt) -> Option<Vec<Pt>> {
    bfs_bidirectional(start, end, successors, |_| vec![])
}

#[library_benchmark]
fn corner_to_corner_astar() -> Path {
    let path = run_astar(&black_box(Pt::new(0, 0)), at_corner, Pt::heuristic);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_bfs() -> Option<Vec<Pt>> {
    let path = run_bfs(&black_box(Pt::new(0, 0)), at_corner);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_bfs_bidirectional() -> Option<Vec<Pt>> {
    let path = run_bfs_bidirectional(&black_box(Pt::new(0, 0)), &black_box(Pt::new(64, 64)));
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_dfs() -> Option<Vec<Pt>> {
    let path = run_dfs(black_box(Pt::new(0, 0)), at_corner);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_dijkstra() -> Path {
    let path = run_dijkstra(&black_box(Pt::new(0, 0)), at_corner);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_fringe() -> Path {
    let path = run_fringe(&black_box(Pt::new(0, 0)), at_corner, Pt::heuristic);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_idastar() -> Path {
    let path = run_idastar(&black_box(Pt::new(0, 0)), at_corner);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn corner_to_corner_iddfs() -> Option<Vec<Pt>> {
    let path = run_iddfs(black_box(Pt::new(0, 0)), at_five);
    assert!(path.is_some());
    path
}

#[library_benchmark]
fn no_path_astar() -> Path {
    let path = run_astar(&black_box(Pt::new(2, 3)), never, one);
    assert!(path.is_none());
    path
}

#[library_benchmark]
fn no_path_bfs() -> Option<Vec<Pt>> {
    let path = run_bfs(&black_box(Pt::new(2, 3)), never);
    assert!(path.is_none());
    path
}

#[library_benchmark]
fn no_path_bfs_bidirectional() -> Option<Vec<Pt>> {
    let path = run_bfs_bidirectional_no_path(
        &black_box(Pt::new(2, 3)),
        &black_box(Pt::new(u16::MAX, u16::MAX)),
    );
    assert!(path.is_none());
    path
}

#[library_benchmark]
fn no_path_dfs() -> Option<Vec<Pt>> {
    let path = run_dfs(black_box(Pt::new(2, 3)), never);
    assert!(path.is_none());
    path
}

#[library_benchmark]
fn no_path_dijkstra() -> Path {
    let path = run_dijkstra(&black_box(Pt::new(2, 3)), never);
    assert!(path.is_none());
    path
}

#[library_benchmark]
fn no_path_fringe() -> Path {
    let path = run_fringe(&black_box(Pt::new(2, 3)), never, one);
    assert!(path.is_none());
    path
}

library_benchmark_group!(
    name = corner_to_corner;
    benchmarks =
        corner_to_corner_astar,
        corner_to_corner_bfs,
        corner_to_corner_bfs_bidirectional,
        corner_to_corner_dfs,
        corner_to_corner_dijkstra,
        corner_to_corner_fringe,
        corner_to_corner_idastar,
        corner_to_corner_iddfs
);

library_benchmark_group!(
    name = no_path;
    benchmarks =
        no_path_astar,
        no_path_bfs,
        no_path_bfs_bidirectional,
        no_path_dfs,
        no_path_dijkstra,
        no_path_fringe
);

main!(library_benchmark_groups = corner_to_corner, no_path);
