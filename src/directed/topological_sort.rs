//! Find a topological order in a directed graph if one exists.

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::mem;

/// Find a topological order in a directed graph if one exists.
///
/// - `roots` is a collection of nodes that ought to be explored.
/// - `successors` returns a list of successors for a given node, including possibly
///    nodes that were not present in `roots`.
///
/// The function returns either `Ok` with an acceptable topological order of nodes
/// given as roots or discovered, or `Err` with a node belonging to a cycle. In the
/// latter case, the strongly connected set can then be found using the
/// [`strongly_connected_component`](super::strongly_connected_components::strongly_connected_component)
/// function, or if only one of the loops is needed the [`bfs_loop`](super::bfs::bfs_loop) function
/// can be used instead to identify one of the shortest loops involving this node.
///
/// # Examples
///
/// We will sort integers from 1 to 9, each integer having its two immediate
/// greater numbers as successors, starting with two roots 5 and 1:
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
/// let sorted = topological_sort(&[5, 1], successors);
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
/// let sorted = topological_sort(&[5, 1], successors);
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
pub fn topological_sort<N, FN, IN>(roots: &[N], mut successors: FN) -> Result<Vec<N>, N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut marked = HashSet::with_capacity(roots.len());
    let mut temp = HashSet::new();
    let mut sorted = VecDeque::with_capacity(roots.len());
    let mut roots: HashSet<N> = roots.iter().cloned().collect::<HashSet<_>>();
    while let Some(node) = roots.iter().cloned().next() {
        temp.clear();
        visit(
            &node,
            &mut successors,
            &mut roots,
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

/// Topologically sort a directed graph into groups of independent nodes.
///
/// - `nodes` is a collection of nodes.
/// - `successors` returns a list of successors for a given node.
///
/// This function works like [`topological_sort`](self::topological_sort), but
/// rather than producing a single ordering of nodes, this function partitions
/// the nodes into groups: the first group contains all nodes with no
/// dependencies, the second group contains all nodes whose only dependencies
/// are in the first group, and so on. Concatenating the groups produces a
/// valid topological sort regardless of how the nodes within each group are
/// reordered. No guarantees are made about the order of nodes within each
/// group. Also, the list of `nodes` must be exhaustive, new nodes must not be
/// returned by the `successors` function.
///
/// The function returns either `Ok` with a valid list of groups, or `Err` with
/// a (groups, remaining) tuple containing a (possibly empty) partial list of
/// groups, and a list of remaining nodes that could not be grouped due to
/// cycles. In the error case, the strongly connected set(s) can then be found
/// using the
/// [`strongly_connected_components`](super::strongly_connected_components::strongly_connected_components)
/// function on the list of remaining nodes.
///
/// The current implementation uses a variation of [Kahn's
/// algorithm](https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm),
/// and runs in O(|V| + |E|) time.
#[allow(clippy::type_complexity)]
pub fn topological_sort_into_groups<N, FN, IN>(
    nodes: &[N],
    mut successors: FN,
) -> Result<Vec<Vec<N>>, (Vec<Vec<N>>, Vec<N>)>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    if nodes.is_empty() {
        return Ok(Vec::new());
    }
    let mut succs_map = HashMap::<N, HashSet<N>>::with_capacity(nodes.len());
    let mut preds_map = HashMap::<N, usize>::with_capacity(nodes.len());
    for node in nodes.iter() {
        succs_map.insert(node.clone(), successors(node).into_iter().collect());
        preds_map.insert(node.clone(), 0);
    }
    for succs in succs_map.values() {
        for succ in succs.iter() {
            *preds_map.get_mut(succ).unwrap() += 1;
        }
    }
    let mut groups = Vec::<Vec<N>>::new();
    let mut prev_group: Vec<N> = preds_map
        .iter()
        .filter_map(|(node, &num_preds)| {
            if num_preds == 0 {
                Some(node.clone())
            } else {
                None
            }
        })
        .collect();
    if prev_group.is_empty() {
        let remaining: Vec<N> = preds_map.into_iter().map(|(node, _)| node).collect();
        return Err((Vec::new(), remaining));
    }
    for node in &prev_group {
        preds_map.remove(node);
    }
    while !preds_map.is_empty() {
        let mut next_group = Vec::<N>::new();
        for node in &prev_group {
            for succ in &succs_map[node] {
                {
                    let num_preds = preds_map.get_mut(succ).unwrap();
                    *num_preds -= 1;
                    if *num_preds > 0 {
                        continue;
                    }
                }
                next_group.push(preds_map.remove_entry(succ).unwrap().0);
            }
        }
        groups.push(mem::replace(&mut prev_group, next_group));
        if prev_group.is_empty() {
            let remaining: Vec<N> = preds_map.into_iter().map(|(node, _)| node).collect();
            return Err((groups, remaining));
        }
    }
    groups.push(prev_group);
    Ok(groups)
}
