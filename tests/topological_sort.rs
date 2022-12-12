use itertools::Itertools;
use pathfinding::directed::topological_sort::topological_sort as tsort;
use pathfinding::directed::topological_sort::topological_sort_into_groups;
use rand::prelude::SliceRandom;
use rand::rngs;

#[test]
fn empty() {
    let empty: Vec<usize> = vec![];
    assert_eq!(tsort(&empty, |&n| vec![n]), Ok(empty));
}

#[test]
fn order() {
    // Shuffle integers from 1 to 1000, and order them so that divisors
    // are located before the numbers they divide.
    let mut rng = rngs::OsRng;
    let mut ints = (1..1000).collect_vec();
    ints.shuffle(&mut rng);
    let sorted = tsort(&ints, |&n| {
        (2..).map(|m| m * n).take_while(|&p| p < 1000).collect_vec()
    })
    .unwrap();
    for (i, &vi) in sorted.iter().enumerate() {
        for &vj in sorted.iter().skip(i + 1) {
            assert!(vi % vj != 0, "{vj} is located after {vi} and divides it");
        }
    }
}

#[test]
fn complexity() {
    // To ensure that the sort is O(|E| + |V|), we ensure that the
    // successors for a particular node are requested exactly one time.
    let mut rng = rngs::OsRng;
    let mut ints = (1..1000).collect_vec();
    ints.shuffle(&mut rng);
    let mut requested = 0;
    let result = tsort(&ints, |&n| {
        requested += 1;
        if n < 999 {
            vec![n + 1]
        } else {
            vec![]
        }
    });
    assert_eq!(result, Ok((1..1000).collect_vec()));
    assert_eq!(requested, 999);
}

// Wrapper around topological_sort_into_groups that sorts each group (since
// topological_sort_into_groups makes no guarantees about node order within
// each group).
#[allow(clippy::type_complexity)]
fn tsig(succs: &[&[usize]]) -> Result<Vec<Vec<usize>>, (Vec<Vec<usize>>, Vec<usize>)> {
    let nodes: Vec<usize> = (0..succs.len()).collect();
    match topological_sort_into_groups(&nodes, |&n| succs[n].iter().cloned()) {
        Ok(mut groups) => {
            for group in groups.iter_mut() {
                group.sort_unstable();
            }
            Ok(groups)
        }
        Err((mut groups, mut remaining)) => {
            for group in groups.iter_mut() {
                group.sort_unstable();
            }
            remaining.sort_unstable();
            Err((groups, remaining))
        }
    }
}

#[test]
fn tsig_empty_graph() {
    assert_eq!(tsig(&[]), Ok(vec![]));
}

#[test]
fn tsig_graph_with_no_edges() {
    assert_eq!(tsig(&[&[], &[], &[]]), Ok(vec![vec![0, 1, 2]]));
}

#[test]
fn tsig_diamond() {
    assert_eq!(
        tsig(&[&[1, 2], &[3], &[3], &[]]),
        Ok(vec![vec![0], vec![1, 2], vec![3]])
    );
}

#[test]
fn tsig_multiple_layers() {
    let succs: &[&[usize]] = &[&[1, 5], &[2], &[3], &[], &[5], &[3]];
    assert_eq!(
        tsig(succs),
        Ok(vec![vec![0, 4], vec![1, 5], vec![2], vec![3]])
    );
}

#[test]
fn tsig_nothing_but_a_cycle() {
    assert_eq!(tsig(&[&[1], &[2], &[0]]), Err((vec![], vec![0, 1, 2])));
}

#[test]
fn tsig_chain_then_cycle() {
    assert_eq!(
        tsig(&[&[1], &[2], &[3], &[2, 4], &[]]),
        Err((vec![vec![0], vec![1]], vec![2, 3, 4]))
    );
}

#[test]
fn tsig_self_edge() {
    assert_eq!(
        tsig(&[&[1, 2], &[3], &[3], &[3]]),
        Err((vec![vec![0], vec![1, 2]], vec![3]))
    );
}
