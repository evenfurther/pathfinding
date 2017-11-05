#![cfg(test)]

extern crate pathfinding;

use pathfinding::SquareMatrix;

#[test]
fn test_sm() {
    let mut m = SquareMatrix::new(2, 0usize);
    m[&(0, 0)] = 0;
    m[&(0, 1)] = 1;
    m[&(1, 0)] = 10;
    m[&(1, 1)] = 11;
    m[&(0, 1)] = 2;
    assert_eq!(m[&(0, 0)], 0);
    assert_eq!(m[&(0, 1)], 2);
    assert_eq!(m[&(1, 0)], 10);
    assert_eq!(m[&(1, 1)], 11);
    m.fill(33);
    assert_eq!(m[&(0, 0)], 33);
    assert_eq!(m[&(0, 1)], 33);
    assert_eq!(m[&(1, 0)], 33);
    assert_eq!(m[&(1, 1)], 33);
}
