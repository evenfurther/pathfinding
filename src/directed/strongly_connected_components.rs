//! Separate nodes of a directed graph into [strongly connected
//! components](https://en.wikipedia.org/wiki/Strongly_connected_component).
//!
//! A [path-based strong component
//! algorithm](https://en.wikipedia.org/wiki/Path-based_strong_component_algorithm)
//! is used.

use rustc_hash::FxHashMap;
use std::hash::Hash;

/// Marks a node whose component has already been closed. Preorder numbers count nodes, so a
/// real one can never reach this value.
const ASSIGNED: usize = usize::MAX;

struct Params<N, FN>
where
    N: Hash + Eq,
{
    /// Preorder number of every node the traversal has entered, or [`ASSIGNED`] once the node's
    /// component has been emitted. Keeping both states in one map means a successor costs a
    /// single lookup instead of one lookup per state.
    preorders: FxHashMap<N, usize>,
    c: usize,
    successors: FN,
    /// Preorder numbers of the path nodes whose component is still open.
    p: Vec<usize>,
    /// Nodes entered but not yet assigned to a component, with their preorder numbers. Nodes
    /// are pushed as they are entered, so the preorder numbers increase from bottom to top.
    s: Vec<(N, usize)>,
    scc: Vec<Vec<N>>,
}

impl<N, FN, IN> Params<N, FN>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    fn new(successors: FN) -> Self {
        Self {
            preorders: FxHashMap::default(),
            c: 0,
            successors,
            p: Vec::new(),
            s: Vec::new(),
            scc: Vec::new(),
        }
    }
}

/// Explore the graph from `start`, emitting each strongly connected component as it is closed.
///
/// The traversal keeps its own stack rather than recursing: the depth is bounded only by the
/// number of nodes, which is enough to exhaust the call stack on graphs of very ordinary size.
fn traverse_from<N, FN, IN>(start: &N, params: &mut Params<N, FN>)
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    // Each entry is a node's preorder number and the successors of it left to look at.
    let mut stack: Vec<(usize, IN::IntoIter)> = Vec::new();
    let pv = params.enter(start.clone());
    stack.push((pv, (params.successors)(start).into_iter()));

    while let Some(top) = stack.last_mut() {
        // The borrow of `stack` ends here, so that the body below is free to push onto it.
        let successor = top.1.next();
        if let Some(w) = successor {
            match params.preorders.get(&w) {
                // Already in a component of its own: it cannot be part of this one.
                Some(&ASSIGNED) => (),
                // Already on the current path, so everything entered after `w` belongs to the
                // same component as `w`: those paths can no longer be closed separately.
                Some(&pw) => {
                    while params.p.last().is_some_and(|&p| p > pw) {
                        params.p.pop();
                    }
                }
                None => {
                    let pw = params.enter(w.clone());
                    stack.push((pw, (params.successors)(&w).into_iter()));
                }
            }
        } else {
            // Every successor has been looked at: if this node still heads an open path, it is
            // the root of a component made of everything entered since.
            let (pv, _) = stack.pop().unwrap();
            if params.p.last() == Some(&pv) {
                params.p.pop();
                // `s` is ordered by preorder number, so the component is exactly its tail.
                let first = params.s.partition_point(|&(_, p)| p < pv);
                let component = params
                    .s
                    .drain(first..)
                    .map(|(node, _)| node)
                    .collect::<Vec<_>>();
                for node in &component {
                    // Cannot fail: every node in `s` was given a preorder number on entry.
                    if let Some(preorder) = params.preorders.get_mut(node) {
                        *preorder = ASSIGNED;
                    }
                }
                params.scc.push(component);
            }
        }
    }
}

impl<N, FN> Params<N, FN>
where
    N: Clone + Hash + Eq,
{
    /// Record `node` as entered and return the preorder number it was given.
    fn enter(&mut self, node: N) -> usize {
        let preorder = self.c;
        self.c += 1;
        self.preorders.insert(node.clone(), preorder);
        self.p.push(preorder);
        self.s.push((node, preorder));
        preorder
    }
}

/// Partition nodes reachable from a starting point into strongly connected components.
///
/// - `start` is the node we want to explore the graph from.
/// - `successors` returns a list of successors for a given node.
///
/// The function returns a list of strongly connected components sets. It will contain
/// at least one component (the one containing the `start` node).
pub fn strongly_connected_components_from<N, FN, IN>(start: &N, successors: FN) -> Vec<Vec<N>>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut params = Params::new(successors);
    traverse_from(start, &mut params);
    params.scc
}

/// Compute the strongly connected component containing a given node.
///
/// - `node` is the node we want the strongly connected component for.
/// - `successors` returns a list of successors for a given node.
///
/// The function returns the strongly connected component containing the node,
/// which is guaranteed to contain at least `node`.
#[expect(clippy::missing_panics_doc)]
pub fn strongly_connected_component<N, FN, IN>(node: &N, successors: FN) -> Vec<N>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    // The unwrap() cannot fail as there will always be at least one group.
    strongly_connected_components_from(node, successors)
        .pop()
        .unwrap()
}

/// Partition all strongly connected components in a graph.
///
/// - `nodes` is a collection of nodes.
/// - `successors` returns a list of successors for a given node.
///
/// The function returns a list of strongly connected components sets.
pub fn strongly_connected_components<N, FN, IN>(nodes: &[N], successors: FN) -> Vec<Vec<N>>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut params = Params::new(successors);
    for node in nodes {
        if !params.preorders.contains_key(node) {
            traverse_from(node, &mut params);
        }
    }
    params.scc
}
