use pathfinding::NodeRefs;
use pathfinding::prelude::{bfs, bfs_bidirectional};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct N(String);

#[test]
fn test_bfs_multiple_starts() {
    let node_a = N("a".into());
    let node_b = N("b".into());
    let node_c = N("c".into());
    let node_d = N("d".into());
    let node_e = N("e".into());

    let graph: HashMap<&N, HashSet<&N>> = HashMap::from_iter([
        (&node_a, HashSet::from([&node_b])),
        (&node_b, HashSet::from([&node_c])),
        (&node_c, HashSet::from([&node_d])),
        (&node_d, HashSet::from([&node_e])),
        (&node_e, HashSet::from([])),
    ]);

    // Single start and end.
    let path = bfs(
        &node_a,
        |n| graph.get(n).unwrap().clone().into_iter().cloned(),
        |n| n == &node_e,
    );
    assert_eq!(
        path,
        Some(
            [&node_a, &node_b, &node_c, &node_d, &node_e]
                .into_iter()
                .cloned()
                .collect()
        )
    );

    // Multiple start and end.
    let start = NodeRefs::from_iter([&node_a, &node_b]);
    let end = NodeRefs::from_iter([&node_d, &node_e]);
    let path = bfs(
        start,
        |n| graph.get(n).unwrap().clone().into_iter().cloned(),
        |n| end.contains(n),
    );
    assert_eq!(
        path,
        Some([&node_b, &node_c, &node_d].into_iter().cloned().collect())
    );
}

#[test]
fn test_bfs_bidirectional_multiple_starts() {
    let node_a = N("a".into());
    let node_b = N("b".into());
    let node_c = N("c".into());
    let node_d = N("d".into());
    let node_e = N("e".into());

    let graph: HashMap<&N, HashSet<&N>> = HashMap::from_iter([
        (&node_a, HashSet::from([&node_b])),
        (&node_b, HashSet::from([&node_c])),
        (&node_c, HashSet::from([&node_d])),
        (&node_d, HashSet::from([&node_e])),
        (&node_e, HashSet::from([])),
    ]);
    let reverse_graph: HashMap<&N, HashSet<&N>> = HashMap::from_iter([
        (&node_a, HashSet::from([])),
        (&node_b, HashSet::from([&node_a])),
        (&node_c, HashSet::from([&node_b])),
        (&node_d, HashSet::from([&node_c])),
        (&node_e, HashSet::from([&node_d])),
    ]);

    // Single start and end.
    let path = bfs_bidirectional(
        &node_a,
        &node_e,
        |n| graph.get(n).unwrap().clone().into_iter().cloned(),
        |n| reverse_graph.get(n).unwrap().clone().into_iter().cloned(),
    );
    assert_eq!(
        path,
        Some(
            [&node_a, &node_b, &node_c, &node_d, &node_e]
                .into_iter()
                .cloned()
                .collect()
        )
    );

    // Multiple start and end.
    let start = NodeRefs::from_iter([&node_a, &node_b]);
    let end = NodeRefs::from_iter([&node_d, &node_e]);
    let path = bfs_bidirectional(
        start,
        end,
        |n| graph.get(n).unwrap().clone().into_iter().cloned(),
        |n| reverse_graph.get(n).unwrap().clone().into_iter().cloned(),
    );
    assert_eq!(
        path,
        Some([&node_b, &node_c, &node_d].into_iter().cloned().collect())
    );
}
