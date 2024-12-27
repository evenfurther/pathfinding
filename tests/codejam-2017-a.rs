// Problem A from the Google Code Jam finals 2017.
// https://code.google.com/codejam/contest/dashboard?c=6314486#s=p0&a=0

use pathfinding::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::prelude::*;
use std::io::{self, Cursor};
use std::num::ParseIntError;

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::Parse(err)
    }
}

fn read_ints(file: &mut dyn BufRead) -> Result<Vec<usize>, Error> {
    let mut s = String::new();
    file.read_line(&mut s)?;
    s.pop();
    s.split(' ')
        .map(|w| w.parse::<usize>().map_err(Into::into))
        .collect()
}

fn test<EK: EdmondsKarp<i32>>(n: usize, file: &mut dyn BufRead) -> Result<String, Error> {
    let n_dices = read_ints(file)?[0];
    let mut dices = Vec::new();
    let mut values = HashMap::new();
    for d in 0..n_dices {
        let mut dice = read_ints(file)?;
        for v in dice.clone() {
            values.entry(v).or_insert_with(HashSet::new).insert(d);
        }
        dice.sort_unstable();
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
            ek.omit_details();
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
                let (_, n, _) = ek.augment();
                debug_assert!(n >= 0);
                #[allow(clippy::cast_sign_loss)]
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
    Ok(format!("Case #{n}: {answer}"))
}

fn codejam<EK: EdmondsKarp<i32>>() {
    let mut file = Cursor::new(include_str!("A-small-practice.in"));
    let ntests = read_ints(&mut file).expect("cannot read number of test cases")[0];
    let mut out = String::new();
    for n in 1..=ntests {
        out += &test::<EK>(n, &mut file).expect("problem with test");
        out += "\n";
    }
    let expected = include_str!("A-small-practice.out");
    assert_eq!(out, expected, "answers do not match");
}

#[test]
fn codejam_dense() {
    codejam::<DenseCapacity<_>>();
}

#[test]
fn codejam_sparse() {
    codejam::<SparseCapacity<_>>();
}
