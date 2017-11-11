#![cfg(feature = "edmonds_karp")]
extern crate pathfinding;

use pathfinding::*;
use std::collections::HashMap;

fn neighbours_wikipedia() -> Vec<((char, char), i32)> {
    Box::new(
        vec![
            ("AB", 3),
            ("AD", 3),
            ("BC", 4),
            ("CA", 3),
            ("CD", 1),
            ("CE", 2),
            ("DE", 2),
            ("DF", 6),
            ("EB", 1),
            ("EG", 1),
            ("FG", 9),
        ].into_iter()
            .map(|(s, c)| {
                ((s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()), c)
            }),
    ).collect()
}

fn check_wikipedia_result(flows: EKFlows<char, i32>) {
    let (caps, total) = flows;
    assert_eq!(caps.len(), 8);
    let caps = caps.into_iter().collect::<HashMap<(char, char), i32>>();
    assert_eq!(caps[&('A', 'B')], 2);
    assert_eq!(caps[&('A', 'D')], 3);
    assert_eq!(caps[&('B', 'C')], 2);
    assert_eq!(caps[&('C', 'D')], 1);
    assert_eq!(caps[&('C', 'E')], 1);
    assert_eq!(caps[&('D', 'F')], 4);
    assert_eq!(caps[&('E', 'G')], 1);
    assert_eq!(caps[&('F', 'G')], 4);
    assert_eq!(total, 5);
}

#[test]
fn wikipedia_example() {
    check_wikipedia_result(edmonds_karp(
        &"ABCDEFGH".chars().collect::<Vec<_>>(),
        &'A',
        &'G',
        neighbours_wikipedia(),
    ));
}

#[test]
fn wikipedia_progressive_example() {
    let neighbours = neighbours_wikipedia();
    let size = neighbours.len();
    let mut ek = EdmondsKarp::new(size, 0, 6);
    let mut capacities = SquareMatrix::new(size, 0);
    for ((from, to), cap) in neighbours {
        let (_, total) = ek.augment(&capacities);
        assert!(total < 5);
        let reset = ek.set_capacity(&mut capacities, from as usize - 65, to as usize - 65, cap);
        assert!(total == 0 || !reset);
    }
    let (caps, total) = ek.augment(&capacities);
    let caps = caps.into_iter()
        .map(|((from, to), cap)| {
            (((from + 65) as u8 as char, (to + 65) as u8 as char), cap)
        })
        .collect::<Vec<_>>();
    check_wikipedia_result((caps, total));
}

#[test]
fn disconnected() {
    let (caps, total) = edmonds_karp(
        &['A', 'B'],
        &'A',
        &'B',
        std::iter::empty::<((char, char), isize)>(),
    );
    assert_eq!(caps.len(), 0);
    assert_eq!(total, 0);
}
