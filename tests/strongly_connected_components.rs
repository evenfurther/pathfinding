use lazy_static::lazy_static;
use pathfinding::directed::strongly_connected_components::*;
use std::collections::hash_map::HashMap;

// Tests in this file use the example at
// https://en.wikipedia.org/wiki/Strongly_connected_component#/media/File:Graph_Condensation.svg

#[allow(clippy::trivially_copy_pass_by_ref)]
fn successors(n: &usize) -> Vec<usize> {
    match *n {
        0 => vec![2],
        1 => vec![0, 5],
        2 => vec![1, 4],
        3 => vec![2, 9],
        4 => vec![3, 5, 10],
        5 => vec![6, 8, 13],
        6 => vec![7],
        7 => vec![8, 15],
        8 => vec![6, 15],
        9 => vec![10],
        10 => vec![11, 12],
        11 => vec![9],
        12 => vec![11, 13],
        13 => vec![14, 15],
        14 => vec![13],
        15 => vec![],
        _ => panic!("error"),
    }
}

lazy_static! {
    static ref EXPECTED: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 3, 4],
        vec![5],
        vec![6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14],
        vec![15],
    ];
    static ref SCC: HashMap<usize, Vec<usize>> = EXPECTED
        .clone()
        .into_iter()
        .flat_map(|v| v.clone().into_iter().map(move |n| (n, v.clone())))
        .collect();
}

#[test]
fn empty_scc() {
    let c = strongly_connected_components(&[], successors);
    assert_eq!(c.len(), 0);
}

#[test]
fn single_scc() {
    let c = strongly_connected_components(&[42], |_| vec![]);
    assert_eq!(c, vec![vec![42]]);
    let s = strongly_connected_component(&42, |_| vec![]);
    assert_eq!(s, vec![42]);
}

#[test]
fn all_scc() {
    let mut c = strongly_connected_components(&(0..15).collect::<Vec<_>>(), successors)
        .into_iter()
        .map(|mut v| {
            v.sort_unstable();
            v
        })
        .collect::<Vec<_>>();
    c.sort();
    assert_eq!(c, *EXPECTED);
}

#[test]
fn some_scc() {
    fn starting_from(start: usize) -> Vec<usize> {
        let mut c = strongly_connected_components_from(&start, successors)
            .into_iter()
            .map(|mut v| {
                v.sort_unstable();
                v
            })
            .collect::<Vec<_>>();
        c.sort();
        // Check that clusters are indeed valid ones
        for v in &c {
            assert!(EXPECTED.contains(v));
        }
        // Return the first element of each cluster
        c.into_iter().map(|v| v[0]).collect()
    }
    for &i in &[0, 1, 2, 3, 4] {
        assert_eq!(starting_from(i), vec![0, 5, 6, 9, 13, 15]);
    }
    assert_eq!(starting_from(5), vec![5, 6, 13, 15]);
    for &i in &[6, 7, 8] {
        assert_eq!(starting_from(i), vec![6, 15]);
    }
    for &i in &[9, 10, 11, 12] {
        assert_eq!(starting_from(i), vec![9, 13, 15]);
    }
    for &i in &[13, 14] {
        assert_eq!(starting_from(i), vec![13, 15]);
    }
    assert_eq!(starting_from(15), vec![15]);
}

#[test]
fn individual_scc() {
    for n in 0..=15 {
        let mut c = strongly_connected_component(&n, successors);
        c.sort_unstable();
        assert_eq!(c, SCC[&n]);
    }
}

#[test]
fn loops() {
    let mut c = strongly_connected_components(&[0], |_| vec![42]);
    c.sort();
    assert_eq!(c, vec![vec![0], vec![42]]);
}
