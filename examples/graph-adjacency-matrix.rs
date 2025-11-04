/// Example demonstrating how to use pathfinding with an adjacency matrix graph representation.
/// This example shows using A* algorithm with a simple heuristic.
use pathfinding::prelude::astar;

fn main() {
    // Node names for display
    const NODE_NAMES: [char; 5] = ['A', 'B', 'C', 'D', 'E'];

    // Represent nodes as indices (0=A, 1=B, 2=C, 3=D, 4=E)
    // None means no edge, Some(weight) means edge with that weight
    let adjacency_matrix: Vec<Vec<Option<u32>>> = vec![
        vec![None, Some(4), Some(2), None, None],  // Node 0 (A)
        vec![None, None, Some(1), Some(5), None],  // Node 1 (B)
        vec![None, None, None, Some(8), Some(10)], // Node 2 (C)
        vec![None, None, None, None, Some(2)],     // Node 3 (D)
        vec![None, None, None, None, None],        // Node 4 (E)
    ];

    let num_nodes = adjacency_matrix.len();

    // Successor function: returns neighbors and their costs
    let successors = |&node: &usize| -> Vec<(usize, u32)> {
        (0..num_nodes)
            .filter_map(|neighbor| {
                adjacency_matrix[node][neighbor].map(|weight| (neighbor, weight))
            })
            .collect()
    };

    // Simple heuristic: distance to goal
    // In a real application, this should be admissible (never overestimate)
    let heuristic = |&node: &usize| -> u32 {
        // Simple heuristic: 0 if at goal, 1 otherwise
        u32::from(node != 4)
    };

    // Find path from node 0 (A) to node 4 (E) using A*
    let result = astar(&0, successors, heuristic, |&node| node == 4);

    match result {
        Some((path, cost)) => {
            let path_names: Vec<char> = path.iter().map(|&i| NODE_NAMES[i]).collect();

            println!("Shortest path from A to E using A*:");
            println!("  Path (indices): {path:?}");
            println!("  Path (names): {path_names:?}");
            println!("  Total cost: {cost}");
            // The shortest path is A (0) -> B (1) -> D (3) -> E (4) with cost 11
            assert_eq!(path, vec![0, 1, 3, 4]);
            assert_eq!(cost, 11);
        }
        None => println!("No path found"),
    }

    // Example 2: Find path from B (1) to E (4)
    let result2 = astar(&1, successors, heuristic, |&node| node == 4);

    match result2 {
        Some((path, cost)) => {
            let path_names: Vec<char> = path.iter().map(|&i| NODE_NAMES[i]).collect();

            println!("\nShortest path from B to E:");
            println!("  Path (indices): {path:?}");
            println!("  Path (names): {path_names:?}");
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
    fn test_adjacency_matrix_example() {
        main();
    }
}
