#![feature(test)]

extern crate pathfinding;
extern crate test;

use pathfinding::{bfs, dijkstra};
use test::Bencher;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pt {
    x: u16,
    y: u16,
    _fill: [u64; 32],
}
impl Pt {
    fn new(x: u16, y: u16) -> Pt {
        Pt {
            x: x,
            y: y,
            _fill: [0u64; 32],
        }
    }
}

#[inline]
fn neighbours(pt: &Pt) -> Vec<Pt> {
    let mut ret = vec![];
    if 0 < pt.x {
        ret.push(Pt::new(pt.x - 1, pt.y))
    }
    if pt.x < 128 {
        ret.push(Pt::new(pt.x + 1, pt.y))
    }
    if 0 < pt.y {
        ret.push(Pt::new(pt.x, pt.y - 1))
    }
    if pt.y < 128 {
        ret.push(Pt::new(pt.x, pt.y + 1))
    }
    ret
}

#[bench]
fn no_path_bfs(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(bfs(&Pt::new(2, 3), |n| neighbours(n), |_| false), None)
    });
}

#[bench]
fn no_path_dijkstra(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(
            dijkstra(
                &Pt::new(2, 3),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |_| false,
            ),
            None
        )
    });
}
