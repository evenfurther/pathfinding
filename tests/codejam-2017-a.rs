// Problem A from the Google Code Jam finals 2017.
// https://code.google.com/codejam/contest/dashboard?c=6314486#s=p0&a=0

extern crate pathfinding;

use pathfinding::*;
use std::io::{self, Cursor};
use std::io::prelude::*;
use std::num::ParseIntError;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::Parse(err)
    }
}

fn read_ints(file: &mut BufRead) -> Result<Vec<usize>, Error> {
    let mut s = String::new();
    file.read_line(&mut s)?;
    s.pop();
    s.split(' ')
        .map(|w| w.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

fn test<EK: EdmondsKarp<i32>>(n: usize, file: &mut BufRead) -> Result<String, Error> {
    let ndices = read_ints(file)?[0];
    let mut dices = Vec::new();
    let mut values = HashMap::new();
    for d in 0..ndices {
        let mut dice = read_ints(file)?;
        for v in dice.clone() {
            values.entry(v).or_insert_with(HashSet::new).insert(d);
        }
        dice.sort();
        dices.push(dice);
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut keys = values.keys().collect::<Vec<_>>();
    keys.sort();
    for &v in keys {
        if groups.is_empty() || *groups.last().unwrap().last().unwrap() != v - 1 {
            groups.push(vec![v]);
        } else {
            groups.last_mut().unwrap().push(v);
        }
    }
    let answer = groups
        .into_iter()
        .map(|group| {
            // Extract the dices used for this group.
            let subdices = group
                .iter()
                .flat_map(|&v| values[&v].clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .enumerate()
                .map(|(a, b)| (b, a))
                .collect::<BTreeMap<_, _>>();
            // Source is 0, sink is 1. Group members are 2 .. 2 + group.len(), dices are
            // 2 + group.len() .. 2 + group.len() + dices.len()
            let value_offset = 2;
            let dice_offset = value_offset + group.len();
            let size = dice_offset + subdices.len();
            let mut ek = EK::new(size, 0, 1);
            ek.omit_detailed_flows();
            // Set capacity 1 between each value and the dice holding this value.
            let smallest_value = group[0];
            for &value in &group {
                for dice in &values[&value] {
                    ek.set_capacity(
                        value - smallest_value + value_offset,
                        subdices[dice] + dice_offset,
                        1,
                    );
                }
            }
            // Set capacity 1 between each dice and the sink.
            for i in 0..subdices.len() {
                ek.set_capacity(i + dice_offset, 1, 1);
            }
            // Add path from the source to the first value.
            ek.set_capacity(0, value_offset, 1);
            let mut bi = 0;
            let mut ei = 1;
            let mut max = 0;
            loop {
                let (_, n) = ek.augment();
                let n = n as usize;
                if n > max {
                    max = n;
                }
                if max >= group.len() - bi {
                    break;
                }
                if n == ei - bi {
                    if ei == group.len() {
                        break;
                    }
                    // Add path from source to value group[ei]
                    ek.set_capacity(0, group[ei] - smallest_value + value_offset, 1);
                    ei += 1;
                } else {
                    // Remove path from source to value group[bi]
                    ek.set_capacity(0, group[bi] - smallest_value + value_offset, 0);
                    bi += 1;
                    if bi == ei {
                        ei += 1;
                    }
                }
            }
            max
        })
        .max()
        .unwrap();
    Ok(format!("Case #{}: {}", n, answer))
}

#[test]
fn codejam() {
    let mut file_dense = Cursor::new(include_str!("A-small-practice.in"));
    let mut file_sparse = Cursor::new(include_str!("A-small-practice.in"));
    let ntests = read_ints(&mut file_dense).expect("cannot read number of test cases")[0];
    read_ints(&mut file_sparse).unwrap();
    let mut out = String::new();
    for n in 1..(ntests + 1) {
        let dense_result =
            test::<DenseCapacity<i32>>(n, &mut file_dense).expect("problem with test");
        let sparse_result =
            test::<SparseCapacity<i32>>(n, &mut file_sparse).expect("problem with test");
        assert_eq!(
            dense_result,
            sparse_result,
            "dense and sparse results are different"
        );
        out += &dense_result;
        out += "\n";
    }
    let expected = include_str!("A-small-practice.out");
    assert_eq!(out, expected, "answers do not match");
}
