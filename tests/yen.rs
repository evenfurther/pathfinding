use pathfinding::prelude::yen;

// A simple tests of Yen's algorithm based on the example and visualization
// from https://en.wikipedia.org/wiki/Yen's_algorithm#Example.
#[test]
fn simple() {
    let result = yen(
        &'c',
        |c| match c {
            'c' => vec![('d', 3), ('e', 2)],
            'd' => vec![('f', 4)],
            'e' => vec![('d', 1), ('f', 2), ('g', 3)],
            'f' => vec![('g', 2), ('h', 1)],
            'g' => vec![('h', 2)],
            'h' => vec![],
            _ => panic!(""),
        },
        |c| *c == 'h',
        3,
    );

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], (vec!['c', 'e', 'f', 'h'], 5));
    assert_eq!(result[1], (vec!['c', 'e', 'g', 'h'], 7));
    assert_eq!(result[2], (vec!['c', 'd', 'f', 'h'], 8));
}

/// Tests that we correctly return fewer routes when
/// we exhaust all possible paths.
#[test]
fn ask_more_than_exist() {
    let result = yen(
        &'c',
        |c| match c {
            'c' => vec![('d', 3), ('e', 2)],
            'd' => vec![('f', 4)],
            'e' => vec![('d', 1), ('f', 2), ('g', 3)],
            'f' => vec![('g', 2), ('h', 1)],
            'g' => vec![('h', 2)],
            'h' => vec![],
            _ => panic!(""),
        },
        |c| *c == 'h',
        10,
    );

    // we asked for 10 but the graph can only produce 7
    assert_eq!(
        result.iter().map(|&(_, c)| c).collect::<Vec<_>>(),
        vec![5, 7, 8, 8, 8, 11, 11]
    );
}

/// Test that we return None in case there is no solution
#[test]
fn no_path() {
    let result = yen(
        &'c',
        |c| match c {
            'c' => vec![('d', 3), ('e', 2)],
            'd' => vec![('f', 4)],
            'e' => vec![('d', 1), ('f', 2), ('g', 3)],
            'f' => vec![('g', 2), ('d', 1)],
            'g' => vec![('e', 2)],
            'h' => vec![],
            _ => panic!(""),
        },
        |c| *c == 'h',
        2,
    );

    assert!(result.is_empty());
}

/// Test that we support loops
#[test]
fn single_node() {
    let result = yen(
        &'c',
        |c| match c {
            'c' => vec![('c', 1)],
            _ => panic!(""),
        },
        |c| *c == 'c',
        2,
    );

    assert_eq!(result, vec![(vec!['c'], 0)]);
}

/// Test that we don't panic if an alternative path is more than two nodes longer than a previous one.
#[test]
fn longer_alternative_path() {
    let result = yen(
        &'c',
        |c| match c {
            'c' => vec![('d', 1), ('h', 1)],
            'd' => vec![('e', 1)],
            'e' => vec![('f', 1)],
            'f' => vec![('g', 1), ('h', 1)],
            'g' => vec![('h', 1)],
            'h' => vec![],
            _ => panic!(""),
        },
        |c| *c == 'h',
        3,
    );

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], (vec!['c', 'h'], 1));
    assert_eq!(result[1], (vec!['c', 'd', 'e', 'f', 'h'], 4));
    assert_eq!(result[2], (vec!['c', 'd', 'e', 'f', 'g', 'h'], 5));
}

/// Check that we return all loopless paths
/// (issue #467)
#[test]
fn all_paths() {
    let mut result = yen(
        &'a',
        |c| match c {
            'a' => vec![('b', 1), ('c', 1), ('d', 1)],
            'b' => vec![('c', 1), ('d', 1)],
            'c' => vec![('b', 1), ('d', 1)],
            'd' => vec![],
            _ => unreachable!(),
        },
        |c| *c == 'd',
        usize::MAX,
    );
    result.sort_unstable();
    assert_eq!(
        result,
        vec![
            (vec!['a', 'b', 'c', 'd'], 3),
            (vec!['a', 'b', 'd'], 2),
            (vec!['a', 'c', 'b', 'd'], 3),
            (vec!['a', 'c', 'd'], 2),
            (vec!['a', 'd'], 1),
        ]
    );
}

#[test]
fn multiple_equal_cost_paths() {
    use std::collections::HashMap;

    // Graph example:
    //     A --> B --> D
    //     A --> C --> D
    // Both paths (A -> B -> D) and (A -> C -> D) have the same cost of 2.

    let mut graph = HashMap::new();
    graph.insert('A', vec![('B', 1), ('C', 1)]);
    graph.insert('B', vec![('D', 1)]);
    graph.insert('C', vec![('D', 1)]);
    graph.insert('D', vec![]); // Goal node

    let successors = |n: &char| {
        let neighbors = match n {
            'A' => "BC",
            'B' | 'C' => "D",
            _ => "",
        };
        neighbors.chars().map(|n| (n, 1))
    };

    // Start is 'A', goal is 'D', and we want 2 shortest paths.
    let paths = yen(&'A', successors, |n| *n == 'D', 2);

    // We expect two distinct paths both with cost 2:
    // Path 1: A -> B -> D
    // Path 2: A -> C -> D
    assert_eq!(paths.len(), 2);

    // Check both paths have total cost of 2 and are distinct
    assert_eq!(paths[0], (vec!['A', 'B', 'D'], 2));
    assert_eq!(paths[1], (vec!['A', 'C', 'D'], 2));
}
