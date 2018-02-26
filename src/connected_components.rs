use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::once;
use std::usize;

/// Lookup entries until we get the same value as the index, with
/// path compression. Adding a new entry to the table consists
/// into pushing the table length.
fn get_and_redirect(table: &mut Vec<usize>, idx: usize) -> usize {
    let r = table[idx];
    if idx == r {
        idx
    } else {
        let r = get_and_redirect(table, r);
        table[idx] = r;
        r
    }
}

/// Separate components of a graph into closed sets.
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
pub fn separate_components<N>(groups: &[Vec<N>]) -> (HashMap<N, usize>, Vec<usize>)
where
    N: Clone + Hash + Eq,
{
    let mut table = Vec::new();
    let mut indices = HashMap::new();
    let mut gindices = Vec::with_capacity(groups.len());
    for g in groups {
        let idxs = g.iter()
            .map(|n| {
                indices
                    .get(n)
                    .map(|&i| get_and_redirect(&mut table, i))
                    .unwrap_or_else(|| {
                        let l = table.len();
                        indices.insert(n, l);
                        table.push(l);
                        l
                    })
            })
            .collect_vec();
        let &idx = idxs.iter().min().unwrap_or(&usize::MAX);
        for i in idxs {
            if i != idx {
                table[i] = idx;
            }
        }
        gindices.push(idx);
    }
    (
        indices
            .into_iter()
            .map(|(n, i)| (n.clone(), get_and_redirect(&mut table, i)))
            .collect(),
        gindices
            .into_iter()
            .map(|i| {
                if i == usize::MAX {
                    i
                } else {
                    get_and_redirect(&mut table, i)
                }
            })
            .collect(),
    )
}

/// Separate components of a graph into closed sets.
///
/// - `groups` is a set of group of vertices connected together. It is
///   acceptable for a group to contain only one node.
///
/// This function returns a list of sets of nodes fomring disjoint connected
/// sets.
pub fn components<N>(groups: &[Vec<N>]) -> Vec<HashSet<N>>
where
    N: Clone + Hash + Eq,
{
    let (_, gindices) = separate_components(groups);
    let gb = gindices
        .into_iter()
        .enumerate()
        .filter(|&(_, n)| n != usize::MAX)
        .sorted_by(|&(_, n1), &(_, n2)| Ord::cmp(&n1, &n2))
        .into_iter()
        .group_by(|&(_, n)| n);
    gb.into_iter()
        .map(|(_, gs)| {
            gs.into_iter()
                .map(|(i, _)| groups[i].clone())
                .flat_map(|i| i)
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
    components(&starts
        .iter()
        .map(|s| {
            neighbours(s)
                .into_iter()
                .chain(once(s.clone()))
                .collect_vec()
        })
        .collect_vec())
}

/// Locate vertices amongst disjoint sets.
///
/// - `components` are disjoint vertices sets.
///
/// This function returns a map between every vertex and the index of
/// the set it belongs to in the `components` list.
#[cfg_attr(feature = "cargo-clippy", allow(implicit_hasher))]
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
