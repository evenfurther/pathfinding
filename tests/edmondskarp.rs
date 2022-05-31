use pathfinding::directed::edmonds_karp::*;
use std::collections::HashMap;

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
    let (caps, total, mut cut) = flows;
    assert_eq!(caps.len(), 8);
    let caps = HashMap::<(char, char), i32>::from_iter(caps);
    assert_eq!(caps[&('A', 'B')], 2);
    assert_eq!(caps[&('A', 'D')], 3);
    assert_eq!(caps[&('B', 'C')], 2);
    assert_eq!(caps[&('C', 'D')], 1);
    assert_eq!(caps[&('C', 'E')], 1);
    assert_eq!(caps[&('D', 'F')], 4);
    assert_eq!(caps[&('E', 'G')], 1);
    assert_eq!(caps[&('F', 'G')], 4);
    assert_eq!(total, 5);
    cut.sort_unstable();
    assert_eq!(cut, vec![(('A', 'D'), 3), (('C', 'D'), 1), (('E', 'G'), 1)]);
}

fn wikipedia_example<EK: EdmondsKarp<i32>>() {
    check_wikipedia_result(edmonds_karp::<_, _, _, EK>(
        &"ABCDEFGH".chars().collect::<Vec<_>>(),
        &'A',
        &'G',
        successors_wikipedia(),
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

#[allow(clippy::cast_possible_truncation)]
fn wikipedia_progressive_example<EK: EdmondsKarp<i32>>() {
    let successors = successors_wikipedia();
    let size = successors.len();
    let mut ek = EK::new(size, 0, 6);
    for ((from, to), cap) in successors {
        let (_, total, _) = ek.augment();
        assert!(total < 5);
        ek.set_capacity(from as usize - 65, to as usize - 65, cap);
    }
    let (caps, total, flows) = ek.augment();
    let mkletter = |d| char::from_u32(d as u32 + 65).unwrap();
    let mkedge = |((a, b), c)| ((mkletter(a), mkletter(b)), c);
    let caps = caps.into_iter().map(mkedge).collect::<Vec<_>>();
    let flows = flows.into_iter().map(mkedge).collect();
    check_wikipedia_result((caps, total, flows));
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
    let (caps, total, _) = edmonds_karp::<_, _, _, EK>(
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
    modified::<DenseCapacity<i32>>();
}

#[test]
fn modified_sparse() {
    modified::<SparseCapacity<i32>>();
}

#[test]
#[should_panic(expected = "source is greater or equal than size")]
fn empty() {
    let mut ek = DenseCapacity::<i32>::new(0, 0, 0);
    ek.augment();
}

#[test]
#[should_panic(expected = "source not found in vertices")]
fn unknown_source() {
    edmonds_karp_dense(&[1, 2, 3], &0, &3, Vec::<((i32, i32), i32)>::new());
}

#[test]
#[should_panic(expected = "source not found in vertices")]
fn unknown_source_2() {
    edmonds_karp_sparse(&[1, 2, 3], &0, &3, Vec::<((i32, i32), i32)>::new());
}

#[test]
#[should_panic(expected = "sink not found in vertices")]
fn unknown_sink() {
    edmonds_karp_dense(&[1, 2, 3], &1, &4, Vec::<((i32, i32), i32)>::new());
}

#[test]
#[should_panic(expected = "sink not found in vertices")]
fn unknown_sink_2() {
    edmonds_karp_sparse(&[1, 2, 3], &1, &4, Vec::<((i32, i32), i32)>::new());
}

fn str_to_graph(desc: &str) -> (Vec<usize>, Vec<Edge<usize, isize>>) {
    let vertices = (0..desc.lines().count() - 1).collect();
    let edges = desc
        .lines()
        .skip(1)
        .enumerate()
        .flat_map(|(from, line)| {
            line.split_whitespace()
                .skip(1)
                .enumerate()
                .filter_map(|(to, cap)| Some(((from, to), cap.parse().ok()?)))
                .collect::<Vec<_>>()
        })
        .collect();
    (vertices, edges)
}

#[test]
fn mincut_basic() {
    let (vertices, edges) = str_to_graph(
        "  0 1 2 3 4 5
         0 . 5 . 6 . .
         1 . . 4 . 7 .
         2 . . . 6 . 1
         3 4 . 4 . . .
         4 . . . . . 6
         5 5 . . 5 . .",
    );
    let (_, cap, mut mincut) =
        edmonds_karp_dense(&vertices, &vertices[0], vertices.last().unwrap(), edges);
    mincut.sort_unstable();
    assert_eq!((mincut, cap), (vec![((0, 1), 5), ((2, 5), 1)], 6));
}

#[test]
fn mincut_wikipedia() {
    let (vertices, edges) = str_to_graph(
        "  0  1  2  3  4  5  6  7
         0 .  10 .  5  .  15 .  .
         1 .  .  9  4  15 .  .  .
         2 .  .  .  .  15 .  .  10
         3 .  2  .  .  8  4  .  .
         4 .  .  .  .  .  .  15 10
         5 .  .  .  .  .  .  16 .
         6 .  .  .  6  .  .  .  10
         7 .  .  .  .  .  .  .  . ",
    );
    let (_, cap, mut mincut) =
        edmonds_karp_dense(&vertices, &vertices[0], vertices.last().unwrap(), edges);
    mincut.sort_unstable();
    assert_eq!(
        (mincut, cap),
        (vec![((1, 2), 9), ((4, 7), 10), ((6, 7), 10)], 29)
    );
}

#[test]
fn set_capacity_test() {
    let n_nodes = 6;
    let source = 0;
    let target = 5;
    let mut ek = DenseCapacity::new(n_nodes, source, target);
    {
        ek.set_capacity(0, 1, 2);
        ek.set_capacity(0, 2, 2);
        ek.set_capacity(1, 3, 1);
        ek.set_capacity(1, 4, 1);
        ek.set_capacity(2, 3, 1);
        ek.set_capacity(2, 4, 1);
        ek.set_capacity(3, 5, 4);
        ek.set_capacity(4, 3, 2);
    }
    let (_flows, max_flow_value, _) = ek.augment();
    assert_eq!(max_flow_value, 4);
    // reduce capacity from 4 to 2
    let residual_capacities_non_negative = |ek: &DenseCapacity<i32>| {
        for v1 in 0..n_nodes {
            for v2 in 0..n_nodes {
                let c = ek.residual_capacity(v1, v2);
                assert!(c >= 0, "residual_capacity({v1}, {v2}) == {c} < 0");
            }
        }
    };
    ek.set_capacity(3, 5, 2);
    residual_capacities_non_negative(&ek);
    ek.augment();
    residual_capacities_non_negative(&ek);
}
