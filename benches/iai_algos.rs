use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use pathfinding::prelude::{astar, bfs, bfs_bidirectional, dfs, dijkstra, fringe, idastar, iddfs};

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

#[library_benchmark]
fn corner_to_corner_astar() {
    assert_ne!(
        astar(
            &Pt::new(0, 0),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            Pt::heuristic,
            |n| n.x == 64 && n.y == 64,
        ),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_bfs() {
    assert_ne!(
        bfs(&Pt::new(0, 0), successors, |n| n.x == 64 && n.y == 64),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_bfs_bidirectional() {
    assert_ne!(
        bfs_bidirectional(&Pt::new(0, 0), &Pt::new(64, 64), successors, successors),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_dfs() {
    assert_ne!(
        dfs(Pt::new(0, 0), successors, |n| n.x == 64 && n.y == 64),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_dijkstra() {
    assert_ne!(
        dijkstra(
            &Pt::new(0, 0),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            |n| n.x == 64 && n.y == 64,
        ),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_fringe() {
    assert_ne!(
        fringe(
            &Pt::new(0, 0),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            Pt::heuristic,
            |n| n.x == 64 && n.y == 64,
        ),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_idastar() {
    assert_ne!(
        idastar(
            &Pt::new(0, 0),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            Pt::heuristic,
            |n| n.x == 64 && n.y == 64,
        ),
        None
    );
}

#[library_benchmark]
fn corner_to_corner_iddfs() {
    assert_ne!(
        iddfs(Pt::new(0, 0), successors, |n| n.x == 5 && n.y == 5),
        None
    );
}

#[library_benchmark]
fn no_path_astar() {
    assert_eq!(
        astar(
            &Pt::new(2, 3),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            |_| 1,
            |_| false,
        ),
        None
    );
}

#[library_benchmark]
fn no_path_bfs() {
    assert_eq!(bfs(&Pt::new(2, 3), successors, |_| false), None);
}

#[library_benchmark]
fn no_path_bfs_bidirectional() {
    assert_eq!(
        bfs_bidirectional(
            &Pt::new(2, 3),
            &Pt::new(u16::MAX, u16::MAX),
            successors,
            |_| vec![]
        ),
        None
    );
}

#[library_benchmark]
fn no_path_dfs() {
    assert_eq!(dfs(Pt::new(2, 3), successors, |_| false), None);
}

#[library_benchmark]
fn no_path_dijkstra() {
    assert_eq!(
        dijkstra(
            &Pt::new(2, 3),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            |_| false,
        ),
        None
    );
}

#[library_benchmark]
fn no_path_fringe() {
    assert_eq!(
        fringe(
            &Pt::new(2, 3),
            |n| successors(n).into_iter().map(|n| (n, 1)),
            |_| 1,
            |_| false,
        ),
        None
    );
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
