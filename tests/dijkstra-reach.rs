use itertools::Itertools;
use pathfinding::prelude::{dijkstra_reach, DijkstraReachableItem};
use std::collections::HashMap;

#[test]
fn dijkstra_reach_numbers() {
    let reach = dijkstra_reach(&0, |prev| vec![(prev + 1, 1), (prev * 2, *prev)])
        .take_while(|x| x.total_cost < 100)
        .collect_vec();
    // the total cost should equal to the node's value, since the starting node is 0 and the cost to reach a successor node is equal to the increase in the node's value
    assert!(reach.iter().all(|x| x.node == x.total_cost));
    assert!((0..100).all(|x| reach.iter().any(|y| x == y.total_cost)));

    // dijkstra_reach should return reachable nodes in order of cost
    assert!(reach
        .iter()
        .map(|x| x.total_cost)
        .tuple_windows()
        .all(|(a, b)| b >= a));
}

#[test]
fn dijkstra_reach_graph() {
    //    2     2
    // A --> B --> C
    // \__________/
    //       5
    let mut graph = HashMap::new();
    graph.insert("A", vec![("B", 2), ("C", 5)]);
    graph.insert("B", vec![("C", 2)]);
    graph.insert("C", vec![]);

    let reach = dijkstra_reach(&"A", |prev| graph[prev].clone()).collect_vec();

    // need to make sure that a node won't be returned twice when a better path is found after the first candidate
    assert!(
        reach
            == vec![
                DijkstraReachableItem {
                    node: "A",
                    parent: None,
                    total_cost: 0,
                },
                DijkstraReachableItem {
                    node: "B",
                    parent: Some("A"),
                    total_cost: 2,
                },
                DijkstraReachableItem {
                    node: "C",
                    parent: Some("B"),
                    total_cost: 4,
                },
            ]
    );
}
