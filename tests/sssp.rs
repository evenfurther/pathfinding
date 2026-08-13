use pathfinding::prelude::{build_path, dijkstra, dijkstra_all, sssp, sssp_all};
use rand::{rngs, RngExt as _};
use std::collections::HashMap;

#[expect(clippy::trivially_copy_pass_by_ref)]
fn successors(&n: &u32) -> Vec<(u32, usize)> {
    if n <= 4 {
        vec![(n * 2, 10), (n * 2 + 1, 10)]
    } else {
        vec![]
    }
}

#[test]
fn matches_dijkstra_all_on_small_tree() {
    let sssp_map = sssp_all(&1, successors);
    let dijkstra_map = dijkstra_all(&1, successors);
    assert_eq!(sssp_map.len(), dijkstra_map.len());
    for (node, (_, cost)) in &dijkstra_map {
        assert_eq!(sssp_map[node].1, *cost, "cost mismatch at {node}");
    }
}

#[test]
fn sssp_path_matches_dijkstra() {
    let (path, cost) = sssp(&1, successors, |&n| n == 9).unwrap();
    let (d_path, d_cost) = dijkstra(&1, successors, |&n| n == 9).unwrap();
    assert_eq!(cost, d_cost);
    assert_eq!(path, d_path);
    assert_eq!(build_path(&9, &sssp_all(&1, successors)), path);
}

#[test]
fn start_is_goal() {
    let (path, cost) = sssp(&1, successors, |&n| n == 1).unwrap();
    assert_eq!(path, vec![1]);
    assert_eq!(cost, 0);
}

#[test]
fn unreachable_goal() {
    assert!(sssp(&1, successors, |&n| n == 100).is_none());
}

fn build_network(size: usize) -> pathfinding::prelude::Matrix<usize> {
    let mut network = pathfinding::prelude::Matrix::new(size, size, 0);
    let mut rng = rngs::ThreadRng::default();
    for a in 0..size {
        for b in 0..size {
            if rng.random_ratio(2, 3) {
                network[(a, b)] = rng.random::<u16>() as usize + 1;
            }
        }
    }
    network
}

fn neighbours(
    network: pathfinding::prelude::Matrix<usize>,
) -> impl FnMut(&usize) -> Vec<(usize, usize)> {
    move |&a| {
        (0..network.rows)
            .filter_map(|b| match network[(a, b)] {
                0 => None,
                p => Some((b, p)),
            })
            .collect()
    }
}

#[test]
fn random_graphs_match_dijkstra_costs() {
    const SIZE: usize = 40;
    let network = build_network(SIZE);
    for start in 0..SIZE {
        let sssp_map = sssp_all(&start, neighbours(network.clone()));
        let dijkstra_map = dijkstra_all(&start, neighbours(network.clone()));
        assert_eq!(
            costs_only(&sssp_map),
            costs_only(&dijkstra_map),
            "costs differ from start {start} in {network:?}"
        );
    }
}

fn costs_only<N: Eq + std::hash::Hash + Clone, C: Copy>(map: &HashMap<N, (N, C)>) -> HashMap<N, C> {
    map.iter().map(|(n, (_, c))| (n.clone(), *c)).collect()
}

#[test]
fn grid_matches_dijkstra() {
    let successors = |&(x, y): &(i32, i32)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), 1_u32))
            .filter(|&((nx, ny), _)| (0..=8).contains(&nx) && (0..=8).contains(&ny))
    };
    let sssp_map = sssp_all(&(0, 0), successors);
    let dijkstra_map = dijkstra_all(&(0, 0), successors);
    assert_eq!(costs_only(&sssp_map), costs_only(&dijkstra_map));
    let (path, cost) = sssp(&(0, 0), successors, |n| *n == (8, 8)).unwrap();
    assert_eq!(cost, 16);
    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&(8, 8)));
}
