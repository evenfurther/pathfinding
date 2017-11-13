extern crate pathfinding;

use pathfinding::*;

macro_rules! array {
    ($a:expr) => {{
        let mut m = SquareMatrix::new($a.len(), 0);
        for i in 0..m.size {
            m[&(0, i)] = $a[i];
        }
        m
    }};
    ($a:expr, $($b: expr),+) => {{
        let mut m = array!($a);
        let mut r = 0;
        $(
            {
                r += 1;
                for i in 0..m.size {
                    m[&(r, i)] = $b[i];
                }
            }
        )+
        m
    }};
    ($a:expr, $($b: expr),+, ) => (array!($a, $($b),+))
}

#[test]
fn tryalgo_examples() {
    // Some tests from https://github.com/jilljenn/tryalgo/blob/master/tests/test_tryalgo.py
    assert_eq!(kuhn_munkres(&array![[1]]), (1, vec![0]));
    assert_eq!(kuhn_munkres(&array![[1, 1], [1, 1]]).0, 2);
    assert_eq!(kuhn_munkres(&array![[1, 2], [1, 1]]), (3, vec![1, 0]));
    assert_eq!(kuhn_munkres(&array![[1, 1], [2, 1]]), (3, vec![1, 0]));
    assert_eq!(kuhn_munkres(&array![[2, 1], [1, 1]]), (3, vec![0, 1]));
    assert_eq!(kuhn_munkres(&array![[1, 1], [1, 2]]), (3, vec![0, 1]));
    assert_eq!(
        kuhn_munkres(&array![[-1, -2, -3], [-6, -5, -4], [-1, -1, -1]]),
        (-6, vec![0, 2, 1])
    );
    assert_eq!(
        kuhn_munkres(&array![[1, 2, 3], [6, 5, 4], [1, 1, 1]]),
        (10, vec![2, 0, 1],)
    );
    assert_eq!(
        kuhn_munkres(&array![
            [7, 53, 183, 439, 863],
            [497, 383, 563, 79, 973],
            [287, 63, 343, 169, 583],
            [627, 343, 773, 959, 943],
            [767, 473, 103, 699, 303],
        ]).0,
        3315
    );
}

#[test]
fn cranes() {
    // Test from https://s-mat-pcs.oulu.fi/~mpa/matreng/eem1_2-1.htm.
    let distances = array![
        [90, 75, 75, 80],
        [35, 85, 55, 65],
        [125, 95, 90, 105],
        [45, 110, 95, 115],
    ];
    assert_eq!(kuhn_munkres_min(&distances).0, 275);
}

#[test]
fn murray() {
    // Test from http://csclab.murraystate.edu/~bob.pilgrim/445/munkres.html
    let weights = array![[1, 2, 3], [2, 4, 6], [3, 6, 9],];
    assert_eq!(kuhn_munkres_min(&weights).0, 10);
}

#[test]
fn mattkrick() {
    // Test from https://github.com/mattkrick/hungarian-on3
    let data = array![[400, 150, 400], [400, 450, 600], [300, 225, 300]];
    assert_eq!(kuhn_munkres_min(&data).0, 850);
}

#[test]
fn hungarian() {
    // Test from http://www.hungarianalgorithm.com/examplehungarianalgorithm.php
    let weights = array![
        [82, 83, 69, 92],
        [77, 37, 49, 92],
        [11, 69, 5, 86],
        [8, 9, 98, 23],
    ];
    assert_eq!(kuhn_munkres_min(&weights).0, 140);
}

#[test]
fn non_square() {
    // Test from https://www.youtube.com/watch?v=aPVtIhnwHPE

    struct A {
        vec: Vec<i32>,
        nx: usize,
        ny: usize,
    }

    impl Weights<i32> for A {
        fn rows(&self) -> usize {
            self.nx
        }
        fn columns(&self) -> usize {
            self.ny
        }
        fn at(&self, row: usize, col: usize) -> i32 {
            self.vec[row * self.ny + col]
        }
        fn neg(&self) -> A {
            A {
                vec: self.vec.iter().map(|n| -*n).collect(),
                nx: self.nx,
                ny: self.ny,
            }
        }
    }

    let data = A {
        vec: vec![
            62,
            78,
            50,
            101,
            82,
            71,
            84,
            61,
            73,
            59,
            87,
            92,
            111,
            71,
            81,
            48,
            64,
            87,
            77,
            80,
        ],
        nx: 4,
        ny: 5,
    };

    let (total, assignments) = kuhn_munkres(&data);
    assert_eq!(total, 376);
    assert_eq!(assignments, vec![3, 1, 2, 4]);
}
