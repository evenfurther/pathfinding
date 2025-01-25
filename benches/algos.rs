use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use pathfinding::prelude::{
    astar, bfs, bfs_bidirectional, dfs, dijkstra, fringe, idastar, iddfs, separate_components,
};
use rand::seq::SliceRandom;
use rand::{Rng as _, RngCore as _, SeedableRng as _};
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;

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

fn corner_to_corner_astar(c: &mut Criterion) {
    c.bench_function("corner_to_corner_astar", |b| {
        b.iter(|| {
            assert_ne!(
                astar(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    Pt::heuristic,
                    |n| n.x == 64 && n.y == 64,
                ),
                None
            );
        });
    });
}

fn corner_to_corner_bfs(c: &mut Criterion) {
    c.bench_function("corner_to_corner_bfs", |b| {
        b.iter(|| {
            assert_ne!(
                bfs(&Pt::new(0, 0), successors, |n| n.x == 64 && n.y == 64,),
                None
            );
        });
    });
}

fn corner_to_corner_bfs_bidirectional(c: &mut Criterion) {
    c.bench_function("corner_to_corner_bfs_bidirectional", |b| {
        b.iter(|| {
            assert_ne!(
                bfs_bidirectional(&Pt::new(0, 0), &Pt::new(64, 64), successors, successors),
                None
            );
        });
    });
}

fn corner_to_corner_dfs(c: &mut Criterion) {
    c.bench_function("corner_to_corner_dfs", |b| {
        b.iter(|| {
            assert_ne!(
                dfs(Pt::new(0, 0), successors, |n| n.x == 64 && n.y == 64),
                None
            );
        });
    });
}

fn corner_to_corner_dijkstra(c: &mut Criterion) {
    c.bench_function("corner_to_corner_dijkstra", |b| {
        b.iter(|| {
            assert_ne!(
                dijkstra(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |n| n.x == 64 && n.y == 64,
                ),
                None
            );
        });
    });
}

fn corner_to_corner_fringe(c: &mut Criterion) {
    c.bench_function("corner_to_corner_fringe", |b| {
        b.iter(|| {
            assert_ne!(
                fringe(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    Pt::heuristic,
                    |n| n.x == 64 && n.y == 64,
                ),
                None
            );
        });
    });
}

fn corner_to_corner_idastar(c: &mut Criterion) {
    c.bench_function("corner_to_corner_idastar", |b| {
        b.iter(|| {
            assert_ne!(
                idastar(
                    &Pt::new(0, 0),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    Pt::heuristic,
                    |n| n.x == 64 && n.y == 64,
                ),
                None
            );
        });
    });
}

fn corner_to_corner_iddfs(c: &mut Criterion) {
    c.bench_function("corner_to_corner_iddfs", |b| {
        b.iter(|| {
            assert_ne!(
                iddfs(Pt::new(0, 0), successors, |n| n.x == 5 && n.y == 5,),
                None
            );
        });
    });
}

fn no_path_astar(c: &mut Criterion) {
    c.bench_function("no_path_astar", |b| {
        b.iter(|| {
            assert_eq!(
                astar(
                    &Pt::new(2, 3),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |_| 1,
                    |_| false,
                ),
                None
            );
        });
    });
}

fn no_path_bfs(c: &mut Criterion) {
    c.bench_function("no_path_bfs", |b| {
        b.iter(|| assert_eq!(bfs(&Pt::new(2, 3), successors, |_| false), None));
    });
}

fn no_path_bfs_bidirectional(c: &mut Criterion) {
    c.bench_function("no_path_bfs_bidirectional", |b| {
        b.iter(|| {
            assert_eq!(
                bfs_bidirectional(
                    &Pt::new(2, 3),
                    &Pt::new(u16::MAX, u16::MAX),
                    successors,
                    |_| vec![]
                ),
                None
            );
        });
    });
}

fn no_path_dfs(c: &mut Criterion) {
    c.bench_function("no_path_dfs", |b| {
        b.iter(|| assert_eq!(bfs(&Pt::new(2, 3), successors, |_| false), None));
    });
}

fn no_path_dijkstra(c: &mut Criterion) {
    c.bench_function("no_path_dijkstra", |b| {
        b.iter(|| {
            assert_eq!(
                dijkstra(
                    &Pt::new(2, 3),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |_| false,
                ),
                None
            );
        });
    });
}

fn no_path_fringe(c: &mut Criterion) {
    c.bench_function("no_path_fringe", |b| {
        b.iter(|| {
            assert_eq!(
                fringe(
                    &Pt::new(2, 3),
                    |n| successors(n).into_iter().map(|n| (n, 1)),
                    |_| 1,
                    |_| false,
                ),
                None
            );
        });
    });
}

fn bench_separate_components(c: &mut Criterion) {
    c.bench_function("separate_components", |b| {
        let mut rng = XorShiftRng::from_seed([
            3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
        ]);
        let mut seen = HashSet::new();
        let mut components = (0..100)
            .map(|_| {
                let mut component = Vec::new();
                for _ in 0..100 {
                    let node = rng.next_u64();
                    if !seen.contains(&node) {
                        seen.insert(node);
                        component.push(node);
                    }
                }
                component.sort_unstable();
                assert!(
                    !component.is_empty(),
                    "component is empty, rng seed needs changing"
                );
                component
            })
            .collect_vec();
        components.sort_by_key(|c| c[0]);
        let mut groups = components
            .iter()
            .flat_map(|component| {
                let mut component = component.clone();
                component.shuffle(&mut rng);
                let mut subcomponents = Vec::new();
                while !component.is_empty() {
                    let cut = rng.gen_range(0..component.len());
                    let mut subcomponent = component.drain(cut..).collect_vec();
                    if !component.is_empty() {
                        subcomponent.push(component[0]);
                    }
                    subcomponents.shuffle(&mut rng);
                    subcomponents.push(subcomponent);
                }
                subcomponents
            })
            .collect_vec();
        groups.shuffle(&mut rng);
        // The result is already checked in a separate test.
        b.iter(|| separate_components(&groups));
    });
}

// We do not test IDA* and IDDFS with the no_path variant
// as the successive increasing full explorations of the
// maze will take too much time.
criterion_group!(
    benches,
    corner_to_corner_astar,
    corner_to_corner_bfs,
    corner_to_corner_bfs_bidirectional,
    corner_to_corner_dfs,
    corner_to_corner_dijkstra,
    corner_to_corner_fringe,
    corner_to_corner_idastar,
    corner_to_corner_iddfs,
    no_path_astar,
    no_path_bfs,
    no_path_bfs_bidirectional,
    no_path_dfs,
    no_path_dijkstra,
    no_path_fringe,
    bench_separate_components,
);
criterion_main!(benches);
