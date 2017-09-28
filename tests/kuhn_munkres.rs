#![cfg(feature = "kuhn_munkres")]

#[macro_use]
extern crate ndarray;
extern crate pathfinding;

use pathfinding::*;

#[test]
fn tryalgo_examples() {
    // Some tests from https://github.com/jilljenn/tryalgo/blob/master/tests/test_tryalgo.py
    assert_eq!(kuhn_munkres(&array![[1]]), (1, vec![0]));
    assert_eq!(kuhn_munkres(&array![[1, 1], [1, 1]]).0, 2);
    assert_eq!(kuhn_munkres(&array![[1, 2], [1, 1]]), (3, vec![1, 0]));
    assert_eq!(kuhn_munkres(&array![[1, 1], [2, 1]]), (3, vec![1, 0]));
    assert_eq!(kuhn_munkres(&array![[2, 1], [1, 1]]), (3, vec![0, 1]));
    assert_eq!(kuhn_munkres(&array![[1, 1], [1, 2]]), (3, vec![0, 1]));
    assert_eq!(kuhn_munkres(&array![[-1, -2, -3], [-6, -5, -4], [-1, -1, -1]]), (-6, vec![0, 2, 1]));
    assert_eq!(kuhn_munkres(&array![[1, 2, 3], [6, 5, 4], [1, 1, 1]]), (
        10,
        vec![
            2,
            0,
            1,
        ],
    ));
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
