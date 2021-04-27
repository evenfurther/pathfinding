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
        2,
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
    assert_eq!(result.len(), 7);
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
