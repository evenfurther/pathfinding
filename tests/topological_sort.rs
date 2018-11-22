extern crate itertools;
extern crate pathfinding;
extern crate rand;

use itertools::Itertools;
use pathfinding::directed::topological_sort::topological_sort as tsort;
use rand::prelude::SliceRandom;

#[test]
fn empty() {
    let empty: Vec<usize> = vec![];
    assert_eq!(tsort(&empty, |&n| vec![n]), Ok(empty));
}

#[test]
fn order() {
    // Shuffle integers from 1 to 1000, and order them so that divisors
    // are located before the numbers they divide.
    let mut rng = rand::rngs::OsRng::new().unwrap();
    let mut ints = (1..1000).collect_vec();
    ints.shuffle(&mut rng);
    let sorted = tsort(&ints, |&n| {
        (2..).map(|m| m * n).take_while(|&p| p < 1000).collect_vec()
    })
    .unwrap();
    for (i, &vi) in sorted.iter().enumerate() {
        for &vj in sorted.iter().skip(i + 1) {
            assert!(
                vi % vj != 0,
                "{} is located after {} and divides it",
                vj,
                vi
            );
        }
    }
}

#[test]
fn complexity() {
    // To ensure that the sort is O(|E| + |V|), we ensure that the
    // successors for a particular node are requested exactly one time.
    let mut rng = rand::rngs::OsRng::new().unwrap();
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
