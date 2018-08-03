// Test from https://adventofcode.com/, 2017-12-12

extern crate pathfinding;

use pathfinding::prelude::*;

use std::collections::{HashMap, HashSet};

#[test]
fn method1() {
    let pipes = include_str!("aoc-2017-12-12.txt")
        .lines()
        .map(|line| {
            let line = line.replace(" <->", ",");
            let mut words = line.split(", ").map(|w| w.parse::<usize>().unwrap());
            (words.next().unwrap(), words.collect::<Vec<_>>())
        }).collect::<HashMap<_, _>>();
    let all_nodes = pipes.keys().cloned().collect::<Vec<_>>();
    let components = connected_components(&all_nodes, |&n| {
        pipes.get(&n).cloned().unwrap_or_else(|| vec![])
    });
    assert_eq!(152, components[component_index(&components)[&0]].len());
    assert_eq!(186, components.len());
}

#[test]
fn method2() {
    let pipes = include_str!("aoc-2017-12-12.txt")
        .lines()
        .map(|line| {
            line.replace(" <->", ",")
                .split(", ")
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    let (indices, groups) = separate_components(&pipes);
    let zero = indices[&0];
    assert_eq!(152, indices.values().filter(|&g| *g == zero).count());
    assert_eq!(186, groups.into_iter().collect::<HashSet<_>>().len());
}

#[test]
fn method3() {
    let pipes = include_str!("aoc-2017-12-12.txt")
        .lines()
        .map(|line| {
            line.replace(" <->", ",")
                .split(", ")
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    let groups = components(&pipes);
    let zero = groups.iter().find(|g| g.contains(&0)).unwrap();
    assert_eq!(152, zero.len());
    assert_eq!(186, groups.len());
}
