#[macro_use]
extern crate lazy_static;
extern crate pathfinding;
extern crate rand;

use pathfinding::prelude::{astar, idastar};
use rand::os::OsRng;
use rand::Rng;
use std::time::Instant;

#[cfg(test)]
const SIDE: u8 = 3;
#[cfg(not(test))]
const SIDE: u8 = 4;
const LIMIT: usize = (SIDE * SIDE) as usize;

macro_rules! absdiff {
    ($a:expr, $b:expr) => { if $a > $b { $a - $b } else { $b - $a } }
}

#[cfg_attr(feature = "cargo-clippy", allow(derive_hash_xor_eq))]
#[derive(Clone, Debug, Hash)]
struct Game {
    positions: [u8; LIMIT], // Correct position of piece at every index
    hole_idx: u8,           // Current index of the hole
    weight: u8,             // Current some of pieces Manhattan distances
}

impl PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        self.hole_idx == other.hole_idx && self.weight == other.weight
            && self.positions == other.positions
    }
}

impl Eq for Game {}

lazy_static! {
    static ref GOAL: Game = Game {
        positions: {
            let mut p = [0u8; LIMIT];
            for (i, e) in p.iter_mut().enumerate() {
                *e = i as u8;
            }
            p
        },
        hole_idx: 0,
        weight: 0,
    };

    static ref NEIGHBOURS: Vec<Vec<u8>> = (0..SIDE*SIDE).map(|idx|
        (0..4).filter_map(|dir|
                          match dir {
                              0 if idx % SIDE > 0 => Some(idx - 1),
                              1 if idx >= SIDE => Some(idx - SIDE),
                              2 if idx % SIDE < SIDE - 1 => Some(idx + 1),
                              3 if idx < SIDE * SIDE - SIDE => Some(idx + SIDE),
                              _ => None,
                          }).collect::<Vec<_>>()).collect();
}

impl Game {
    /// Move the hole to the given index.
    fn switch(&self, idx: u8) -> Game {
        let mut g = self.clone();
        g.positions.swap(self.hole_idx as usize, idx as usize);
        g.hole_idx = idx;
        g.weight = g.weight
            + g.distance(self.hole_idx) // Distance of the moved piece at its new index
            - self.distance(idx); // Distance of the moved piece at its previous index
        g
    }

    #[inline]
    fn x(pos: u8) -> u8 {
        pos % SIDE
    }

    #[inline]
    fn y(pos: u8) -> u8 {
        pos / SIDE
    }

    // Compute the Manhattan distance between the piece at idx and its correct position.
    fn distance(&self, idx: u8) -> u8 {
        let (actual_x, actual_y) = (Self::x(idx), Self::y(idx));
        let (correct_x, correct_y) = (
            Self::x(self.positions[idx as usize]),
            Self::y(self.positions[idx as usize]),
        );
        absdiff!(actual_x, correct_x) + absdiff!(actual_y, correct_y)
    }

    fn solved(&self) -> bool {
        self.positions == GOAL.positions
    }

    fn neighbours(&self) -> Vec<(Game, u8)> {
        NEIGHBOURS[self.hole_idx as usize]
            .iter()
            .map(|&n| (self.switch(n), 1))
            .collect()
    }

    fn is_solvable(&self) -> bool {
        let mut inversions = 0;
        for i in 0..LIMIT {
            let c = self.positions[i];
            if c != 0 {
                for j in i + 1..LIMIT {
                    let d = self.positions[j];
                    if d != 0 && d < c {
                        inversions ^= 1
                    }
                }
            }
        }
        if SIDE % 2 == 1 {
            inversions == 0
        } else {
            Self::y(self.hole_idx) % 2 == inversions
        }
    }

    fn from_array(positions: [u8; LIMIT]) -> Game {
        let hole_idx = positions
            .iter()
            .enumerate()
            .find(|&(_, &n)| n == 0)
            .unwrap()
            .0 as u8;
        let mut game = Game {
            positions,
            hole_idx,
            weight: 0,
        };
        game.weight = (0..LIMIT as u8)
            .filter(|&n| n != game.hole_idx)
            .map(|n| game.distance(n))
            .sum();
        game
    }

    fn shuffled() -> Game {
        let mut rng = OsRng::new().unwrap();
        loop {
            let mut positions = Self::default().positions;
            rng.shuffle(&mut positions);
            let game = Self::from_array(positions);
            if game.is_solvable() {
                return game;
            }
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        GOAL.clone()
    }
}

#[test]
fn test() {
    // main() already contains checks.
    main();
}

fn main() {
    let b = Game::shuffled();
    println!("{:?}", b);
    assert!(b.is_solvable());
    let idastar_result = {
        let before = Instant::now();
        let result = idastar(&b, |b| b.neighbours(), |b| b.weight, |b| b.solved()).unwrap();
        let elapsed = before.elapsed();
        println!(
            "idastar: {} moves in {}.{:03} seconds",
            result.1,
            elapsed.as_secs(),
            elapsed.subsec_nanos() / 1_000_000
        );
        result
    };
    let astar_result = {
        let before = Instant::now();
        let result = astar(&b, |b| b.neighbours(), |b| b.weight, |b| b.solved()).unwrap();
        let elapsed = before.elapsed();
        println!(
            "astar: {} moves in {}.{:03} seconds",
            result.1,
            elapsed.as_secs(),
            elapsed.subsec_nanos() / 1_000_000
        );
        result
    };
    assert_eq!(idastar_result.1, astar_result.1);
    assert!(idastar_result.1 >= b.weight);
}
