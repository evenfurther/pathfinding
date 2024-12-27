#![cfg(test)]

// http://bit.ly/2jwqlY6

use pathfinding::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

fn add_successor(
    n: &mut HashMap<Point, Vec<(Point, usize)>>,
    from: &Point,
    to: &Point,
    cost: usize,
) {
    let entry = n.entry(from.clone()).or_default();
    entry.push((to.clone(), cost));
}

type SuccessorInfo = Vec<(Point, usize)>;

fn parse(input: &str) -> (Vec<Point>, HashMap<Point, SuccessorInfo>) {
    let mut nodes = Vec::new();
    let mut successors = HashMap::new();
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<usize>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .for_each(|words| {
            let src = Point {
                row: words[0],
                col: words[1],
            };
            nodes.push(src.clone());
            for n in words[3..].chunks(3) {
                let dst = Point {
                    row: n[0],
                    col: n[1],
                };
                let cost = n[2];
                assert!(cost >= distance(&src, &dst));
                add_successor(&mut successors, &src, &dst, cost);
            }
        });
    (nodes, successors)
}

const fn distance(a: &Point, b: &Point) -> usize {
    a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
}

#[test]
fn main() {
    let expectations = [
        vec![28, 44, 220, 184, 144, 208, 76],
        vec![60, 212, 176, 136, 200, 92],
        vec![252, 216, 176, 240, 36],
        vec![48, 84, 64, 276],
        vec![48, 40, 240],
        vec![72, 200],
        vec![264],
    ];
    let (nodes, graph) = parse(include_str!("r299.data"));
    for (i, start) in nodes[..7].iter().enumerate() {
        for (j, target) in nodes[i + 1..8].iter().enumerate() {
            let expected = expectations[i][j];
            assert_eq!(
                astar(
                    start,
                    |n| graph[n].iter().cloned(),
                    |n| distance(n, target),
                    |n| n == target,
                )
                .unwrap()
                .1,
                expected
            );
            assert_eq!(
                dijkstra(start, |n| graph[n].iter().cloned(), |n| n == target)
                    .unwrap()
                    .1,
                expected
            );
            assert_eq!(
                fringe(
                    start,
                    |n| graph[n].iter().cloned(),
                    |n| distance(n, target),
                    |n| n == target,
                )
                .unwrap()
                .1,
                expected
            );
            if expected < 150 {
                // Longer paths will take too long to run with IDA*.
                assert_eq!(
                    idastar(
                        start,
                        |n| graph[n].iter().cloned(),
                        |n| distance(n, target),
                        |n| n == target,
                    )
                    .unwrap()
                    .1,
                    expected
                );
            }
        }
    }
}
