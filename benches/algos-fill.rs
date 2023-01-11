// This version uses a filler in the Pt structure to increase
// the cost of cloning a node.

use codspeed_criterion_compat::*;
use pathfinding::prelude::{astar, bfs, dfs, dijkstra, fringe, idastar, iddfs};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pt {
    x: u16,
    y: u16,
    _fill: [u64; 32],
}

impl Pt {
    fn new(x: u16, y: u16) -> Pt {
        Pt {
            x,
            y,
            _fill: [0u64; 32],
        }
    }

    #[inline]
    fn heuristic(p: &Pt) -> usize {
        (64 - p.x - p.y) as usize
    }
}

#[inline]
fn successors(pt: &Pt) -> Vec<Pt> {
    let mut ret = Vec::with_capacity(4);
    if 0 < pt.x {
        ret.push(Pt::new(pt.x - 1, pt.y))
    }
    if pt.x < 32 {
        ret.push(Pt::new(pt.x + 1, pt.y))
    }
    if 0 < pt.y {
        ret.push(Pt::new(pt.x, pt.y - 1))
    }
    if pt.y < 32 {
        ret.push(Pt::new(pt.x, pt.y + 1))
    }
    ret
}

fn corner_to_corner_astar(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_astar", |b| {
        b.iter(|| {
            assert_ne!(
                astar(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    Pt::heuristic,
                    |n| n.x == 32 && n.y == 32,
                ),
                None
            )
        })
    });
}

fn corner_to_corner_bfs(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_bfs", |b| {
        b.iter(|| {
            assert_ne!(
                bfs(&Pt::new(0, 0), successors, |n| n.x == 32 && n.y == 32,),
                None
            )
        })
    });
}

fn corner_to_corner_dfs(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_dfs", |b| {
        b.iter(|| {
            assert_ne!(
                dfs(Pt::new(0, 0), successors, |n| n.x == 32 && n.y == 32),
                None
            )
        })
    });
}

fn corner_to_corner_dijkstra(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_dijkstra", |b| {
        b.iter(|| {
            assert_ne!(
                dijkstra(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |n| n.x == 32 && n.y == 32,
                ),
                None
            )
        })
    });
}

fn corner_to_corner_fringe(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_fringe", |b| {
        b.iter(|| {
            assert_ne!(
                fringe(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    Pt::heuristic,
                    |n| n.x == 32 && n.y == 32,
                ),
                None
            )
        })
    });
}

fn corner_to_corner_idastar(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_idastar", |b| {
        b.iter(|| {
            assert_ne!(
                idastar(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    Pt::heuristic,
                    |n| n.x == 32 && n.y == 32,
                ),
                None
            )
        })
    });
}

fn corner_to_corner_iddfs(c: &mut Criterion) {
    c.bench_function("fill-corner_to_corner_iddfs", |b| {
        b.iter(|| {
            assert_ne!(
                iddfs(Pt::new(0, 0), successors, |n| n.x == 5 && n.y == 5,),
                None
            )
        })
    });
}

fn no_path_astar(c: &mut Criterion) {
    c.bench_function("fill-no_path_astar", |b| {
        b.iter(|| {
            assert_eq!(
                astar(
                    &Pt::new(2, 3),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |_| 1,
                    |_| false,
                ),
                None
            )
        })
    });
}

fn no_path_bfs(c: &mut Criterion) {
    c.bench_function("fill-no_path_bfs", |b| {
        b.iter(|| assert_eq!(bfs(&Pt::new(2, 3), successors, |_| false), None))
    });
}

fn no_path_dijkstra(c: &mut Criterion) {
    c.bench_function("fill-no_path_dijkstra", |b| {
        b.iter(|| {
            assert_eq!(
                dijkstra(
                    &Pt::new(2, 3),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |_| false,
                ),
                None
            )
        })
    });
}

fn no_path_fringe(c: &mut Criterion) {
    c.bench_function("fill-no_path_fringe", |b| {
        b.iter(|| {
            assert_eq!(
                fringe(
                    &Pt::new(2, 3),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |_| 1,
                    |_| false,
                ),
                None
            )
        })
    });
}

// We do not test IDA* and IDDFS with the no_path variant
// as the successive increasing full explorations of the
// maze will take too much time.
criterion_group!(
    benches,
    corner_to_corner_astar,
    corner_to_corner_bfs,
    corner_to_corner_dfs,
    corner_to_corner_dijkstra,
    corner_to_corner_fringe,
    corner_to_corner_idastar,
    corner_to_corner_iddfs,
    no_path_astar,
    no_path_bfs,
    no_path_dijkstra,
    no_path_fringe,
);
criterion_main!(benches);
