extern crate num;

mod astar;
mod bfs;
mod dijkstra;
mod fringe;

pub use astar::*;
pub use bfs::*;
pub use dijkstra::*;
pub use fringe::*;

use std::cmp::Ordering;

struct InvCmpHolder<K, P> {
    key: K,
    payload: P,
}

impl<K: PartialEq, P> PartialEq for InvCmpHolder<K, P> {
    fn eq(&self, other: &InvCmpHolder<K, P>) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: PartialEq, P> Eq for InvCmpHolder<K, P> {}

impl<K: PartialOrd, P> PartialOrd for InvCmpHolder<K, P> {
    fn partial_cmp(&self, other: &InvCmpHolder<K, P>) -> Option<Ordering> {
        other.key.partial_cmp(&self.key)
    }
}

impl<K: Ord, P> Ord for InvCmpHolder<K, P> {
    fn cmp(&self, other: &InvCmpHolder<K, P>) -> Ordering {
        other.key.cmp(&self.key)
    }
}
