//! Find a topological order in a directed graph if one exists.

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// Find a topological order in a directed graph if one exists.
///
/// - `nodes` is a collection of nodes.
/// - `successors` returns a list of successors for a given node.
///
/// The function returns either `Ok` with an acceptable topological order,
/// or `Err` with a node belonging to a cycle. In the latter case, the
/// strongly connected set can then be found using the
/// [strongly_connected_component](super::strongly_connected_components::strongly_connected_component)
/// function, or if only one of the loops is needed the [bfs_loop][super::bfs::bfs_loop] function
/// can be used instead to identify one of the shortest loops involving this node.
///
/// # Examples
///
/// We will sort integers from 1 to 9, each integer having its two immediate
/// greater numbers as successors:
///
/// ```
/// use pathfinding::prelude::topological_sort;
///
/// fn successors(node: &usize) -> Vec<usize> {
///   match *node {
///     n if n <= 7 => vec![n+1, n+2],
///     8 => vec![9],
///     _ => vec![],
///   }
/// }
///
/// let sorted = topological_sort(&[3, 7, 1, 4, 2, 9, 8, 6, 5], successors);
/// assert_eq!(sorted, Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
/// ```
///
/// If, however, there is a loop in the graph (for example, all nodes but 7
/// have also 7 has a successor), one of the nodes in the loop will be returned as
/// an error:
///
/// ```
/// use pathfinding::prelude::*;
///
/// fn successors(node: &usize) -> Vec<usize> {
///   match *node {
///     n if n <= 6 => vec![n+1, n+2, 7],
///     7 => vec![8, 9],
///     8 => vec![7, 9],
///     _ => vec![7],
///   }
/// }
///
/// let sorted = topological_sort(&[3, 7, 1, 4, 2, 9, 8, 6, 5], successors);
/// assert!(sorted.is_err());
///
/// // Let's assume that the returned node is 8 (it can be any node which is part
/// // of a loop). We can lookup up one of the shortest loops containing 8
/// // (8 -> 7 -> 8 is the unique loop with two hops containing 8):
///
/// assert_eq!(bfs_loop(&8, successors), Some(vec![8, 7, 8]));
///
/// // We can also request the whole strongly connected set containing 8. Here
/// // 7, 8, and 9 are all reachable from one another.
///
/// let mut set = strongly_connected_component(&8, successors);
/// set.sort();
/// assert_eq!(set, vec![7, 8, 9]);
/// ```
pub fn topological_sort<N, FN, IN>(nodes: &[N], mut successors: FN) -> Result<Vec<N>, N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut unmarked: HashSet<N> = nodes.iter().cloned().collect::<HashSet<_>>();
    let mut marked = HashSet::with_capacity(nodes.len());
    let mut temp = HashSet::new();
    let mut sorted = VecDeque::with_capacity(nodes.len());
    while let Some(node) = unmarked.iter().cloned().next() {
        temp.clear();
        visit(
            &node,
            &mut successors,
            &mut unmarked,
            &mut marked,
            &mut temp,
            &mut sorted,
        )?;
    }
    Ok(sorted.into_iter().collect())
}

fn visit<N, FN, IN>(
    node: &N,
    successors: &mut FN,
    unmarked: &mut HashSet<N>,
    marked: &mut HashSet<N>,
    temp: &mut HashSet<N>,
    sorted: &mut VecDeque<N>,
) -> Result<(), N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    unmarked.remove(node);
    if marked.contains(node) {
        return Ok(());
    }
    if temp.contains(node) {
        return Err(node.clone());
    }
    temp.insert(node.clone());
    for n in successors(node) {
        visit(&n, successors, unmarked, marked, temp, sorted)?;
    }
    marked.insert(node.clone());
    sorted.push_front(node.clone());
    Ok(())
}
