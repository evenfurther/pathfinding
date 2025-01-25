//! This example demonstrates the BFS bidirectional algorithm,
//! and compares it with the regular BFS algorithm.

use pathfinding::prelude::{bfs, bfs_bidirectional};
use std::ops::Add;
use std::time::Instant;

const SIZE: isize = 64;
const BOTTOM_LEFT: P = P(0, 0);
const TOP_RIGHT: P = P(SIZE, SIZE);
const CENTER: P = P(SIZE / 2, SIZE / 2);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct P(isize, isize);

impl Add for P {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        P(self.0 + other.0, self.1 + other.1)
    }
}

fn successors(p: &P) -> Vec<P> {
    [P(0, 1), P(0, -1), P(1, 0), P(-1, 0)]
        .into_iter()
        .map(|delta| p.clone() + delta)
        .filter(|p| p.0 >= 0 && p.0 <= SIZE && p.1 >= 0 && p.1 <= SIZE)
        .collect()
}

fn main() {
    run_corner_to_corner();
    run_center_to_corner();
}

/// Corner to corner:
/// =================
///
/// In this case both algorithms will perform similarly.
/// In fact, regular BFS will perform slightly better, since the algorithm is slightly simpler.
///
/// We can understand this in terms of the number of points that need to be searched in order to reach
/// the goal. In the below diagrams this corresponds to the area covered in the final snapshot.
///
/// In both cases every point gets searched - the entire area is filled. For this reason we can intuitively see that
/// regular BFS and bidirectional BFS will perform similarly.
///
/// Regular BFS:
/// ============
///
/// $---------$         $---------$         $---------$         $---------$               $---------$         $---------$
/// |        G|         |        G|         |        G|         |        G|               |FFFFFFF G|         |FFFFFFFFG|
/// |         |         |         |         |         |         |         |               |FFFFFFFF |         |FFFFFFFFF|
/// |         |         |         |         |         |         |         |               |FFFFFFFFF|         |FFFFFFFFF|
/// |         |         |         |         |         |         |         |               |FFFFFFFFF|         |FFFFFFFFF|
/// |         |    =>   |         |    =>   |         |    =>   |         |   => ... =>   |FFFFFFFFF|    =>   |FFFFFFFFF|
/// |         |         |         |         |         |         |F        |               |FFFFFFFFF|         |FFFFFFFFF|
/// |         |         |         |         |F        |         |FF       |               |FFFFFFFFF|         |FFFFFFFFF|
/// |         |         |F        |         |FF       |         |FFF      |               |FFFFFFFFF|         |FFFFFFFFF|
/// |S        |         |SF       |         |SFF      |         |SFFF     |               |SFFFFFFFF|         |SFFFFFFFF|
/// $---------$         $---------$         $---------$         $---------$               $---------$         $---------$
///
/// Bidirectional BFS:
/// ==================
///
/// $---------$         $---------$         $---------$         $---------$               $---------$         $---------$
/// |        G|         |       BG|         |      BBG|         |     BBBG|               | BBBBBBBG|         |FBBBBBBBG|
/// |         |         |        B|         |       BB|         |      BBB|               |F BBBBBBB|         |FFBBBBBBB|
/// |         |         |         |         |        B|         |       BB|               |FF BBBBBB|         |FFFBBBBBB|
/// |         |         |         |         |         |         |        B|               |FFF BBBBB|         |FFFFBBBBB|
/// |         |    =>   |         |    =>   |         |    =>   |         |   => ... =>   |FFFF BBBB|    =>   |FFFFFBBBB|
/// |         |         |         |         |         |         |F        |               |FFFFF BBB|         |FFFFFFBBB|
/// |         |         |         |         |F        |         |FF       |               |FFFFFF BB|         |FFFFFFFBB|
/// |         |         |F        |         |FF       |         |FFF      |               |FFFFFFF B|         |FFFFFFFFB|
/// |S        |         |SF       |         |SFF      |         |SFFF     |               |SFFFFFFF |         |SFFFFFFFF|
/// $---------$         $---------$         $---------$         $---------$               $---------$         $---------$
fn run_corner_to_corner() {
    let instant = Instant::now();
    bfs(&BOTTOM_LEFT, &successors, |p| *p == TOP_RIGHT);
    let duration_bfs = instant.elapsed();

    let instant = Instant::now();
    bfs_bidirectional(&BOTTOM_LEFT, &TOP_RIGHT, successors, successors);
    let duration_bfs_bidirectional = instant.elapsed();

    print!(
        "
Corner to Corner
================
BFS took {duration_bfs:?}
Bidirectional BFS took {duration_bfs_bidirectional:?}
"
    );
}

/// Center to corner:
/// =================
///
/// In this case bidirectional BFS will outperform regular BFS.
///
/// We can understand this in terms of the number of points that need to be searched in order to reach
/// the goal. In the below diagrams this corresponds to the area covered in the final snapshot.
///
/// In this case for the regular BFS every point still needs to be searched - again, the entire area is filled.
/// However, for the bidirectional BFS some points remain unsearched - the entire area is not filled. For this
/// reason we can intuitively see that bidirectional BFS will outperform regular BFS here.
///
/// Regular BFS:
/// ============
///
/// $---------$         $---------$         $---------$               $---------$          $---------$
/// |        G|         |        G|         |        G|               | FFFFFFFG|          |FFFFFFFFG|
/// |         |         |         |         |         |               |FFFFFFFFF|          |FFFFFFFFF|
/// |         |         |    F    |         |    F    |               |FFFFFFFFF|          |FFFFFFFFF|
/// |         |         |   FFF   |         |   FFF   |               |FFFFFFFFF|          |FFFFFFFFF|
/// |    S    |    =>   |  FFSFF  |    =>   |  FFSFF  |   => ... =>   |FFFFSFFFF|    =>    |FFFFSFFFF|
/// |         |         |   FFF   |         |   FFF   |               |FFFFFFFFF|          |FFFFFFFFF|
/// |         |         |    F    |         |    F    |               |FFFFFFFFF|          |FFFFFFFFF|
/// |         |         |         |         |         |               |FFFFFFFFF|          |FFFFFFFFF|
/// |         |         |         |         |         |               | FFFFFFF |          |FFFFFFFFF|
/// $---------$         $---------$         $---------$               $---------$          $---------$
///
/// Bidirectional BFS:
/// ==================
///
/// $---------$         $---------$         $---------$         $---------$         $---------$
/// |        G|         |       BG|         |      BBG|         |     BBBG|         |    FBBBG|
/// |         |         |        B|         |       BB|         |    F BBB|         |   FFFBBB|
/// |         |         |         |         |    F    |         |   FFF  B|         |  FFFFFBB|
/// |         |         |    F    |         |   FFF   |         |  FFFFF  |         | FFFFFFFB|
/// |    S    |    =>   |   FSF   |    =>   |  FFSFF  |    =>   | FFFSFFF |    =>   |FFFFSFFFF|
/// |         |         |    F    |         |   FFF   |         |  FFFFF  |         | FFFFFFF |
/// |         |         |         |         |    F    |         |   FFF   |         |  FFFFF  |
/// |         |         |         |         |         |         |    F    |         |   FFF   |
/// |         |         |         |         |         |         |         |         |    F    |
/// $---------$         $---------$         $---------$         $---------$         $---------$
fn run_center_to_corner() {
    let instant = Instant::now();
    bfs(&CENTER, &successors, |p| *p == TOP_RIGHT);
    let duration_bfs = instant.elapsed();

    let instant = Instant::now();
    bfs_bidirectional(&CENTER, &TOP_RIGHT, successors, successors);
    let duration_bfs_bidirectional = instant.elapsed();

    print!(
        "
Center to Corner
================
BFS took {duration_bfs:?}
Bidirectional BFS took {duration_bfs_bidirectional:?}
"
    );
}
