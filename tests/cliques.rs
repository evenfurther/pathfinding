use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::*;

#[test]
fn find_cliques() {
    let vertices: Vec<i32> = (1..10).collect_vec();
    let cliques = maximal_cliques_collect(&vertices, &mut |a, b| (*a - *b) % 3 == 0);
    let cliques_as_vectors: Vec<Vec<i32>> = sort(&cliques);

    assert_eq!(
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
        cliques_as_vectors
    );
}

#[test]
fn test_same_node_appears_in_multiple_clique() {
    let vertices: Vec<i32> = (1..10).collect_vec();
    let cliques = maximal_cliques_collect(&vertices, &mut |a, b| {
        (*a % 3 == 0) && (*b % 3 == 0) || ((*a - *b) % 4 == 0)
    });
    let cliques_as_vectors: Vec<Vec<i32>> = sort(&cliques);

    assert_eq!(
        vec![
            vec![1, 5, 9],
            vec![2, 6],
            vec![3, 6, 9],
            vec![3, 7],
            vec![4, 8]
        ],
        cliques_as_vectors
    );
}

fn sort(cliques: &[HashSet<&i32>]) -> Vec<Vec<i32>> {
    let mut cliques_as_vectors: Vec<Vec<i32>> = cliques
        .iter()
        .map(|cliq| {
            let mut s = cliq.iter().map(|&x| *x).collect_vec();
            s.sort_unstable();
            s
        })
        .collect();
    cliques_as_vectors.sort();
    cliques_as_vectors
}
