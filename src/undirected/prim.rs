//! Find minimum-spanning-tree in an undirected graph using [Prim's
//! algorithm](https://en.wikipedia.org/wiki/Prim%27s_algorithm).

use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

/// Find a minimum-spanning-tree. From a collection of weighted edges,
/// return a vector of edges forming a minimum-spanning-tree.
///
/// Edges are undirected: `(a, b, c)` and `(b, a, c)` describe the same edge, and either form
/// may be used. The tree is grown from the first endpoint of the first edge.
///
/// # Disconnected graphs
///
/// The tree is grown outwards from one node, so only the component containing that node is
/// spanned; edges in any other component are not returned. [`kruskal`](super::kruskal::kruskal)
/// differs here, and returns a spanning forest covering every component.
///
/// ```
/// use pathfinding::prelude::{kruskal, prim};
///
/// // Two components: 1-2 and 3-4.
/// let edges = vec![(1, 2, 1), (3, 4, 1)];
///
/// // prim spans the component holding node 1, the first endpoint of the first edge.
/// assert_eq!(prim(&edges), vec![(&1, &2, 1)]);
///
/// // kruskal spans both.
/// assert_eq!(kruskal(&edges).count(), 2);
/// ```
pub fn prim<N, C>(edges: &[(N, N, C)]) -> Vec<(&N, &N, C)>
where
    N: Hash + Eq + Ord,
    C: Clone + Ord,
{
    let Some((start, ..)) = edges.first() else {
        return vec![];
    };

    // Index every edge under both of its endpoints, once. Growing the tree then only looks at
    // the edges leaving the node just added, instead of rescanning the whole edge list for
    // each of them.
    let mut incident: FxHashMap<&N, Vec<(&C, &N)>> = FxHashMap::default();
    for (a, b, cost) in edges {
        incident.entry(a).or_default().push((cost, b));
        if a != b {
            incident.entry(b).or_default().push((cost, a));
        }
    }

    let mut mst = Vec::new();
    let mut visited: FxHashSet<&N> = FxHashSet::default();
    visited.insert(start);
    let mut priority_queue = BinaryHeap::new();
    let mut grown = Some(start);
    while let Some(node) = grown.take() {
        // Offer every edge that leaves the tree through the node just added...
        if let Some(candidates) = incident.get(node) {
            for &(cost, other) in candidates {
                if !visited.contains(other) {
                    priority_queue.push(Reverse((cost, node, other)));
                }
            }
        }
        // ... then take the cheapest edge that still reaches a new node.
        while let Some(Reverse((cost, from, to))) = priority_queue.pop() {
            if visited.contains(to) {
                continue;
            }
            mst.push((from, to, cost.clone()));
            visited.insert(to);
            grown = Some(to);
            break;
        }
    }
    mst
}
