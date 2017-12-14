extern crate itertools;
extern crate pathfinding;

use itertools::Itertools;
use pathfinding::*;
use std::usize;

#[test]
fn basic_separate_components() {
    let (h, g) = separate_components(&[vec![1, 2], vec![3, 4], vec![5, 6], vec![1, 4]]);
    assert!([1, 2, 3, 4].into_iter().map(|n| h[n]).all_equal());
    assert_eq!(h[&5], h[&6]);
    assert!(h[&1] != h[&5]);
    assert_eq!(h.len(), 6);
    assert_eq!(g[0], g[1]);
    assert_eq!(g[0], g[3]);
    assert!(g[0] != g[2]);
    assert_eq!(g.len(), 4);
}

#[test]
fn empty_separate_components() {
    let (h, g) = separate_components(&[vec![1, 2], vec![3, 4], vec![], vec![1, 4]]);
    assert!([1, 2, 3, 4].into_iter().map(|n| h[n]).all_equal());
    assert_eq!(h.len(), 4);
    assert_eq!(g[0], g[1]);
    assert_eq!(g[0], g[3]);
    assert!(g[0] != g[2]);
    assert_eq!(g[2], usize::MAX);
    assert_eq!(g.len(), 4);
}

#[test]
fn basic_components() {
    let c = components(&[vec![1, 2], vec![3, 4], vec![5, 6], vec![1, 4, 7]]);
    assert_eq!(c.len(), 2);
    assert_eq!(c[0].clone().into_iter().sorted(), vec![1, 2, 3, 4, 7]);
    assert_eq!(c[1].clone().into_iter().sorted(), vec![5, 6]);
}

#[test]
fn empty_components() {
    let c = components(&[vec![1, 2], vec![3, 4], vec![], vec![1, 4, 7]]);
    assert_eq!(c.len(), 1);
    assert_eq!(c[0].clone().into_iter().sorted(), vec![1, 2, 3, 4, 7]);
}

#[test]
fn basic_connected_components() {
    let mut counter = 0;
    let c = connected_components(&[1, 4], |&n| {
        counter += 1;
        if n % 2 == 0 {
            vec![2, 4, 6, 8]
        } else {
            vec![1, 3, 5, 7]
        }
    });
    assert_eq!(c.len(), 2);
    assert_eq!(c[0].clone().into_iter().sorted(), vec![1, 3, 5, 7]);
    assert_eq!(c[1].clone().into_iter().sorted(), vec![2, 4, 6, 8]);
    assert_eq!(counter, 2);
}
