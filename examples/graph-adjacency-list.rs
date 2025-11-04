/// Example demonstrating how to use pathfinding with an adjacency list graph representation.
/// This example shows a weighted directed graph and uses Dijkstra's algorithm to find
/// the shortest path.
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

fn main() {
    // Create a weighted graph using adjacency list
    // Each node maps to a list of (neighbor, weight) pairs
    let graph: HashMap<&str, Vec<(&str, u32)>> = [
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

    // Find shortest path from A to E using Dijkstra's algorithm
    let result = dijkstra(&"A", successors, |&node| node == "E");

    match result {
        Some((path, cost)) => {
            println!("Shortest path from A to E:");
            println!("  Path: {path:?}");
            println!("  Total cost: {cost}");
            // The shortest path is A -> B (4) -> D (5) -> E (2) = 11
            assert_eq!(path, vec!["A", "B", "D", "E"]);
            assert_eq!(cost, 11);
        }
        None => println!("No path found"),
    }

    // Find another path: A to D
    let result2 = dijkstra(&"A", successors, |&node| node == "D");

    match result2 {
        Some((path, cost)) => {
            println!("\nShortest path from A to D:");
            println!("  Path: {path:?}");
            println!("  Total cost: {cost}");
        }
        None => println!("No path found"),
    }

    println!("\nExample completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacency_list_example() {
        main();
    }
}
