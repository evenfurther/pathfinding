//! Separate components of an undirected graph into disjoint sets.

use itertools::Itertools;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::once;
use std::usize;

/// Lookup entries until we get the same value as the index, with
/// path halving. Adding a new entry to the table consists
/// into pushing the table length.
fn get_and_redirect(table: &mut Vec<usize>, mut idx: usize) -> usize {
    while idx != table[idx] {
        table[idx] = table[table[idx]];
        idx = table[idx];
    }
    idx
}

/// Separate components of an undirected graph into disjoint sets.
///
/// - `groups` is a set of group of vertices connected together. It is
///   acceptable for a group to contain only one node. Empty groups
///   receive special treatment (see below).
///
/// This function returns a pair containing:
///
/// - A mapping from every vertex to its set identifier. The set identifiers are
/// opaque and will not necessarily be compact. However, it is guaranteed that
/// they will not be greater than the number of groups.
/// - A mapping from every group to its set identifier, with the identifiers being
/// the same ones as the ones in the previous mapping. Each group corresponds to
/// the identifier at the same index, except for empty group whose identifier is
/// set to `std::usize::MAX`.
///
/// Note that if you have a raw undirected graph, you can build
/// such a structure by creating a group for every vertex containing
/// the vertex itself and its immediate neighbours.
#[must_use]
pub fn separate_components<N>(groups: &[Vec<N>]) -> (HashMap<N, usize>, Vec<usize>)
where
    N: Clone + Hash + Eq,
{
    let mut table = (0..groups.len()).collect_vec();
    let mut indices = HashMap::new();
    for (mut group_index, group) in groups.iter().enumerate() {
        if group.is_empty() {
            table[group_index] = usize::max_value();
        }
        for element in group {
            match indices.entry(element.clone()) {
                Occupied(e) => {
                    table[group_index] = get_and_redirect(&mut table, *e.get());
                    group_index = table[group_index];
                }
                Vacant(e) => {
                    e.insert(group_index);
                }
            }
        }
    }
    for group_index in indices.values_mut() {
        *group_index = get_and_redirect(&mut table, *group_index);
    }
    for group_index in 0..groups.len() {
        if table[group_index] != usize::max_value() {
            let target = get_and_redirect(&mut table, group_index);
            // Due to path halving, this particular entry might not
            // be up-to-date yet.
            table[group_index] = target;
        }
    }
    (indices, table)
}

/// Separate components of an undirected graph into disjoint sets.
///
/// - `groups` is a set of group of vertices connected together. It is
///   acceptable for a group to contain only one node.
///
/// This function returns a list of sets of nodes forming disjoint connected
/// sets.
#[must_use]
pub fn components<N>(groups: &[Vec<N>]) -> Vec<HashSet<N>>
where
    N: Clone + Hash + Eq,
{
    let (_, gindices) = separate_components(groups);
    let gb = gindices
        .into_iter()
        .enumerate()
        .filter(|&(_, n)| n != usize::max_value())
        .sorted_by(|&(_, n1), &(_, n2)| Ord::cmp(&n1, &n2))
        .group_by(|&(_, n)| n);
    gb.into_iter()
        .map(|(_, gs)| {
            gs.flat_map(|(i, _)| groups[i].clone())
                .collect::<HashSet<_>>()
        })
        .collect()
}

/// Extract connected components from a graph.
///
/// - `starts` is a collection of vertices to be considered as start points.
/// - `neighbours` is a function returning the neighbours of a given node.
///
/// This function returns a list of sets of nodes forming disjoint connected
/// sets.
pub fn connected_components<N, FN, IN>(starts: &[N], mut neighbours: FN) -> Vec<HashSet<N>>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    components(
        &starts
            .iter()
            .map(|s| {
                neighbours(s)
                    .into_iter()
                    .chain(once(s.clone()))
                    .collect_vec()
            })
            .collect_vec(),
    )
}

/// Locate vertices amongst disjoint sets.
///
/// - `components` are disjoint vertices sets.
///
/// This function returns a map between every vertex and the index of
/// the set it belongs to in the `components` list.
#[allow(clippy::implicit_hasher)]
#[must_use]
pub fn component_index<N>(components: &[HashSet<N>]) -> HashMap<N, usize>
where
    N: Clone + Hash + Eq,
{
    let mut assoc = HashMap::with_capacity(components.len());
    for (i, c) in components.iter().enumerate() {
        for n in c {
            assoc.insert(n.clone(), i);
        }
    }
    assoc
}
