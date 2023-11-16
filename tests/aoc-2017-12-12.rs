// Test from https://adventofcode.com/, 2017-12-12

use lazy_static::lazy_static;
use pathfinding::prelude::*;

use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PIPES: Vec<Vec<usize>> = include_str!("aoc-2017-12-12.txt")
        .lines()
        .map(|line| line
            .replace(" <->", ",")
            .split(", ")
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
}

#[test]
fn method1() {
    let pipes = PIPES
        .iter()
        .map(|l| (l[0], l[1..].to_vec()))
        .collect::<HashMap<_, _>>();
    let all_nodes = pipes.keys().copied().collect::<Vec<_>>();
    let components =
        connected_components(&all_nodes, |&n| pipes.get(&n).cloned().unwrap_or_default());
    assert_eq!(152, components[component_index(&components)[&0]].len());
    assert_eq!(186, components.len());
}

#[test]
fn method2() {
    let (indices, groups) = separate_components(&PIPES);
    let zero = indices[&0];
    assert_eq!(152, indices.values().filter(|&g| *g == zero).count());
    assert_eq!(186, groups.into_iter().collect::<HashSet<_>>().len());
}

#[test]
fn method3() {
    let groups = components(&PIPES);
    let zero = groups.iter().find(|g| g.contains(&0)).unwrap();
    assert_eq!(152, zero.len());
    assert_eq!(186, groups.len());
}
