//! Choosing between nodes of equal estimated cost, from outside the library.

use pathfinding::prelude::{astar, idastar};
use std::cell::Cell;

/// One move, leaving room beneath it for a tie-break that cannot outweigh a real step.
const STEP: u64 = 1024;

const SIDE: u64 = 12;

fn grid(&(x, y): &(u64, u64)) -> Vec<((u64, u64), u64)> {
    [(1, 0), (0, 1)]
        .into_iter()
        .map(move |(dx, dy)| ((x + dx, y + dy), STEP))
        .filter(|&((nx, ny), _)| nx < SIDE && ny < SIDE)
        .collect()
}

const fn distance(&(x, y): &(u64, u64)) -> u64 {
    STEP * ((SIDE - 1 - x) + (SIDE - 1 - y))
}

/// Every monotone route across the grid costs the same, so which one comes back is decided
/// entirely by the tie-break, and a term smaller than a step can only affect that.
#[test]
fn a_term_beneath_one_step_chooses_between_equal_paths() {
    let goal = (SIDE - 1, SIDE - 1);

    let (default_path, cost) = astar(&(0, 0), grid, distance, |&n| n == goal).expect("no path");
    let (left_path, left_cost) =
        astar(&(0, 0), grid, |n| distance(n) + n.0, |&n| n == goal).expect("no path");
    let (top_path, top_cost) =
        astar(&(0, 0), grid, |n| distance(n) + n.1, |&n| n == goal).expect("no path");

    // Same length, because the tie-break never outweighs a step.
    assert_eq!(cost, left_cost);
    assert_eq!(cost, top_cost);
    assert_eq!(cost, STEP * 2 * (SIDE - 1));

    // Preferring a smaller column hugs one edge, a smaller row the other.
    assert_ne!(default_path, left_path);
    assert!(
        left_path[1] == (0, 1),
        "preferring a smaller column should step down first, got {:?}",
        left_path[1]
    );
    assert!(
        top_path[1] == (1, 0),
        "preferring a smaller row should step across first, got {:?}",
        top_path[1]
    );
}

/// The tie-break decides how much of the graph is looked at, which is the reason to care.
#[test]
fn the_tie_break_changes_how_much_is_expanded() {
    let goal = (SIDE - 1, SIDE - 1);

    let count = |heuristic: &dyn Fn(&(u64, u64)) -> u64| {
        let expansions = Cell::new(0u32);
        let counted = |n: &(u64, u64)| {
            expansions.set(expansions.get() + 1);
            grid(n)
        };
        let (_, cost) = astar(&(0, 0), counted, |n| heuristic(n), |&n| n == goal).expect("no path");
        assert_eq!(cost, STEP * 2 * (SIDE - 1), "the path must stay optimal");
        expansions.get()
    };

    // The default prefers the node furthest from the start among equals, which walks straight
    // at the goal. Forcing the opposite preference is admissible and still optimal, but it
    // spreads the search over the whole grid instead.
    let default_expansions = count(&distance);
    let reversed = count(&|n| (distance(n) / STEP) * (STEP - 1));

    assert!(
        reversed > default_expansions * 4,
        "expected the reversed tie-break to expand far more, got {reversed} against \
         {default_expansions}"
    );
}

/// `idastar` explores successors of equal estimated cost in the order they were given, so the
/// caller decides simply by yielding the preferred one first.
#[test]
fn idastar_follows_the_order_successors_are_given_in() {
    // Two routes of identical cost, so their estimates tie the whole way down.
    let routes = |reversed: bool| {
        move |&n: &char| {
            let mut out: Vec<(char, u32)> = match n {
                'S' => vec![('A', 2), ('B', 2)],
                'A' | 'B' => vec![('G', 2)],
                _ => vec![],
            };
            if reversed {
                out.reverse();
            }
            out
        }
    };

    let (through_a, cost) =
        idastar(&'S', routes(false), |_| 0, |&n| n == 'G').expect("no path found");
    let (through_b, reversed_cost) =
        idastar(&'S', routes(true), |_| 0, |&n| n == 'G').expect("no path found");

    assert_eq!(through_a, vec!['S', 'A', 'G']);
    assert_eq!(through_b, vec!['S', 'B', 'G']);
    assert_eq!(cost, reversed_cost);
}
