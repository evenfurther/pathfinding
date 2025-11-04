/// Example demonstrating how to use pathfinding with unweighted graphs using BFS.
/// In unweighted graphs, the successor function returns just nodes without costs.
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
    let successors = |node: &&str| -> Vec<&str> { graph.get(node).cloned().unwrap_or_default() };

    // Find shortest path from A to F using BFS
    let result = bfs(&"A", successors, |&node| node == "F");

    match result {
        Some(path) => {
            println!("Shortest path from A to F: {path:?}");
            println!("Number of hops: {}", path.len() - 1);
            assert_eq!(path, vec!["A", "C", "F"]);
            assert_eq!(path.len() - 1, 2); // 2 hops
        }
        None => println!("No path found"),
    }

    // Example 2: Find path from A to E
    let result2 = bfs(&"A", successors, |&node| node == "E");

    match result2 {
        Some(path) => {
            println!("\nShortest path from A to E: {path:?}");
            println!("Number of hops: {}", path.len() - 1);
        }
        None => println!("No path found"),
    }

    println!("\nExample completed successfully!");
    println!("\nThis demonstrates BFS on an unweighted graph where:");
    println!("- All edges have equal cost (1 hop)");
    println!("- Successor function returns just nodes, not (node, cost) pairs");
    println!("- BFS finds the path with the fewest hops");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unweighted_bfs_example() {
        main();
    }
}
