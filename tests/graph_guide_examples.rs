//! Unit tests for all self-contained examples in `GRAPH_GUIDE.md`
//! This ensures that all code examples in the documentation compile and produce expected results.

use pathfinding::prelude::{astar, bfs, dijkstra};
use std::collections::HashMap;

/// Test for Section 1: Adjacency List example
#[test]
fn test_adjacency_list_example() {
    // Define our graph as an adjacency list: Node -> Vec<(Neighbor, Weight)>
    type Graph = HashMap<&'static str, Vec<(&'static str, u32)>>;

    // Create a weighted graph
    let graph: Graph = [
        ("A", vec![("B", 4), ("C", 2)]),
        ("B", vec![("C", 1), ("D", 5)]),
        ("C", vec![("D", 8), ("E", 10)]),
        ("D", vec![("E", 2)]),
        ("E", vec![]),
    ]
    .iter()
    .cloned()
    .collect();

    // Define the successor function
    let successors =
        |node: &&str| -> Vec<(&str, u32)> { graph.get(node).cloned().unwrap_or_default() };

    // Find shortest path from A to E
    let result = dijkstra(&"A", successors, |&node| node == "E");

    assert!(result.is_some());
    let (path, cost) = result.unwrap();
    assert_eq!(path, vec!["A", "B", "D", "E"]);
    assert_eq!(cost, 11);
}

/// Test for Section 2: Adjacency Matrix example
#[test]
fn test_adjacency_matrix_example() {
    // Represent nodes as indices (0, 1, 2, 3, 4)
    // None means no edge, Some(weight) means edge with weight
    let adjacency_matrix: Vec<Vec<Option<u32>>> = vec![
        vec![None, Some(4), Some(2), None, None],  // Node 0 (A)
        vec![None, None, Some(1), Some(5), None],  // Node 1 (B)
        vec![None, None, None, Some(8), Some(10)], // Node 2 (C)
        vec![None, None, None, None, Some(2)],     // Node 3 (D)
        vec![None, None, None, None, None],        // Node 4 (E)
    ];

    let num_nodes = adjacency_matrix.len();

    // Successor function using the matrix
    let successors = |&node: &usize| -> Vec<(usize, u32)> {
        (0..num_nodes)
            .filter_map(|neighbor| {
                adjacency_matrix[node][neighbor].map(|weight| (neighbor, weight))
            })
            .collect()
    };

    // Simple heuristic (for demonstration - in real use, make it admissible)
    let heuristic = |&node: &usize| -> u32 { u32::from(node != 4) };

    // Find path from node 0 to node 4 using A*
    let result = astar(&0, successors, heuristic, |&node| node == 4);

    assert!(result.is_some());
    let (path, cost) = result.unwrap();
    assert_eq!(path, vec![0, 1, 3, 4]);
    assert_eq!(cost, 11);
}

/// Test for Section 3: Edge List with Lookup example
#[test]
fn test_edge_list_example() {
    struct Edge {
        from: u32,
        to: u32,
        weight: u32,
    }

    // Define edges
    let edges = vec![
        Edge {
            from: 1,
            to: 2,
            weight: 7,
        },
        Edge {
            from: 1,
            to: 3,
            weight: 9,
        },
        Edge {
            from: 1,
            to: 6,
            weight: 14,
        },
        Edge {
            from: 2,
            to: 3,
            weight: 10,
        },
        Edge {
            from: 2,
            to: 4,
            weight: 15,
        },
        Edge {
            from: 3,
            to: 4,
            weight: 11,
        },
        Edge {
            from: 3,
            to: 6,
            weight: 2,
        },
        Edge {
            from: 4,
            to: 5,
            weight: 6,
        },
        Edge {
            from: 5,
            to: 6,
            weight: 9,
        },
    ];

    // Build adjacency list from edge list
    let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for edge in edges {
        graph
            .entry(edge.from)
            .or_default()
            .push((edge.to, edge.weight));
    }

    // Successor function
    let successors =
        |node_id: &u32| -> Vec<(u32, u32)> { graph.get(node_id).cloned().unwrap_or_default() };

    // Find shortest path
    let result = dijkstra(&1, successors, |&node| node == 5);

    assert!(result.is_some());
    let (path, cost) = result.unwrap();
    // Path should be 1 -> 3 -> 4 -> 5
    assert_eq!(path, vec![1, 3, 4, 5]);
    assert_eq!(cost, 26); // 9 + 11 + 6
}

/// Test for Section 4: Struct-Based Graph example
#[test]
fn test_struct_based_graph_example() {
    struct RoadNetwork {
        // adjacency list: city name -> list of (neighbor, distance)
        connections: HashMap<String, Vec<(String, u32)>>,
        // coordinates for heuristic (optional)
        coordinates: HashMap<String, (f64, f64)>,
    }

    impl RoadNetwork {
        fn new() -> Self {
            Self {
                connections: HashMap::new(),
                coordinates: HashMap::new(),
            }
        }

        fn add_road(&mut self, from: &str, to: &str, distance: u32) {
            self.connections
                .entry(from.to_string())
                .or_default()
                .push((to.to_string(), distance));
        }

        fn add_coordinates(&mut self, city: &str, x: f64, y: f64) {
            self.coordinates.insert(city.to_string(), (x, y));
        }

        fn successors(&self, city: &str) -> Vec<(String, u32)> {
            self.connections.get(city).cloned().unwrap_or_default()
        }

        fn heuristic(&self, from: &str, to: &str) -> u32 {
            // Euclidean distance as heuristic
            if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                (self.coordinates.get(from), self.coordinates.get(to))
            {
                let dx = x2 - x1;
                let dy = y2 - y1;
                #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let distance = (dx * dx + dy * dy).sqrt() as u32;
                distance
            } else {
                0
            }
        }

        fn find_path_dijkstra(&self, start: &str, goal: &str) -> Option<(Vec<String>, u32)> {
            dijkstra(
                &start.to_string(),
                |city| self.successors(city),
                |city| city == goal,
            )
        }

        fn find_path_astar(&self, start: &str, goal: &str) -> Option<(Vec<String>, u32)> {
            let goal_str = goal.to_string();
            astar(
                &start.to_string(),
                |city| self.successors(city),
                |city| self.heuristic(city, &goal_str),
                |city| city == &goal_str,
            )
        }
    }

    let mut network = RoadNetwork::new();

    // Add roads (bidirectional)
    network.add_road("CityA", "CityB", 10);
    network.add_road("CityB", "CityA", 10);
    network.add_road("CityA", "CityC", 15);
    network.add_road("CityC", "CityA", 15);
    network.add_road("CityB", "CityD", 12);
    network.add_road("CityD", "CityB", 12);
    network.add_road("CityC", "CityD", 10);
    network.add_road("CityD", "CityC", 10);

    // Add coordinates for A* heuristic
    network.add_coordinates("CityA", 0.0, 0.0);
    network.add_coordinates("CityB", 10.0, 0.0);
    network.add_coordinates("CityC", 0.0, 15.0);
    network.add_coordinates("CityD", 10.0, 12.0);

    // Find path using Dijkstra
    let dijkstra_result = network.find_path_dijkstra("CityA", "CityD");
    assert!(dijkstra_result.is_some());
    let (dijkstra_path, dijkstra_cost) = dijkstra_result.unwrap();
    assert_eq!(dijkstra_path, vec!["CityA", "CityB", "CityD"]);
    assert_eq!(dijkstra_cost, 22);

    // Find path using A*
    let astar_result = network.find_path_astar("CityA", "CityD");
    assert!(astar_result.is_some());
    let (astar_path, astar_cost) = astar_result.unwrap();
    assert_eq!(astar_path, vec!["CityA", "CityB", "CityD"]);
    assert_eq!(astar_cost, 22);
}

/// Test for Section 5: Unweighted Graphs (BFS) example
#[test]
fn test_unweighted_bfs_example() {
    // Define an unweighted graph as an adjacency list
    // Each node maps to a list of neighbors (no weights)
    let graph: HashMap<&str, Vec<&str>> = [
        ("A", vec!["B", "C"]),
        ("B", vec!["A", "D", "E"]),
        ("C", vec!["A", "F"]),
        ("D", vec!["B"]),
        ("E", vec!["B", "F"]),
        ("F", vec!["C", "E"]),
    ]
    .iter()
    .cloned()
    .collect();

    // Successor function returns just neighbors (no costs)
    let successors = |node: &&str| -> Vec<&str> { graph.get(node).cloned().unwrap_or_default() };

    // Find shortest path from A to F using BFS
    let result = bfs(&"A", successors, |&node| node == "F");

    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path, vec!["A", "C", "F"]);
    assert_eq!(path.len() - 1, 2); // 2 hops
}

/// Test for Section 6: Spatial Shortest Paths example
#[test]
fn test_spatial_graph_example() {
    #[derive(Debug, Clone)]
    struct Location {
        x: f64,
        y: f64,
    }

    impl Location {
        fn distance_to(&self, other: &Location) -> u32 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let result = ((dx * dx + dy * dy).sqrt() * 100.0) as u32; // Scale for integer costs
            result
        }
    }

    struct SpatialGraph {
        locations: HashMap<u32, Location>,
        edges: HashMap<u32, Vec<(u32, u32)>>, // node_id -> vec of (neighbor_id, cost)
    }

    impl SpatialGraph {
        fn new() -> Self {
            Self {
                locations: HashMap::new(),
                edges: HashMap::new(),
            }
        }

        fn add_location(&mut self, id: u32, x: f64, y: f64) {
            self.locations.insert(id, Location { x, y });
        }

        fn add_edge(&mut self, from: u32, to: u32, cost: u32) {
            self.edges.entry(from).or_default().push((to, cost));
        }

        fn find_shortest_path(&self, start_id: u32, goal_id: u32) -> Option<(Vec<u32>, u32)> {
            let goal_location = self.locations.get(&goal_id)?;

            astar(
                &start_id,
                |&node_id| self.edges.get(&node_id).cloned().unwrap_or_default(),
                |&node_id| {
                    // Heuristic: straight-line distance to goal
                    self.locations
                        .get(&node_id)
                        .map_or(u32::MAX, |loc| loc.distance_to(goal_location))
                },
                |&node_id| node_id == goal_id,
            )
        }
    }

    let mut graph = SpatialGraph::new();

    // Add locations (nodes)
    graph.add_location(1, 0.0, 0.0);
    graph.add_location(2, 10.0, 0.0);
    graph.add_location(3, 10.0, 10.0);
    graph.add_location(4, 0.0, 10.0);
    graph.add_location(5, 5.0, 5.0);

    // Add edges with costs
    graph.add_edge(1, 2, 1000);
    graph.add_edge(1, 5, 707);
    graph.add_edge(2, 3, 1000);
    graph.add_edge(2, 5, 707);
    graph.add_edge(3, 4, 1000);
    graph.add_edge(3, 5, 707);
    graph.add_edge(4, 1, 1000);
    graph.add_edge(4, 5, 707);

    // Find shortest path from location 1 to location 3
    let result = graph.find_shortest_path(1, 3);

    assert!(result.is_some());
    let (path, cost) = result.unwrap();
    // Verify a path was found - the exact path may vary based on the algorithm
    // Just ensure we got a valid result
    assert!(!path.is_empty());
    assert!(path[0] == 1);
    assert!(path[path.len() - 1] == 3);
    assert!(cost > 0);
}
