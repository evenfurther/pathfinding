use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::directed::count_paths::count_paths;

fn input() -> HashMap<String, Vec<String>> {
    let input = include_str!("aoc-2021-12.txt");
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        map.entry(a.to_string()).or_default().push(b.to_string());
        map.entry(b.to_string()).or_default().push(a.to_string());
    }
    map
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    current: String,
    small_caves: Vec<String>,
    small_cave_twice: bool,
}

fn solve(small_cave_twice: bool) -> usize {
    let map = input();

    count_paths(
        State {
            current: "start".to_string(),
            small_caves: Vec::new(),
            small_cave_twice,
        },
        |c| {
            map[&c.current]
                .iter()
                .filter(|x| x != &"start" && (!c.small_caves.contains(x) || c.small_cave_twice))
                .map(move |x| State {
                    current: x.to_string(),
                    small_caves: c
                        .small_caves
                        .iter()
                        .cloned()
                        .chain([x.to_string()])
                        .filter(|x| x.chars().next().unwrap().is_lowercase())
                        .collect(),
                    small_cave_twice: c.small_cave_twice && !c.small_caves.contains(x),
                })
                .collect_vec()
        },
        |c| c.current == "end",
    )
}

#[test]
fn part1() {
    assert_eq!(solve(false), 226);
}

#[test]
fn part2() {
    assert_eq!(solve(true), 3509);
}
