//! This example demonstrates the bidirectional Dijkstra algorithm, and compares it with the
//! regular Dijkstra algorithm on a large weighted grid.
//!
//! Both searches return the same shortest-path cost, but the bidirectional variant usually settles
//! fewer nodes: instead of growing a single frontier all the way from the start to the goal, it
//! grows two smaller frontiers that meet in the middle.
//!
//! We search from the centre of the grid to a corner. A single Dijkstra search from the centre
//! must grow its frontier outwards until it reaches the corner, by which point it has covered
//! essentially the whole grid. The bidirectional search instead grows one frontier around the
//! centre and one around the corner; they meet roughly halfway, so together they touch far fewer
//! cells.

use pathfinding::prelude::{dijkstra, dijkstra_bidirectional};
use std::time::Instant;

const SIZE: i32 = 400;
const START: (i32, i32) = (SIZE / 2, SIZE / 2);
const GOAL: (i32, i32) = (SIZE, SIZE);

/// Moving between two orthogonally adjacent cells costs one plus a small penalty that depends on
/// both endpoints, so the edge weights are not all identical and Dijkstra cannot be replaced by a
/// plain BFS. The penalty is symmetric in the two cells, so the grid is undirected and the same
/// closure describes both successors and predecessors.
#[expect(clippy::trivially_copy_pass_by_ref)]
fn neighbours(&(x, y): &(i32, i32)) -> Vec<((i32, i32), usize)> {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter(|&(nx, ny)| (0..=SIZE).contains(&nx) && (0..=SIZE).contains(&ny))
        .map(|(nx, ny)| ((nx, ny), 1 + (x + y + nx + ny).unsigned_abs() as usize % 3))
        .collect()
}

fn main() {
    let instant = Instant::now();
    let (_, cost) = dijkstra(&START, neighbours, |p| *p == GOAL).expect("no path found");
    let duration_dijkstra = instant.elapsed();

    let instant = Instant::now();
    let (_, cost_bidirectional) =
        dijkstra_bidirectional(&START, &GOAL, neighbours, neighbours).expect("no path found");
    let duration_bidirectional = instant.elapsed();

    assert_eq!(cost, cost_bidirectional);

    println!("Shortest path cost: {cost}");
    println!("Dijkstra took {duration_dijkstra:?}");
    println!("Bidirectional Dijkstra took {duration_bidirectional:?}");
}
