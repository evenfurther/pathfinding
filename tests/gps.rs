#![cfg(test)]

extern crate pathfinding;

use pathfinding::*;
use std::collections::HashMap;

// Latitude, longitude
struct Coords(f32, f32);

impl Coords {
    fn lat_rad(&self) -> f32 {
        self.0.to_radians()
    }

    fn lon_rad(&self) -> f32 {
        self.1.to_radians()
    }

    fn distance_in_meters(&self, other: &Coords) -> u64 {
        let x = (other.lon_rad() - self.lon_rad()) *
                ((other.lat_rad() + self.lat_rad()) / 2.0).cos();
        let y = other.lat_rad() - self.lat_rad();
        (x.hypot(y) * 6_371_000.0).round() as u64
    }
}

fn coords() -> HashMap<&'static str, Coords> {
    let mut locations = HashMap::new();
    locations.insert("Paris", Coords(48.8567, 2.3508));
    locations.insert("Lyon", Coords(45.76, 4.84));
    locations.insert("Marseille", Coords(43.2964, 5.37));
    locations.insert("Bordeaux", Coords(44.84, -0.58));
    locations.insert("Cannes", Coords(43.5513, 7.0128));
    locations.insert("Toulouse", Coords(43.6045, 1.444));
    locations.insert("Reims", Coords(49.2628, 4.0347));
    locations
}

fn neighbour_distances(coords: &HashMap<&str, Coords>)
                       -> HashMap<&'static str, Vec<(&'static str, u64)>> {
    let mut neighbours = HashMap::new();
    {
        let mut insert_neighbour = |from: &'static str, to: &[&'static str]| {
            let from_coords = &coords[from];
            let ns = to.into_iter()
                .map(|&neighbour| (neighbour, from_coords.distance_in_meters(&coords[neighbour])))
                .collect();
            neighbours.insert(from, ns);
        };
        insert_neighbour("Paris", &["Lyon", "Bordeaux", "Reims"]);
        insert_neighbour("Lyon", &["Paris", "Marseille"]);
        insert_neighbour("Marseille", &["Lyon", "Cannes", "Toulouse"]);
        insert_neighbour("Bordeaux", &["Toulouse", "Paris"]);
        insert_neighbour("Cannes", &["Marseille"]);
        insert_neighbour("Toulouse", &["Marseille", "Bordeaux"]);
        insert_neighbour("Reims", &["Paris"]);
    }
    neighbours
}

#[test]
fn test_gps() {
    let coords = coords();
    let neighbour_distances = neighbour_distances(&coords);
    let start = "Paris";
    let goal = "Cannes";
    let goal_coords = &coords[goal];
    let expected_path = vec!["Paris", "Lyon", "Marseille", "Cannes"];

    let r = astar(&start,
                  |city| neighbour_distances[city].clone(),
                  |city| goal_coords.distance_in_meters(&coords[city]),
                  |city| city == &goal);
    let (path, cost_astar) = r.expect("no path found with astar");
    assert_eq!(path, expected_path, "bad path found with astar");

    let r = fringe(&start,
                   |city| neighbour_distances[city].clone(),
                   |city| goal_coords.distance_in_meters(&coords[city]),
                   |city| city == &goal);
    let (path, cost_fringe) = r.expect("no path found with fringe");
    assert_eq!(path, expected_path, "bad path found with fringe");

    assert_eq!(cost_astar,
               cost_fringe,
               "costs for astar and fringe are different");
}
