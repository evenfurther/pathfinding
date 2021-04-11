//! Compute a shortest path using the [Dijkstra search
//! algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
use num_traits::Zero;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use super::dijkstra::dijkstra;

/// A representation of a path.
#[derive(Eq, PartialEq, Debug)]
pub struct Path<N: Eq + Hash + Clone, C: Zero + Ord + Copy> {
    /// The nodes along the path
    pub nodes: Vec<N>,
    /// The total cost of the path
    pub cost: C,
}

impl<N, C> PartialOrd for Path<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare costs first, then amount of nodes
        let cmp = self.cost.partial_cmp(&other.cost);
        match cmp {
            Some(Ordering::Equal) => self.nodes.len().partial_cmp(&other.nodes.len()),
            _ => cmp,
        }
    }
}

impl<N, C> Ord for Path<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare costs first, then amount of nodes
        let cmp = self.cost.cmp(&other.cost);
        match cmp {
            Ordering::Equal => self.nodes.len().cmp(&other.nodes.len()),
            _ => cmp,
        }
    }
}
/// Compute the k-shortest paths using the [Yen's search
/// algorithm](https://en.wikipedia.org/wiki/Yen%27s_algorithm).success
///
/// The `k`-shortest paths starting from `start` up to a node for which `success` returns `true`
/// are computed along with their total cost. The result is return as a vector `Path` objects
/// wrapped in `Some`. In case no paths were found, `None` is returned.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost of moving from
///    the node to the successor. Costs MUST be positive.
/// - `success` checks weather the goal has been reached.
/// - `k` is the amount of paths requests, including the shortest one.
///
/// The returned paths comprises both the start and the end node and are ordered by their costs
/// starting with the lowest cost.
///
/// # Example
/// We will search the 3 shortest paths from node C to node H. See
/// https://en.wikipedia.org/wiki/Yen's_algorithm#Example for a visualization.
///
/// ```
/// use pathfinding::prelude::{Path, yen};
/// let result = yen(
///     &'c',
///     |c| match c {
///         'c' => vec![('d', 3), ('e', 2)],
///         'd' => vec![('f', 4)],
///         'e' => vec![('d', 1), ('f', 2), ('g', 3)],
///         'f' => vec![('g', 2), ('h', 1)],
///         'g' => vec![('h', 2)],
///         'h' => vec![],
///         _ => panic!(""),
///         },
///         |c| *c == 'h',
///     2);
///     assert!(result.is_some());
///     let paths = result.unwrap();
///     assert_eq!(
///         paths[0],
///         Path {
///             nodes: vec!['c', 'e', 'f', 'h'],
///             cost: 5
///         }
///     );
///     assert_eq!(
///         paths[1],
///         Path {
///             nodes: vec!['c', 'e', 'g', 'h'],
///             cost: 7
///         }
///     );
///     assert_eq!(
///         paths[2],
///         Path {
///             nodes: vec!['c', 'd', 'f', 'h'],
///             cost: 8
///         }
///     );
/// ```

pub fn yen<N, C, FN, IN, FS>(
    start: &N,
    successors: FN,
    success: FS,
    k: usize,
) -> Option<Vec<Path<N, C>>>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN + Copy,
    IN: IntoIterator<Item = (N, C)>,
    FS: Fn(&N) -> bool + Copy,
{
    let (n, c) = dijkstra(start, successors, success)?;

    let mut visited = HashSet::new();
    // A vector containing our paths.
    let mut routes = vec![Path { nodes: n, cost: c }];
    // A min-heap to store our lowest-cost route candidate
    let mut k_routes = BinaryHeap::new();
    for ki in 0..k {
        if routes.len() <= ki {
            // We have no more routes to explore
            break;
        }
        // Take the most recent route to explore new spurs.
        let previous = &routes[ki].nodes;
        // Iterate over every node except the sink node.
        for i in 0..(previous.len() - 1) {
            let spur_node = &previous[i];
            let root_path = &previous[0..i];

            let mut filtered_edges = HashSet::new();
            for path in routes.iter() {
                if &path.nodes[0..i] == root_path && path.nodes.len() > i + 1 {
                    filtered_edges.insert((&path.nodes[i], &path.nodes[i + 1]));
                }
            }
            let filtered_nodes: HashSet<&N> = HashSet::from_iter(root_path);
            // We are creating a new successor function that will not return the
            // filtered edges and nodes that routes already used.
            let filtered_successor = |n: &N| {
                successors(n)
                    .into_iter()
                    .filter(|(n2, _)| {
                        !filtered_nodes.contains(&n2) && !filtered_edges.contains(&(n, n2))
                    })
                    .collect::<Vec<_>>()
            };

            // Let us find the spur path from the spur node to the sink using.
            if let Some((spur_path, _)) = dijkstra(spur_node, filtered_successor, success) {
                let nodes = make_path(Vec::from(root_path), spur_path);
                // If we have found the same path before, we will not add it.
                if !visited.contains(&hash(&nodes)) {
                    // Since we don't know the root_path cost, we need to recalculate.
                    let cost = make_cost(&nodes, successors);
                    let path = Path {
                        nodes,
                        cost,
                    };
                    // Mark as visited
                    visited.insert(hash(&path.nodes));
                    // Build a min-heap
                    k_routes.push(Reverse(path));
                }
            }
        }
        if let Some(k_route) = k_routes.pop() {
            routes.push(k_route.0);
        }
    }

    routes.sort();
    Some(routes)
}

#[inline]
fn hash<N: Eq + Hash + Clone>(nodes: &[N]) -> u64 {
    let mut hs = DefaultHasher::new();
    nodes.hash(&mut hs);
    hs.finish()
}

#[inline]
fn make_path<N: Eq + Hash + Clone>(a: Vec<N>, b: Vec<N>) -> Vec<N> {
    let mut p = a;
    p.extend(b);
    p
}

#[inline]
fn make_cost<N, FN, IN, C>(nodes: &[N], successors: FN) -> C
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN + Copy,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut cost = C::zero();
    if nodes.is_empty() {
        return cost;
    }

    for edge in nodes.windows(2) {
        let ns = successors(&edge[0]);
        for (n, c) in ns {
            if n == edge[1] {
                cost = cost + c;
            }
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple tests of Yen's algorithm based on the example and visualization
    // from https://en.wikipedia.org/wiki/Yen's_algorithm#Example.
    #[test]
    fn simple() {
        let result = yen(
            &'c',
            |c| match c {
                'c' => vec![('d', 3), ('e', 2)],
                'd' => vec![('f', 4)],
                'e' => vec![('d', 1), ('f', 2), ('g', 3)],
                'f' => vec![('g', 2), ('h', 1)],
                'g' => vec![('h', 2)],
                'h' => vec![],
                _ => panic!(""),
            },
            |c| *c == 'h',
            2,
        );
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            Path {
                nodes: vec!['c', 'e', 'f', 'h'],
                cost: 5
            }
        );
        assert_eq!(
            result[1],
            Path {
                nodes: vec!['c', 'e', 'g', 'h'],
                cost: 7
            }
        );
        assert_eq!(
            result[2],
            Path {
                nodes: vec!['c', 'd', 'f', 'h'],
                cost: 8
            }
        );
    }

    /// Tests that we correctly return fewer routes when
    /// we exhaust all possible paths.
    #[test]
    fn ask_more_than_exist() {
        let result = yen(
            &'c',
            |c| match c {
                'c' => vec![('d', 3), ('e', 2)],
                'd' => vec![('f', 4)],
                'e' => vec![('d', 1), ('f', 2), ('g', 3)],
                'f' => vec![('g', 2), ('h', 1)],
                'g' => vec![('h', 2)],
                'h' => vec![],
                _ => panic!(""),
            },
            |c| *c == 'h',
            10,
        );
        assert!(result.is_some());

        let result = result.unwrap();
        // we asked for 10 but the graph can only produce 7
        assert_eq!(result.len(), 7);
    }

    /// Test that we return None in case there is no solution
    #[test]
    fn no_path() {
        let result = yen(
            &'c',
            |c| match c {
                'c' => vec![('d', 3), ('e', 2)],
                'd' => vec![('f', 4)],
                'e' => vec![('d', 1), ('f', 2), ('g', 3)],
                'f' => vec![('g', 2), ('d', 1)],
                'g' => vec![('e', 2)],
                'h' => vec![],
                _ => panic!(""),
            },
            |c| *c == 'h',
            2,
        );

        assert!(result.is_none());
    }

    /// Test that we return None in case there is no solution
    #[test]
    fn single_node() {
        let result = yen(
            &'c',
            |c| match c {
                'c' => vec![('c', 1)],
                _ => panic!(""),
            },
            |c| *c == 'c',
            2,
        );

        assert!(result.is_some());
        let paths = result.unwrap();
        assert_eq!(paths[0].nodes.len(), 1);
        assert_eq!(paths[0].nodes[0], 'c');
    }
}
