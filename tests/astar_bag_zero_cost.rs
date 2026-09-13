//! `astar_bag` used to walk back through optimal parents until it found one with no parents.
//! An edge costing nothing makes a node an optimal parent of itself, or of a node it forms a
//! zero-cost cycle with, so that walk never ended and the first solution never arrived.

use pathfinding::prelude::{astar, astar_bag};

/// The smallest failing case: a single zero-cost self loop.
#[test]
fn a_zero_cost_self_loop_still_yields_the_path() {
    let succ = |&n: &u32| match n {
        0 => vec![(1, 1u32)],
        1 => vec![(1, 0), (2, 1)],
        _ => vec![],
    };
    let (solutions, cost) = astar_bag(&0, succ, |_| 0, |&n| n == 2).expect("a path exists");
    assert_eq!(cost, 2);
    assert_eq!(solutions.collect::<Vec<_>>(), vec![vec![0, 1, 2]]);
}

#[test]
fn a_zero_cost_cycle_still_yields_the_path() {
    let succ = |&n: &u32| match n {
        0 => vec![(1, 1u32)],
        1 => vec![(2, 1), (3, 0)],
        3 => vec![(1, 0)],
        _ => vec![],
    };
    let (solutions, cost) = astar_bag(&0, succ, |_| 0, |&n| n == 2).expect("a path exists");
    assert_eq!(cost, 2);
    assert_eq!(solutions.collect::<Vec<_>>(), vec![vec![0, 1, 2]]);
}

/// Zero-cost edges that do not form a cycle were never a problem, and must keep working.
#[test]
fn zero_cost_edges_without_a_cycle_are_unaffected() {
    let succ = |&n: &u32| match n {
        0 => vec![(1, 0u32), (2, 0)],
        1 | 2 => vec![(3, 1)],
        _ => vec![],
    };
    let (solutions, cost) = astar_bag(&0, succ, |_| 0, |&n| n == 3).expect("a path exists");
    assert_eq!(cost, 1);
    let mut found = solutions.collect::<Vec<_>>();
    found.sort_unstable();
    assert_eq!(found, vec![vec![0, 1, 3], vec![0, 2, 3]]);
}

struct Rng(u64);

impl Rng {
    const fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }

    fn below(&mut self, n: usize) -> usize {
        let n = u64::try_from(n).expect("orders used here are small");
        usize::try_from(self.next() % n).expect("a value below n fits in a usize")
    }

    fn cost(&mut self) -> u32 {
        u32::try_from(self.next() % 4).expect("a value below 4 fits in a u32")
    }
}

/// Random graphs, deliberately including zero-cost edges, parallel edges and self loops.
fn random_graph(order: usize, rng: &mut Rng) -> Vec<Vec<(usize, u32)>> {
    let mut out = vec![Vec::new(); order];
    for edges in &mut out {
        for _ in 0..rng.below(3) {
            edges.push((rng.below(order), rng.cost()));
        }
    }
    out
}

/// Every distinct simple path from `start` to `goal` of minimum cost, found by brute force.
fn shortest_simple_paths(
    graph: &[Vec<(usize, u32)>],
    start: usize,
    goal: usize,
) -> (Option<u32>, Vec<Vec<usize>>) {
    let mut best: Option<u32> = None;
    let mut paths: Vec<Vec<usize>> = Vec::new();
    let mut stack = vec![(vec![start], 0u32)];
    while let Some((path, cost)) = stack.pop() {
        let last = *path.last().expect("paths are never empty");
        if last == goal {
            match best {
                Some(b) if cost > b => {}
                Some(b) if cost == b => paths.push(path),
                _ => {
                    best = Some(cost);
                    paths = vec![path];
                }
            }
            continue;
        }
        for &(next, edge) in &graph[last] {
            if !path.contains(&next) {
                let mut extended = path.clone();
                extended.push(next);
                stack.push((extended, cost + edge));
            }
        }
    }
    paths.sort_unstable();
    paths.dedup();
    (best, paths)
}

#[test]
fn agrees_with_brute_force_on_random_graphs_with_zero_costs() {
    let mut rng = Rng(0xABCD_0007);
    let mut checked = 0;
    for trial in 0..400 {
        let order = 2 + rng.below(7);
        let graph = random_graph(order, &mut rng);
        let (start, goal) = (rng.below(order), rng.below(order));
        if start == goal {
            continue;
        }
        checked += 1;

        let (want_cost, want_paths) = shortest_simple_paths(&graph, start, goal);
        let got = astar_bag(&start, |&u| graph[u].clone(), |_| 0, |&u| u == goal);

        match (want_cost, got) {
            (None, None) => {}
            (Some(want), Some((solutions, cost))) => {
                assert_eq!(cost, want, "trial {trial}: wrong cost");
                let mut found = solutions.collect::<Vec<_>>();
                let before = found.len();
                found.sort_unstable();
                found.dedup();
                assert_eq!(found.len(), before, "trial {trial}: duplicate solutions");
                assert_eq!(
                    found, want_paths,
                    "trial {trial}: wrong set of shortest paths from {start} to {goal}"
                );
            }
            (want, got) => panic!(
                "trial {trial}: brute force {want:?} but astar_bag {:?}",
                got.map(|(_, c)| c)
            ),
        }
    }
    // start == goal is skipped, and with graphs this small it comes up often.
    assert!(
        checked > 250,
        "expected most trials to be usable, only {checked} were"
    );
}

/// Whatever `astar_bag` reports must match what `astar` reports on the same graph.
#[test]
fn agrees_with_astar_on_random_graphs_with_zero_costs() {
    let mut rng = Rng(0x5EED_9001);
    for trial in 0..400 {
        let order = 2 + rng.below(10);
        let graph = random_graph(order, &mut rng);
        let (start, goal) = (rng.below(order), rng.below(order));

        let single = astar(&start, |&u| graph[u].clone(), |_| 0, |&u| u == goal);
        let bag = astar_bag(&start, |&u| graph[u].clone(), |_| 0, |&u| u == goal);

        match (single, bag) {
            (None, None) => {}
            (Some((_, c1)), Some((solutions, c2))) => {
                assert_eq!(c1, c2, "trial {trial}: astar {c1} but astar_bag {c2}");
                assert!(
                    solutions.take(1).count() == 1,
                    "trial {trial}: astar_bag produced no solution despite reporting a cost"
                );
            }
            (a, b) => panic!(
                "trial {trial}: astar {:?} but astar_bag {:?}",
                a.map(|(_, c)| c),
                b.map(|(_, c)| c)
            ),
        }
    }
}
