use noisy_float::prelude::{Float, R64, r64};
use pathfinding::prelude::{astar, theta_star};
use rand::{RngExt as _, SeedableRng as _};
use rand_xorshift::XorShiftRng;

const SIDE: i32 = 24;

fn cells() -> usize {
    usize::try_from(SIDE * SIDE).unwrap()
}

/// A square grid with some cells blocked, moving in the eight compass directions.
struct Map {
    blocked: Vec<bool>,
}

impl Map {
    fn random(rng: &mut XorShiftRng, blocked_percent: u32) -> Self {
        let mut blocked = vec![false; cells()];
        for cell in &mut blocked {
            *cell = rng.random_range(0..100u32) < blocked_percent;
        }
        blocked[0] = false;
        let last = cells() - 1;
        blocked[last] = false;
        Self { blocked }
    }

    fn open(&self, (column, row): (i32, i32)) -> bool {
        let Ok(index) = usize::try_from(row * SIDE + column) else {
            return false;
        };
        (0..SIDE).contains(&column) && (0..SIDE).contains(&row) && !self.blocked[index]
    }

    #[expect(clippy::trivially_copy_pass_by_ref)]
    fn successors(&self, &(column, row): &(i32, i32)) -> Vec<((i32, i32), R64)> {
        let mut out = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx, dy) == (0, 0) {
                    continue;
                }
                let next = (column + dx, row + dy);
                if self.open(next) {
                    out.push((next, distance((column, row), next)));
                }
            }
        }
        out
    }

    /// Cost of a straight line, or `None` if it crosses a blocked cell. Every cell the segment
    /// touches is sampled, densely enough that no blocked cell can be stepped over.
    #[expect(clippy::trivially_copy_pass_by_ref)]
    fn sight(&self, &from: &(i32, i32), &to: &(i32, i32)) -> Option<R64> {
        let steps = ((from.0 - to.0).abs().max((from.1 - to.1).abs()) * 4).max(1);
        for step in 0..=steps {
            let along = f64::from(step) / f64::from(steps);
            let column = f64::from(from.0) + along * f64::from(to.0 - from.0);
            let row = f64::from(from.1) + along * f64::from(to.1 - from.1);
            if !self.open((round_to_cell(column), round_to_cell(row))) {
                return None;
            }
        }
        Some(distance(from, to))
    }
}

/// Coordinates are small and the value is a rounded position on the grid, so this cannot
/// truncate in any way that matters.
#[expect(clippy::cast_possible_truncation)]
fn round_to_cell(value: f64) -> i32 {
    value.round() as i32
}

fn distance(from: (i32, i32), to: (i32, i32)) -> R64 {
    r64(f64::from(from.0 - to.0).hypot(f64::from(from.1 - to.1)))
}

#[test]
fn blocked_sight_degenerates_to_astar() {
    // With no line of sight ever available, every shortcut is refused and the search has to
    // behave exactly like `astar`.
    let mut rng = XorShiftRng::from_seed([11; 16]);
    for round in 0..40 {
        let map = Map::random(&mut rng, 25);
        let goal = (SIDE - 1, SIDE - 1);
        let by_astar = astar(
            &(0, 0),
            |n| map.successors(n),
            |&n| distance(n, goal),
            |&n| n == goal,
        );
        let by_theta = theta_star(
            &(0, 0),
            |n| map.successors(n),
            |&n| distance(n, goal),
            |_: &(i32, i32), _: &(i32, i32)| None,
            |&n| n == goal,
        );
        assert_eq!(
            by_astar.as_ref().map(|(p, c)| (p.clone(), *c)),
            by_theta,
            "round {round}"
        );
    }
}

#[test]
fn never_longer_than_astar_and_the_path_is_walkable() {
    let mut rng = XorShiftRng::from_seed([29; 16]);
    let mut shortened = 0;
    for round in 0..60 {
        let map = Map::random(&mut rng, 20);
        let goal = (SIDE - 1, SIDE - 1);
        let Some((_, astar_cost)) = astar(
            &(0, 0),
            |n| map.successors(n),
            |&n| distance(n, goal),
            |&n| n == goal,
        ) else {
            continue;
        };
        let (path, cost) = theta_star(
            &(0, 0),
            |n| map.successors(n),
            |&n| distance(n, goal),
            |a, b| map.sight(a, b),
            |&n| n == goal,
        )
        .unwrap_or_else(|| panic!("round {round}: astar found a path and theta_star did not"));

        assert_eq!(path.first(), Some(&(0, 0)), "round {round}");
        assert_eq!(path.last(), Some(&goal), "round {round}");
        assert!(
            cost <= astar_cost,
            "round {round}: theta_star returned {cost}, longer than astar's {astar_cost}"
        );
        if cost < astar_cost {
            shortened += 1;
        }

        // Every segment must be a clear straight line, and they must add up to the cost.
        let mut total = r64(0.0);
        for step in path.windows(2) {
            let segment = map
                .sight(&step[0], &step[1])
                .unwrap_or_else(|| panic!("round {round}: {:?} cannot see {:?}", step[0], step[1]));
            total += segment;
        }
        assert!(
            (total - cost).abs() < r64(1e-9),
            "round {round}: segments total {total} but cost is {cost}"
        );

        let mut seen = path;
        seen.sort_unstable();
        let len = seen.len();
        seen.dedup();
        assert_eq!(seen.len(), len, "round {round}: path repeats a waypoint");
    }
    // The whole point is that it usually does better than the grid-constrained path.
    assert!(
        shortened > 30,
        "only {shortened} of the paths were shorter than astar's"
    );
}

#[test]
fn open_ground_gives_a_straight_line() {
    // Nothing blocked, so the start can see the goal and the answer is two waypoints.
    let map = Map {
        blocked: vec![false; (SIDE * SIDE) as usize],
    };
    let goal = (SIDE - 1, SIDE - 1);
    let (path, cost) = theta_star(
        &(0, 0),
        |n| map.successors(n),
        |&n| distance(n, goal),
        |a, b| map.sight(a, b),
        |&n| n == goal,
    )
    .expect("no path found");
    assert_eq!(path, vec![(0, 0), goal]);
    assert!((cost - distance((0, 0), goal)).abs() < r64(1e-9));
}

#[test]
fn no_path_when_the_goal_is_walled_off() {
    let mut blocked = vec![false; cells()];
    for row in 0..SIDE {
        blocked[usize::try_from(row * SIDE + SIDE / 2).unwrap()] = true;
    }
    let map = Map { blocked };
    let goal = (SIDE - 1, SIDE - 1);
    assert_eq!(
        theta_star(
            &(0, 0),
            |n| map.successors(n),
            |&n| distance(n, goal),
            |a, b| map.sight(a, b),
            |&n| n == goal,
        ),
        None
    );
}
