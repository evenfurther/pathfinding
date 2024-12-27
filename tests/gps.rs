#![cfg(test)]

use pathfinding::prelude::*;
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

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn distance_in_meters(&self, other: &Self) -> u64 {
        let x =
            (other.lon_rad() - self.lon_rad()) * ((other.lat_rad() + self.lat_rad()) / 2.0).cos();
        let y = other.lat_rad() - self.lat_rad();
        (x.hypot(y) * 6_371_000.0).round() as u64
    }
}

fn coords() -> HashMap<&'static str, Coords> {
    vec![
        ("Paris", Coords(48.8567, 2.3508)),
        ("Lyon", Coords(45.76, 4.84)),
        ("Marseille", Coords(43.2964, 5.37)),
        ("Bordeaux", Coords(44.84, -0.58)),
        ("Cannes", Coords(43.5513, 7.0128)),
        ("Toulouse", Coords(43.6045, 1.444)),
        ("Reims", Coords(49.2628, 4.0347)),
    ]
    .into_iter()
    .collect()
}

fn successor_distances(
    coords: &HashMap<&str, Coords>,
) -> HashMap<&'static str, Vec<(&'static str, u64)>> {
    let mut successors = HashMap::new();
    {
        let mut insert_successor = |from: &'static str, to: &'static str| {
            let from_coords = &coords[from];
            let ns = to
                .split(',')
                .map(|successor| {
                    (
                        successor,
                        from_coords.distance_in_meters(&coords[successor]),
                    )
                })
                .collect();
            successors.insert(from, ns);
        };
        insert_successor("Paris", "Lyon,Bordeaux,Reims");
        insert_successor("Lyon", "Paris,Marseille");
        insert_successor("Marseille", "Lyon,Cannes,Toulouse");
        insert_successor("Bordeaux", "Toulouse,Paris");
        insert_successor("Cannes", "Marseille");
        insert_successor("Toulouse", "Marseille,Bordeaux");
        insert_successor("Reims", "Paris");
    }
    successors
}

#[test]
fn gps() {
    let coords = coords();
    let successor_distances = successor_distances(&coords);
    let (start, goal) = ("Paris", "Cannes");
    let goal_coords = &coords[goal];
    let expected_path = vec!["Paris", "Lyon", "Marseille", "Cannes"];

    let r = astar(
        &start,
        |city| successor_distances[city].clone(),
        |city| goal_coords.distance_in_meters(&coords[city]),
        |city| city == &goal,
    );
    let (path, cost_astar) = r.expect("no path found with astar");
    assert_eq!(path, expected_path, "bad path found with astar");

    let r = fringe(
        &start,
        |city| successor_distances[city].clone(),
        |city| goal_coords.distance_in_meters(&coords[city]),
        |city| city == &goal,
    );
    let (path, cost_fringe) = r.expect("no path found with fringe");
    assert_eq!(path, expected_path, "bad path found with fringe");

    assert_eq!(
        cost_astar, cost_fringe,
        "costs for astar and fringe are different"
    );

    let r = dijkstra(
        &start,
        |city| successor_distances[city].clone(),
        |city| city == &goal,
    );
    let (path, cost_dijkstra) = r.expect("no path found with dijkstra");
    assert_eq!(path, expected_path, "bad path found with dijkstra");

    assert_eq!(
        cost_astar, cost_dijkstra,
        "costs for astar and dijkstra are different"
    );

    let r = idastar(
        &start,
        |city| successor_distances[city].clone(),
        |city| goal_coords.distance_in_meters(&coords[city]),
        |city| city == &goal,
    );
    let (path, cost_idastar) = r.expect("no path found with idastar");
    assert_eq!(path, expected_path, "bad path found with idastar");

    assert_eq!(
        cost_astar, cost_idastar,
        "costs for astar and idastar are different"
    );
}
