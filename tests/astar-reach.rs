use itertools::Itertools;
use pathfinding::prelude::{AstarReachableItem, astar, astar_reach, dijkstra_reach};

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

/// An admissible heuristic need not be consistent, and when it is not, A* finds a cheaper route
/// to a node it has already expanded and expands it again. The iterator has to do the same, or
/// it reports costs that are simply wrong.
///
/// Taken from the review of this pull request: `h(A) = 0` while `h(B) = 2`, so `h` drops by 2
/// across the single edge `B -> A`, which costs 1. Reaching `A` looks best at cost 3 until `B`
/// is expanded and offers it at cost 2.
#[test]
fn inconsistent_heuristic_reopens_like_astar() {
    const START: char = 'S';
    let successors = |&n: &char| match n {
        'S' => vec![('A', 3), ('B', 1)],
        'B' => vec![('A', 1)],
        'A' => vec![('G', 1)],
        _ => vec![],
    };
    let heuristic = |&n: &char| match n {
        'B' => 2,
        _ => 0,
    };

    let by_astar = astar(&START, successors, heuristic, |&n| n == 'G').expect("no path");
    assert_eq!(by_astar.1, 3, "the cheapest route is S -> B -> A -> G");

    // The iterator must agree with `astar` about the goal's cost.
    let reached = astar_reach(&START, successors, heuristic)
        .find(|item| item.node == 'G')
        .expect("goal never reached");
    assert_eq!(
        reached.total_cost, by_astar.1,
        "iterator reported {} for the goal, astar says {}",
        reached.total_cost, by_astar.1
    );

    // `A` is expanded twice: once at 3, then again at 2 once `B` has been seen.
    let costs_for_a = astar_reach(&START, successors, heuristic)
        .filter(|item| item.node == 'A')
        .map(|item| item.total_cost)
        .collect::<Vec<_>>();
    assert_eq!(costs_for_a, vec![3, 2]);
}

/// `estimated_cost` must be the priority that actually selected the node, not a fresh call to
/// the heuristic when the node is expanded.
///
/// The heuristic is an `FnMut`, so it is allowed to be stateful — instrumented to count calls,
/// or memoising something expensive. Calling it a second time for a node already queued both
/// reports a value that never took part in the search and perturbs the search being observed.
/// The heuristic here answers differently on a second call for the same node, which makes the
/// difference visible: the reported `f` must be the one computed when the node was queued.
#[test]
fn the_heuristic_is_not_called_again_for_expanded_nodes() {
    // Only right and down, so every route to a cell is the same length and no cell is ever
    // requeued more cheaply. Each is therefore queued exactly once.
    let successors = |&(x, y): &(i32, i32)| {
        [(1, 0), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), 1_u32))
            .filter(|&((nx, ny), _)| (0..4).contains(&nx) && (0..4).contains(&ny))
    };
    let goal = (3, 3);
    #[expect(clippy::cast_sign_loss)]
    let plain = move |&(x, y): &(i32, i32)| ((goal.0 - x) + (goal.1 - y)) as u32;

    let mut asked = std::collections::HashSet::new();
    let once_only = |n: &(i32, i32)| {
        if asked.insert(*n) {
            plain(n)
        } else {
            0 // a second question about the same node gets a different answer
        }
    };
    let items = astar_reach(&(0, 0), successors, once_only).collect::<Vec<_>>();

    for item in &items {
        assert_eq!(
            item.estimated_cost,
            item.total_cost + plain(&item.node),
            "f for {:?} did not come from the queue",
            item.node
        );
    }
    // The start node included: it used to be queued with an estimate of zero.
    assert_eq!(items[0].node, (0, 0));
    assert_eq!(items[0].estimated_cost, plain(&(0, 0)));
}
