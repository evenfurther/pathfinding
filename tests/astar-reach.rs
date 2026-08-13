use itertools::Itertools;
use pathfinding::prelude::{astar, astar_reach, dijkstra_reach, AstarReachableItem};

#[test]
fn astar_reach_graph() {
    //    2     2
    // A --> B --> C
    // \__________/
    //       5
    let mut graph = std::collections::HashMap::new();
    graph.insert("A", vec![("B", 2), ("C", 5)]);
    graph.insert("B", vec![("C", 2)]);
    graph.insert("C", vec![]);

    let reach = astar_reach(&"A", |prev| graph[prev].clone(), |_| 0).collect_vec();

    assert_eq!(
        reach,
        vec![
            AstarReachableItem {
                node: "A",
                parent: None,
                total_cost: 0,
                estimated_cost: 0,
            },
            AstarReachableItem {
                node: "B",
                parent: Some("A"),
                total_cost: 2,
                estimated_cost: 2,
            },
            AstarReachableItem {
                node: "C",
                parent: Some("B"),
                total_cost: 4,
                estimated_cost: 4,
            },
        ]
    );
}

#[test]
fn zero_heuristic_matches_dijkstra_reach() {
    let mut graph = std::collections::HashMap::new();
    graph.insert("A", vec![("B", 2), ("C", 5)]);
    graph.insert("B", vec![("C", 2)]);
    graph.insert("C", vec![]);

    let astar_items = astar_reach(&"A", |prev| graph[prev].clone(), |_| 0).collect_vec();
    let dijkstra_items = dijkstra_reach(&"A", |prev| graph[prev].clone()).collect_vec();

    assert_eq!(astar_items.len(), dijkstra_items.len());
    for (a, d) in astar_items.iter().zip(&dijkstra_items) {
        assert_eq!(a.node, d.node);
        assert_eq!(a.parent, d.parent);
        assert_eq!(a.total_cost, d.total_cost);
        assert_eq!(a.estimated_cost, a.total_cost);
    }
}

#[test]
fn stops_at_goal_with_same_cost_as_astar() {
    const GOAL: (i32, i32) = (4, 4);

    let successors = |&(x, y): &(i32, i32)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), 1_u32))
            .filter(|&((nx, ny), _)| (0..=4).contains(&nx) && (0..=4).contains(&ny))
    };
    let heuristic = |&(x, y): &(i32, i32)| GOAL.0.abs_diff(x) + GOAL.1.abs_diff(y);

    let reached = astar_reach(&(0, 0), successors, heuristic)
        .find(|r| r.node == GOAL)
        .expect("goal not reached");
    let (path, cost) = astar(&(0, 0), successors, heuristic, |n| *n == GOAL).expect("no path");

    assert_eq!(reached.total_cost, cost);
    assert_eq!(reached.estimated_cost, cost);
    assert_eq!(path.last(), Some(&GOAL));
}

#[test]
fn heuristic_expands_fewer_nodes_than_dijkstra() {
    const GOAL: (i32, i32) = (6, 6);

    let successors = |&(x, y): &(i32, i32)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), 1_u32))
            .filter(|&((nx, ny), _)| (0..=6).contains(&nx) && (0..=6).contains(&ny))
    };
    let heuristic = |&(x, y): &(i32, i32)| GOAL.0.abs_diff(x) + GOAL.1.abs_diff(y);

    let guided = astar_reach(&(0, 0), successors, heuristic)
        .take_while(|r| r.node != GOAL)
        .count();
    let unguided = astar_reach(&(0, 0), successors, |_| 0)
        .take_while(|r| r.node != GOAL)
        .count();

    assert!(
        guided < unguided,
        "guided expansions {guided} should be fewer than unguided {unguided}"
    );
}

#[test]
fn is_fused() {
    let mut it = astar_reach(
        &1,
        |&n| vec![(n + 1, 1)].into_iter().filter(|&(x, _)| x <= 3),
        |_| 0,
    );
    assert!(it.next().is_some());
    assert!(it.next().is_some());
    assert!(it.next().is_some());
    for _ in 0..3 {
        assert!(it.next().is_none());
    }
}

#[test]
fn parent_chain_reaches_start() {
    const GOAL: (i32, i32) = (3, 1);
    let successors = |&(x, y): &(i32, i32)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), 1_u32))
            .filter(|&((nx, ny), _)| (0..=3).contains(&nx) && (0..=3).contains(&ny))
    };
    let heuristic = |&(x, y): &(i32, i32)| GOAL.0.abs_diff(x) + GOAL.1.abs_diff(y);

    let mut by_node = std::collections::HashMap::new();
    for item in astar_reach(&(0, 0), successors, heuristic) {
        let at_goal = item.node == GOAL;
        by_node.insert(item.node, item);
        if at_goal {
            break;
        }
    }

    let mut path = vec![GOAL];
    let mut current = GOAL;
    while let Some(parent) = by_node[&current].parent {
        path.push(parent);
        current = parent;
    }
    path.reverse();

    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&GOAL));
    let (_, cost) = astar(&(0, 0), successors, heuristic, |n| *n == GOAL).unwrap();
    assert_eq!(by_node[&GOAL].total_cost, cost);
}
