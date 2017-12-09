#![cfg(test)]

extern crate pathfinding;

use pathfinding::Matrix;

#[test]
fn sm() {
    let mut m = Matrix::new(2, 2, 0usize);
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

#[test]
fn from_vec() {
    let m = Matrix::from_vec(2, 3, vec![10, 20, 30, 40, 50, 60]);
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 3);
    assert!(!m.is_square());
    assert_eq!(m[&(0, 0)], 10);
    assert_eq!(m[&(0, 1)], 20);
    assert_eq!(m[&(0, 2)], 30);
    assert_eq!(m[&(1, 0)], 40);
    assert_eq!(m[&(1, 1)], 50);
    assert_eq!(m[&(1, 2)], 60);
}

#[test]
fn square_from_vec() {
    let m = Matrix::square_from_vec(vec![10, 20, 30, 40]);
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 2);
    assert!(m.is_square());
    assert_eq!(m[&(0, 0)], 10);
    assert_eq!(m[&(0, 1)], 20);
    assert_eq!(m[&(1, 0)], 30);
    assert_eq!(m[&(1, 1)], 40);
}

#[test]
#[should_panic]
fn from_vec_panic() {
    Matrix::from_vec(2, 3, vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn square_from_vec_panic() {
    Matrix::square_from_vec(vec![1, 2, 3]);
}
