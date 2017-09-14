extern crate pathfinding;

use pathfinding::*;
use std::collections::HashMap;

#[test]
fn wikipedia_example() {
    let (caps, total) = edmondskarp(
        &"ABCDEFGH".chars().collect::<Vec<_>>(),
        &'A',
        &'G',
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
    );
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
fn disconnected() {
    let (caps, total) = edmondskarp(
        &['A', 'B'],
        &'A',
        &'B',
        std::iter::empty::<((char, char), isize)>(),
    );
    assert_eq!(caps.len(), 0);
    assert_eq!(total, 0);
}
