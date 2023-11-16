use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::directed::count_paths::count_paths;

fn input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }
    map
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State<'a> {
    current: &'a str,
    small_caves: Vec<&'a str>,
    small_cave_twice: bool,
}

fn solve(small_cave_twice: bool) -> usize {
    let map = input(include_str!("aoc-2021-12.txt"));

    count_paths(
        State {
            current: "start",
            small_caves: Vec::new(),
            small_cave_twice,
        },
        |c| {
            map[&c.current]
                .iter()
                .filter(|&&x| x != "start" && (!c.small_caves.contains(&x) || c.small_cave_twice))
                .map(move |x| State {
                    current: x,
                    small_caves: c
                        .small_caves
                        .iter()
                        .copied()
                        .chain((x.as_bytes()[0] >= b'a').then_some(*x))
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
