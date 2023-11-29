use pathfinding::kuhn_munkres::*;
use pathfinding::{matrix, matrix::Matrix};

#[test]
fn tryalgo_examples() {
    // Some tests from https://github.com/jilljenn/tryalgo/blob/master/tests/test_tryalgo.py
    assert_eq!(kuhn_munkres(&matrix![[1]]), (1, vec![0]));
    assert_eq!(kuhn_munkres(&matrix![[1, 1], [1, 1]]).0, 2);
    assert_eq!(kuhn_munkres(&matrix![[1, 2], [1, 1]]), (3, vec![1, 0]));
    assert_eq!(kuhn_munkres(&matrix![[1, 1], [2, 1]]), (3, vec![1, 0]));
    assert_eq!(kuhn_munkres(&matrix![[2, 1], [1, 1]]), (3, vec![0, 1]));
    assert_eq!(kuhn_munkres(&matrix![[1, 1], [1, 2]]), (3, vec![0, 1]));
    assert_eq!(
        kuhn_munkres(&matrix![[-1, -2, -3], [-6, -5, -4], [-1, -1, -1]]),
        (-6, vec![0, 2, 1])
    );
    assert_eq!(
        kuhn_munkres(&matrix![[1, 2, 3], [6, 5, 4], [1, 1, 1]]),
        (10, vec![2, 0, 1])
    );
    assert_eq!(
        kuhn_munkres(&matrix![
            [7, 53, 183, 439, 863],
            [497, 383, 563, 79, 973],
            [287, 63, 343, 169, 583],
            [627, 343, 773, 959, 943],
            [767, 473, 103, 699, 303],
        ])
        .0,
        3315
    );
}

#[test]
fn cranes() {
    // Test from https://s-mat-pcs.oulu.fi/~mpa/matreng/eem1_2-1.htm.
    let distances = matrix![
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
    let weights = matrix![[1, 2, 3], [2, 4, 6], [3, 6, 9],];
    assert_eq!(kuhn_munkres_min(&weights).0, 10);
}

#[test]
fn mattkrick() {
    // Test from https://github.com/mattkrick/hungarian-on3
    let data = matrix![[400, 150, 400], [400, 450, 600], [300, 225, 300]];
    assert_eq!(kuhn_munkres_min(&data).0, 850);
}

#[test]
fn hungarian() {
    // Test from http://www.hungarianalgorithm.com/examplehungarianalgorithm.php
    let weights = matrix![
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
    let data = matrix![
        [62, 78, 50, 101, 82],
        [71, 84, 61, 73, 59],
        [87, 92, 111, 71, 81],
        [48, 64, 87, 77, 80]
    ];
    let (total, assignments) = kuhn_munkres(&data);
    assert_eq!(total, 376);
    assert_eq!(assignments, vec![3, 1, 2, 4]);
}

#[test]
fn empty() {
    let (total, assignments) = kuhn_munkres(&Matrix::<i32>::new_empty(0));
    assert_eq!(total, 0);
    assert_eq!(assignments, vec![]);
}

#[test]
#[should_panic(expected = "number of rows must not be larger than number of columns")]
fn unbalanced() {
    kuhn_munkres(&Matrix::new(3, 2, 0));
}
