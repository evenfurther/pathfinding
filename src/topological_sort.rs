use std::collections::HashSet;
use std::hash::Hash;

/// Find a topological order in a directed graph if one exists.
///
/// - `nodes` is a collection of nodes.
/// - `successors` returns a list of successors for a given node.
///
/// The function returns either `Ok` with an acceptable topological order,
/// or `Err` with a node belonging to a cycle.
///
/// # Examples
///
/// We will sort integers from 1 to 9, each integer having its two immediate
/// greater numbers as successors:
///
/// ```
/// use pathfinding::topological_sort;
///
/// let sorted = topological_sort(&[3, 7, 1, 4, 2, 9, 8, 6, 5],
///                               |&n| (n+1..10).take(2));
///  assert_eq!(sorted, Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
/// ```
///
/// If, however, there is a loop in the graph (for example, all nodes including
/// 9, have 9 has a successor), one of the nodes in the loop will be returned as
/// an error:
///
/// ```
/// use pathfinding::topological_sort;
///
/// let sorted = topological_sort(&[3, 7, 1, 4, 2, 9, 8, 6, 5],
///                               |&n| (n+1..10).take(2).chain(std::iter::once(9)));
///  assert_eq!(sorted, Err(9));
/// ```
pub fn topological_sort<N, FN, IN>(nodes: &[N], successors: FN) -> Result<Vec<N>, N>
where
    N: Eq + Hash + Clone,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut unmarked: HashSet<N> = nodes.iter().cloned().collect::<HashSet<_>>();
    let mut marked = HashSet::with_capacity(nodes.len());
    let mut temp = HashSet::new();
    let mut sorted = Vec::with_capacity(nodes.len());
    while let Some(node) = unmarked.iter().cloned().next() {
        temp.clear();
        visit(
            &node,
            &successors,
            &mut unmarked,
            &mut marked,
            &mut temp,
            &mut sorted,
        )?;
    }
    sorted.reverse();
    Ok(sorted)
}

fn visit<N, FN, IN>(
    node: &N,
    successors: &FN,
    unmarked: &mut HashSet<N>,
    marked: &mut HashSet<N>,
    temp: &mut HashSet<N>,
    sorted: &mut Vec<N>,
) -> Result<(), N>
where
    N: Eq + Hash + Clone,
    FN: Fn(&N) -> IN,
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
    sorted.push(node.clone());
    Ok(())
}
