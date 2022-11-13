use pathfinding::directed::edmonds_karp::*;
use std::collections::{HashMap, HashSet};

/// Return a list of edges with their capacities.
fn successors_wikipedia() -> Vec<((char, char), i32)> {
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
    ]
    .into_iter()
    .map(|(s, c)| {
        let mut name = s.chars();
        ((name.next().unwrap(), name.next().unwrap()), c)
    })
    .collect()
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
        successors_wikipedia(),
    ));
    // Run the test again, now with the min cut part.
    let (flow, cut) = edmonds_karp_mincut::<_, _, _, EK>(
        &"ABCDEFGH".chars().collect::<Vec<_>>(),
        &'A',
        &'G',
        successors_wikipedia(),
    );
    check_wikipedia_result(flow);
    let source_set: HashSet<char> = cut.0.into_iter().collect();
    let ref_set = HashSet::from(['A', 'B', 'C', 'E']);
    assert_eq!(source_set.difference(&ref_set).count(), 0);
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
    let successors = successors_wikipedia();
    let size = successors.len();
    let mut ek = EK::new(size, 0, 6);
    for ((from, to), cap) in successors {
        let (_, total) = ek.augment();
        assert!(total < 5);
        ek.set_capacity(from as usize - 65, to as usize - 65, cap);
    }
    let (caps, total) = ek.augment();
    let caps = caps
        .into_iter()
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

fn str_to_graph(desc: &str) -> (Vec<usize>, Vec<((usize, usize), isize)>) {
    let vertices = desc
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(i, _)| i)
        .collect();
    let edges: Vec<((usize, usize), isize)> = desc
        .lines()
        .skip(1)
        .enumerate()
        .map(|(from, line)| {
            line.split_whitespace()
                .skip(1)
                .map(|item| item.parse::<isize>())
                .enumerate()
                .filter(|(_, result)| result.is_ok())
                .map(|(to, result)| ((from, to), result.unwrap()))
                .collect::<Vec<((usize, usize), isize)>>()
        })
        .reduce(|mut accum, mut item| {
            accum.append(&mut item);
            accum
        })
        .unwrap();
    (vertices, edges)
}

#[test]
fn mincut_basic() {
    let graph_str = "  0 1 2 3 4 5\n\
                     0 . 5 . 6 . .\n\
                     1 . . 4 . 7 .\n\
                     2 . . . 6 . 1\n\
                     3 4 . 4 . . .\n\
                     4 . . . . . 6\n\
                     5 5 . . 5 . .\n";
    let (vertices, edges) = str_to_graph(graph_str);
    let (_, mincut) = edmonds_karp_mincut_dense(
        &vertices,
        &vertices[0],
        &vertices.last().unwrap(),
        edges.into_iter(),
    );
    assert_eq!(
        mincut.0.into_iter().collect::<HashSet<_>>(),
        HashSet::from([0, 2, 3]),
    );
    assert_eq!(mincut.1, 6);
}

#[test]
fn mincut_wikipedia() {
    let graph_str = "  0  1  2  3  4  5  6  7 \n\
                     0 .  10 .  5  .  15 .  . \n\
                     1 .  .  9  4  15 .  .  . \n\
                     2 .  .  .  .  15 .  .  10\n\
                     3 .  2  .  .  8  4  .  . \n\
                     4 .  .  .  .  .  .  15 10\n\
                     5 .  .  .  .  .  .  16 . \n\
                     6 .  .  .  6  .  .  .  10\n\
                     7 .  .  .  .  .  .  .  . \n";
    let (vertices, edges) = str_to_graph(graph_str);
    let (_, mincut) = edmonds_karp_mincut_dense(
        &vertices,
        &vertices[0],
        &vertices.last().unwrap(),
        edges.into_iter(),
    );
    assert_eq!(
        mincut.0.into_iter().collect::<HashSet<_>>(),
        HashSet::from([0, 1, 3, 4, 5, 6]),
    );
    assert_eq!(mincut.1, 29);
}
