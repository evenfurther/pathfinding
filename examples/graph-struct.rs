/// Example demonstrating how to encapsulate graph logic in a struct.
/// This example shows a road network with both Dijkstra and A* algorithms.
use pathfinding::prelude::{astar, dijkstra};
use std::collections::HashMap;

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

    fn add_bidirectional_road(&mut self, city1: &str, city2: &str, distance: u32) {
        self.add_road(city1, city2, distance);
        self.add_road(city2, city1, distance);
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

fn main() {
    let mut network = RoadNetwork::new();

    // Build a road network
    network.add_bidirectional_road("CityA", "CityB", 10);
    network.add_bidirectional_road("CityA", "CityC", 15);
    network.add_bidirectional_road("CityB", "CityD", 12);
    network.add_bidirectional_road("CityC", "CityD", 10);
    network.add_bidirectional_road("CityB", "CityE", 8);
    network.add_bidirectional_road("CityD", "CityE", 5);

    // Add coordinates for A* heuristic
    network.add_coordinates("CityA", 0.0, 0.0);
    network.add_coordinates("CityB", 10.0, 0.0);
    network.add_coordinates("CityC", 0.0, 15.0);
    network.add_coordinates("CityD", 10.0, 12.0);
    network.add_coordinates("CityE", 15.0, 8.0);

    println!("Road Network Pathfinding Example\n");

    // Find path using Dijkstra
    if let Some((path, cost)) = network.find_path_dijkstra("CityA", "CityE") {
        println!("Dijkstra's Algorithm:");
        println!("  Path from CityA to CityE: {path:?}");
        println!("  Total distance: {cost}");
    }

    // Find path using A*
    if let Some((path, cost)) = network.find_path_astar("CityA", "CityE") {
        println!("\nA* Algorithm:");
        println!("  Path from CityA to CityE: {path:?}");
        println!("  Total distance: {cost}");
    }

    // Another example
    if let Some((path, cost)) = network.find_path_dijkstra("CityA", "CityD") {
        println!("\nDijkstra from CityA to CityD:");
        println!("  Path: {path:?}");
        println!("  Total distance: {cost}");
        assert_eq!(cost, 22); // A -> B (10) -> D (12) = 22
    }

    println!("\nExample completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_struct_example() {
        main();
    }
}
