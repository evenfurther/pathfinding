use num::Zero;
use std::collections::{BinaryHeap, HashSet};
use std::iter::once;
use std::hash::Hash;

use super::InvCmpHolder;

pub fn astar<N, C, FN, IN, FH, FS>(start: &N,
                                   neighbours: FN,
                                   heuristic: FH,
                                   success: FS)
                                   -> Option<(Vec<N>, C)>
    where N: Eq + Hash + Clone,
          C: Zero + Ord + Copy,
          FN: Fn(&N) -> IN,
          IN: IntoIterator<Item = (N, C)>,
          FH: Fn(&N) -> C,
          FS: Fn(&N) -> bool
{
    let mut to_see = BinaryHeap::new();
    to_see.push(InvCmpHolder {
        key: heuristic(start),
        payload: (Zero::zero(), vec![start.clone()]),
    });
    let mut considered = HashSet::new();
    considered.insert(start.clone());
    while let Some(InvCmpHolder { key: _, payload: (cost, path) }) = to_see.pop() {
        let node = path.last().unwrap();
        if success(node) {
            return Some((path.clone(), cost));
        }
        for (neighbour, move_cost) in neighbours(node) {
            if !considered.contains(&neighbour) {
                considered.insert(neighbour.clone());
                let new_cost = cost + move_cost;
                let new_predicted_cost = new_cost + heuristic(&neighbour);
                let new_path = path.iter().cloned().chain(once(neighbour)).collect();
                to_see.push(InvCmpHolder {
                    key: new_predicted_cost,
                    payload: (new_cost, new_path),
                });
            }
        }
    }
    None
}
