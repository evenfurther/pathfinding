extern crate pathfinding;

use pathfinding::*;
use std::collections::HashMap;

/// Return a list of edges with their capacities.
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
            .map(|(s, c)| ((s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()), c)),
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

fn wikipedia_example<EK: EdmondsKarp<i32>>() {
    check_wikipedia_result(edmonds_karp::<_, _, _, EK>(
        &"ABCDEFGH".chars().collect::<Vec<_>>(),
        &'A',
        &'G',
        neighbours_wikipedia(),
    ));
}

#[test]
fn wikipedia_example_dense() {
    wikipedia_example::<DenseCapacity<_>>();
}

#[test]
fn wikipedia_example_sparse() {
    wikipedia_example::<SparseCapacity<_>>();
}

fn wikipedia_progressive_example<EK: EdmondsKarp<i32>>() {
    let neighbours = neighbours_wikipedia();
    let size = neighbours.len();
    let mut ek = EK::new(size, 0, 6);
    for ((from, to), cap) in neighbours {
        let (_, total) = ek.augment();
        assert!(total < 5);
        ek.set_capacity(from as usize - 65, to as usize - 65, cap);
    }
    let (caps, total) = ek.augment();
    let caps = caps.into_iter()
        .map(|((from, to), cap)| (((from + 65) as u8 as char, (to + 65) as u8 as char), cap))
        .collect::<Vec<_>>();
    check_wikipedia_result((caps, total));
}

#[test]
fn wikipedia_progressive_example_dense() {
    wikipedia_progressive_example::<DenseCapacity<_>>();
}

#[test]
fn wikipedia_progressive_example_sparse() {
    wikipedia_progressive_example::<SparseCapacity<_>>();
}

fn disconnected<EK: EdmondsKarp<isize>>() {
    let (caps, total) = edmonds_karp::<_, _, _, EK>(
        &['A', 'B'],
        &'A',
        &'B',
        std::iter::empty::<((char, char), isize)>(),
    );
    assert_eq!(caps.len(), 0);
    assert_eq!(total, 0);
}

#[test]
fn disconnected_dense() {
    disconnected::<DenseCapacity<_>>();
}

#[test]
fn disconnected_sparse() {
    disconnected::<SparseCapacity<_>>();
}

fn modified<EK: EdmondsKarp<i32>>() {
    // Graph is:
    //
    // 0 -(6)-> 1 -(5)-> 2 -(7)-> 3
    // |                          ^
    // +--(4)-> 4 -(8)-> 5 -(9)---+
    //
    // Upper branch has capacity 5, lower branch 4.
    let mut ek = EK::new(6, 0, 3);
    ek.set_capacity(0, 1, 6);
    ek.set_capacity(1, 2, 5);
    ek.set_capacity(2, 3, 7);
    ek.set_capacity(0, 4, 4);
    ek.set_capacity(4, 5, 8);
    ek.set_capacity(5, 3, 9);
    assert_eq!(ek.augment().1, 9);
    // Set lower branch capacity to 5.
    ek.set_capacity(0, 4, 5);
    assert_eq!(ek.augment().1, 10);
    // Try setting lower branch individual capacities
    // to 4 one at a time.
    for &(from, to) in &[(0, 4), (4, 5), (5, 3)] {
        ek.set_capacity(from, to, 4);
        assert_eq!(ek.augment().1, 9);
        ek.set_capacity(from, to, 5);
        assert_eq!(ek.augment().1, 10);
    }
    // Set capacity 0->4 to 4.
    ek.set_capacity(0, 4, 4);
    assert_eq!(ek.augment().1, 9);
    // Add a branch 1->4 of 2.
    ek.set_capacity(1, 4, 2);
    assert_eq!(ek.augment().1, 10);
}

#[test]
fn modified_dense() {
    modified::<DenseCapacity<i32>>()
}

#[test]
fn modified_sparse() {
    modified::<SparseCapacity<i32>>()
}

#[test]
#[should_panic]
fn empty() {
    let mut ek = DenseCapacity::<i32>::new(0, 0, 0);
    ek.augment();
}

#[test]
#[should_panic]
fn unknown_source() {
    edmonds_karp_dense(&[1, 2, 3], &0, &3, Vec::<((i32, i32), i32)>::new());
}

#[test]
#[should_panic]
fn unknown_sink() {
    edmonds_karp_dense(&[1, 2, 3], &1, &4, Vec::<((i32, i32), i32)>::new());
}
