# Working with Graphs in Pathfinding

This guide explains how to use the pathfinding library with traditional graph structures consisting of nodes, edges, and weights.

## Overview

Unlike some graph libraries that provide predefined graph data structures, the `pathfinding` library takes a **functional approach**. Instead of requiring you to use a specific graph type, the algorithms accept a **successor function** that defines how to navigate from one node to its neighbors.

This flexible design allows you to use any graph representation you prefer:
- Adjacency lists
- Adjacency matrices
- Edge lists
- Custom data structures
- Even implicit graphs (where edges are computed on-the-fly)

## Core Concept: The Successor Function

All pathfinding algorithms in this library require a **successor function**. This function:
- Takes a node as input
- Returns an iterator/collection of neighboring nodes
- For weighted algorithms (Dijkstra, A*), returns `(neighbor_node, cost)` pairs
- For unweighted algorithms (BFS, DFS), returns just neighbor nodes
- Defines the structure of your graph implicitly

```rust
// For weighted graphs (Dijkstra, A*, Fringe, etc.)
fn successors(node: &Node) -> impl IntoIterator<Item = (Node, Cost)>

// For unweighted graphs (BFS, DFS, etc.)
fn successors(node: &Node) -> impl IntoIterator<Item = Node>
```

## Graph Representations

### 1. Adjacency List

An adjacency list stores each node's neighbors and edge weights. This is efficient for sparse graphs.

```rust
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

// Define our graph as an adjacency list: Node -> Vec<(Neighbor, Weight)>
type Graph = HashMap<&'static str, Vec<(&'static str, u32)>>;

fn main() {
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
    let successors = |node: &&str| -> Vec<(&str, u32)> {
        graph.get(node).cloned().unwrap_or_default()
    };

    // Find shortest path from A to E
    let result = dijkstra(&"A", successors, |&node| node == "E");

    match result {
        Some((path, cost)) => {
            println!("Path: {:?}", path);
            println!("Total cost: {}", cost);
        }
        None => println!("No path found"),
    }
}
```

### 2. Adjacency Matrix

An adjacency matrix is a 2D array where `matrix[i][j]` represents the edge weight from node `i` to node `j`. Useful for dense graphs.

```rust
use pathfinding::prelude::astar;

fn main() {
    // Represent nodes as indices (0, 1, 2, 3, 4)
    // None means no edge, Some(weight) means edge with weight
    let adjacency_matrix: Vec<Vec<Option<u32>>> = vec![
        vec![None,    Some(4), Some(2), None,    None],    // Node 0 (A)
        vec![None,    None,    Some(1), Some(5), None],    // Node 1 (B)
        vec![None,    None,    None,    Some(8), Some(10)], // Node 2 (C)
        vec![None,    None,    None,    None,    Some(2)],  // Node 3 (D)
        vec![None,    None,    None,    None,    None],     // Node 4 (E)
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
    let heuristic = |&node: &usize| -> u32 {
        if node == 4 { 0 } else { 1 }
    };

    // Find path from node 0 to node 4 using A*
    let result = astar(
        &0,
        successors,
        heuristic,
        |&node| node == 4,
    );

    match result {
        Some((path, cost)) => {
            println!("Path: {:?}", path);
            println!("Total cost: {}", cost);
        }
        None => println!("No path found"),
    }
}
```

### 3. Edge List with Lookup

An edge list stores all edges as tuples. You can convert it to an adjacency list for efficient lookups.

```rust
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    id: u32,
}

struct Edge {
    from: u32,
    to: u32,
    weight: u32,
}

fn main() {
    // Define edges
    let edges = vec![
        Edge { from: 1, to: 2, weight: 7 },
        Edge { from: 1, to: 3, weight: 9 },
        Edge { from: 1, to: 6, weight: 14 },
        Edge { from: 2, to: 3, weight: 10 },
        Edge { from: 2, to: 4, weight: 15 },
        Edge { from: 3, to: 4, weight: 11 },
        Edge { from: 3, to: 6, weight: 2 },
        Edge { from: 4, to: 5, weight: 6 },
        Edge { from: 5, to: 6, weight: 9 },
    ];

    // Build adjacency list from edge list
    let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for edge in edges {
        graph.entry(edge.from).or_default().push((edge.to, edge.weight));
    }

    // Successor function
    let successors = |node_id: &u32| -> Vec<(u32, u32)> {
        graph.get(node_id).cloned().unwrap_or_default()
    };

    // Find shortest path
    let result = dijkstra(&1, successors, |&node| node == 5);

    match result {
        Some((path, cost)) => {
            println!("Path: {:?}", path);
            println!("Total cost: {}", cost);
        }
        None => println!("No path found"),
    }
}
```

### 4. Struct-Based Graph

You can encapsulate the graph logic in a struct with methods.

```rust
use pathfinding::prelude::{astar, dijkstra};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct City {
    name: String,
}

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
            // Note: In production code, consider using proper rounding
            // and handling potential truncation explicitly
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

fn main() {
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
    if let Some((path, cost)) = network.find_path_dijkstra("CityA", "CityD") {
        println!("Dijkstra - Path: {:?}, Cost: {}", path, cost);
    }

    // Find path using A*
    if let Some((path, cost)) = network.find_path_astar("CityA", "CityD") {
        println!("A* - Path: {:?}, Cost: {}", path, cost);
    }
}
```

### 5. Unweighted Graphs (for BFS/DFS)

For unweighted graphs where all edges have the same cost, use algorithms like BFS or DFS. The successor function returns just nodes without costs.

```rust
use pathfinding::prelude::bfs;
use std::collections::HashMap;

fn main() {
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
    let successors = |node: &&str| -> Vec<&str> {
        graph.get(node).cloned().unwrap_or_default()
    };

    // Find shortest path from A to F using BFS
    let result = bfs(&"A", successors, |&node| node == "F");

    match result {
        Some(path) => {
            println!("Shortest path from A to F: {:?}", path);
            println!("Number of hops: {}", path.len() - 1);
            // Expected: ["A", "C", "F"] with 2 hops
        }
        None => println!("No path found"),
    }
}
```

This is particularly useful for:
- Social network analysis (finding shortest connection between people)
- Maze solving (where each step has the same cost)
- Web crawling (where you want to find pages within a certain number of clicks)
- Game state exploration (finding shortest sequence of moves)

## Choosing the Right Algorithm

### Dijkstra's Algorithm

Use when:
- You need the shortest path in a weighted graph
- All edge weights are non-negative
- You don't have a good heuristic for the goal

```rust
use pathfinding::prelude::dijkstra;

let result = dijkstra(
    &start_node,
    |node| get_neighbors(node),  // Returns Vec<(Neighbor, Cost)>
    |node| node == goal_node,
);
```

### A* Algorithm

Use when:
- You need the shortest path in a weighted graph
- You have a good admissible heuristic (underestimates true cost)
- You want faster performance than Dijkstra

```rust
use pathfinding::prelude::astar;

let result = astar(
    &start_node,
    |node| get_neighbors(node),      // Returns Vec<(Neighbor, Cost)>
    |node| estimate_cost_to_goal(node), // Heuristic function
    |node| node == goal_node,
);
```

### BFS (Breadth-First Search)

Use when:
- All edges have the same cost (unweighted graph)
- You need the shortest path by number of hops

```rust
use pathfinding::prelude::bfs;

let result = bfs(
    &start_node,
    |node| get_neighbors(node),  // Returns Vec<Neighbor> (no costs)
    |node| node == goal_node,
);
```

## Practical Example: Spatial Shortest Paths

This example demonstrates finding shortest paths on a spatial graph (like in GIS or mapping applications).

```rust
use pathfinding::prelude::astar;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Location {
    id: u32,
    x: f64,
    y: f64,
}

impl Location {
    fn distance_to(&self, other: &Location) -> u32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx + dy * dy).sqrt() * 100.0) as u32 // Scale for integer costs
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
        self.locations.insert(id, Location { id, x, y });
    }

    fn add_edge(&mut self, from: u32, to: u32, cost: u32) {
        self.edges.entry(from).or_default().push((to, cost));
    }

    fn find_shortest_path(&self, start_id: u32, goal_id: u32) -> Option<(Vec<u32>, u32)> {
        let goal_location = self.locations.get(&goal_id)?;

        astar(
            &start_id,
            |&node_id| {
                self.edges
                    .get(&node_id)
                    .cloned()
                    .unwrap_or_default()
            },
            |&node_id| {
                // Heuristic: straight-line distance to goal
                self.locations
                    .get(&node_id)
                    .map(|loc| loc.distance_to(goal_location))
                    .unwrap_or(u32::MAX)
            },
            |&node_id| node_id == goal_id,
        )
    }
}

fn main() {
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
    if let Some((path, cost)) = graph.find_shortest_path(1, 3) {
        println!("Shortest path: {:?}", path);
        println!("Total cost: {}", cost);
    } else {
        println!("No path found");
    }
}
```

## Converting from Other Languages

If you're coming from R or other languages with explicit graph structures:

### From R (igraph)

In R with `igraph`:
```r
library(igraph)
g <- graph_from_data_frame(edges_df)
shortest_paths(g, from, to)
```

In Rust with `pathfinding`:
```rust
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

// Convert your R edge list to adjacency list
let mut graph: HashMap<String, Vec<(String, u32)>> = HashMap::new();
for (from, to, weight) in edges {
    graph.entry(from).or_default().push((to, weight));
}

// Use dijkstra
let result = dijkstra(
    &start_node,
    |node| graph.get(node).cloned().unwrap_or_default(),
    |node| node == &goal_node,
);
```

### From Python (NetworkX)

In Python with `networkx`:
```python
import networkx as nx
G = nx.Graph()
G.add_weighted_edges_from(edges)
path = nx.shortest_path(G, source, target, weight='weight')
```

In Rust with `pathfinding`:
```rust
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
for (from, to, weight) in edges {
    graph.entry(from).or_default().push((to, weight));
}

let result = dijkstra(
    &source,
    |node| graph.get(node).cloned().unwrap_or_default(),
    |node| *node == target,
);
```

## Tips and Best Practices

1. **Node Types**: Your nodes can be any type that implements `Eq`, `Hash`, and `Clone`. Common choices:
   - Integers (`u32`, `usize`)
   - Strings (`String`, `&str`)
   - Custom structs
   - Tuples (e.g., `(i32, i32)` for grid coordinates)

2. **Cost Types**: Costs must implement `Zero`, `Ord`, and `Copy`. Common choices:
   - Unsigned integers (`u32`, `usize`)
   - Signed integers (`i32`)
   - For floating-point, use `ordered_float` crate

3. **Heuristic Functions**: For A*, ensure your heuristic is admissible (never overestimates the true cost)

4. **Memory Efficiency**: The successor function is called on-demand, so you can:
   - Generate successors dynamically
   - Use lazy evaluation
   - Avoid storing the entire graph in memory if it's very large

5. **Bidirectional Graphs**: If your graph is undirected or you need bidirectional edges, add edges in both directions:
   ```rust
   graph.entry(a).or_default().push((b, cost));
   graph.entry(b).or_default().push((a, cost));
   ```

## Further Reading

- [API Documentation](https://docs.rs/pathfinding/)
- [Algorithm descriptions in lib.rs](/src/lib.rs)
- [Examples directory](/examples/)
- Wikipedia articles on graph algorithms (linked in the API docs)

## Common Patterns

### Pattern 1: Checking if a path exists

```rust
let path_exists = dijkstra(&start, successors, |&n| n == goal).is_some();
```

### Pattern 2: Finding all shortest paths

```rust
use pathfinding::prelude::dijkstra_all;

let result = dijkstra_all(&start, successors);
// result contains all reachable nodes and their costs from start
```

### Pattern 3: Multiple goal checking

```rust
let goals = vec![goal1, goal2, goal3];
let result = dijkstra(&start, successors, |node| goals.contains(node));
```

### Pattern 4: Visiting all nodes within a cost budget

```rust
use pathfinding::prelude::dijkstra_all;

let reachable = dijkstra_all(&start, successors);
let within_budget: Vec<_> = reachable
    .into_iter()
    .filter(|(_, cost)| *cost <= budget)
    .collect();
```

## Conclusion

The `pathfinding` library's functional approach gives you complete flexibility in how you represent and work with graphs. Whether you prefer adjacency lists, matrices, or custom structures, you can easily adapt them by defining an appropriate successor function. This design makes the library suitable for everything from simple grid-based pathfinding to complex spatial networks.
