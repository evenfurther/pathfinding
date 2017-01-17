#![cfg(test)]

// http://bit.ly/2jwqlY6

extern crate pathfinding;

use pathfinding::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

fn add_neighbour(n: &mut HashMap<Point, Vec<(Point, usize)>>,
                 from: &Point,
                 to: &Point,
                 cost: usize) {
    let mut entry = n.entry(from.clone()).or_insert(Vec::new());
    entry.push((to.clone(), cost));
}

fn parse(input: &str) -> (Vec<Point>, HashMap<Point, Vec<(Point, usize)>>) {
    let mut nodes = Vec::new();
    let mut neighbours = HashMap::new();
    for words in input.lines()
        .map(|l| l.split(" ").map(|s| s.parse::<usize>().unwrap_or(0)).collect::<Vec<_>>()) {
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
            add_neighbour(&mut neighbours, &src, &dst, cost);
            add_neighbour(&mut neighbours, &dst, &src, cost);
        }
    }
    (nodes, neighbours)
}

macro_rules! absdiff {
    ($a:expr, $b:expr) => {if $a > $b { $a - $b } else { $b - $a }}
}

fn distance(a: &Point, b: &Point) -> usize {
    absdiff!(a.row, b.row) + absdiff!(a.col, b.col)
}

#[test]
fn main() {
    let expectations = vec![vec![28, 44, 220, 184, 144, 208, 76],
                            vec![60, 212, 176, 136, 200, 92],
                            vec![252, 216, 176, 240, 36],
                            vec![48, 84, 64, 276],
                            vec![48, 40, 240],
                            vec![72, 200],
                            vec![264]];
    let (nodes, graph) = parse(include_str!("r299.data"));
    for (i, start) in nodes[..7].iter().enumerate() {
        for (j, target) in nodes[i + 1..8].iter().enumerate() {
            let expected = expectations[i][j];
            assert_eq!(astar(start,
                             |n| graph[n].iter().cloned(),
                             |n| distance(n, target),
                             |n| n == target)
                           .unwrap()
                           .1,
                       expected);
            assert_eq!(dijkstra(start, |n| graph[n].iter().cloned(), |n| n == target)
                           .unwrap()
                           .1,
                       expected);
            assert_eq!(fringe(start,
                              |n| graph[n].iter().cloned(),
                              |n| distance(n, target),
                              |n| n == target)
                           .unwrap()
                           .1,
                       expected);
        }
    }
}
