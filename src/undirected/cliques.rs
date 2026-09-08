//! Find cliques in an undirected graph.

use rustc_hash::FxHashSet;
use std::collections::HashSet;
use std::hash::Hash;

/// Find all maximal cliques in an undirected graph.
///
/// That is, it lists all subsets of vertices with the two properties that each pair of vertices in
/// one of the listed subsets is connected by an edge, and no listed subset can have
/// any additional vertices added to it while preserving its complete connectivity.
///  [Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).
///
///
/// - `vertices` is the list of all nodes.
/// - `connected` returns true if the two given node is connected.
/// - return a list of cliques.
pub fn maximal_cliques_collect<N, FN, IN>(vertices: IN, connected: &mut FN) -> Vec<HashSet<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N, &N) -> bool,
    IN: IntoIterator<Item = N>,
{
    let mut result = Vec::new();
    let mut consumer = |n: &HashSet<N>| result.push(n.to_owned());
    maximal_cliques(vertices, connected, &mut consumer);
    result
}

/// Find all maximal cliques in an undirected graph.
///
/// That is, it lists all subsets of vertices with the two properties that each pair of vertices in
/// one of the listed subsets is connected by an edge, and no listed subset can have
/// any additional vertices added to it while preserving its complete connectivity.
///  [Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).
///
///
/// - `vertices` is the list of all nodes.
/// - `connected` returns true if the two given node is connected.
/// - 'consumer' function which called for each clique.
///
pub fn maximal_cliques<N, FN, IN, CO>(vertices: IN, connected: &mut FN, consumer: &mut CO)
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N, &N) -> bool,
    IN: IntoIterator<Item = N>,
    CO: FnMut(&HashSet<N>),
{
    // A vertex must not appear twice in a clique, so the list is deduplicated; keeping the
    // order it was given in keeps the output stable from one run to the next.
    let mut seen = FxHashSet::default();
    let mut candidates = Vec::new();
    for vertex in vertices {
        if seen.insert(vertex.clone()) {
            candidates.push(vertex);
        }
    }
    bron_kerbosch(
        connected,
        &mut Vec::new(),
        &mut candidates,
        &mut Vec::new(),
        consumer,
    );
}

/// One step of the Bron-Kerbosch enumeration.
///
/// `clique` is the clique built so far, `candidates` the vertices that could still extend it, and
/// `excluded` those that could but have already been explored in another branch: a clique is
/// maximal exactly when both of the latter are empty.
///
/// The three sets are vectors rather than hash sets. They are only ever filtered and scanned,
/// never looked up by key, and they shrink quickly with depth, so hashing every vertex at every
/// level costs more than the linear scans it saves.
fn bron_kerbosch<N, FN, CO>(
    connected: &mut FN,
    clique: &mut Vec<N>,
    candidates: &mut Vec<N>,
    excluded: &mut Vec<N>,
    consumer: &mut CO,
) where
    N: Eq + Hash + Clone,
    FN: FnMut(&N, &N) -> bool,
    CO: FnMut(&HashSet<N>),
{
    if candidates.is_empty() {
        if excluded.is_empty() {
            consumer(&clique.iter().cloned().collect());
        }
        return;
    }

    // Take as pivot a vertex of `candidates ∪ excluded` connected to as many candidates as
    // possible. Every maximal clique extending this one either leaves the pivot out or contains
    // one of its neighbours, so the candidates the pivot is connected to need not be branched on
    // here: they are reached through a deeper call instead. Without this the enumeration
    // re-derives the same cliques through every permutation of their vertices.
    //
    // `connected` is free to report a vertex as connected to itself, but a vertex is never its
    // own neighbour here: were the pivot allowed to exclude itself from the branch set, a
    // candidate could be dropped without ever being branched on.
    let branch = {
        let mut pivot = None;
        for vertex in candidates.iter().chain(excluded.iter()) {
            let reach = candidates
                .iter()
                .filter(|n| *n != vertex && connected(vertex, n))
                .count();
            if pivot.is_none_or(|(most, _)| reach > most) {
                pivot = Some((reach, vertex));
            }
        }
        // `candidates` is not empty, so a pivot was always found.
        match pivot {
            Some((_, pivot)) => candidates
                .iter()
                .filter(|n| *n == pivot || !connected(pivot, n))
                .cloned()
                .collect::<Vec<_>>(),
            None => Vec::new(),
        }
    };

    for vertex in branch {
        let mut next_candidates = candidates
            .iter()
            .filter(|n| **n != vertex && connected(&vertex, n))
            .cloned()
            .collect::<Vec<_>>();
        let mut next_excluded = excluded
            .iter()
            .filter(|n| **n != vertex && connected(&vertex, n))
            .cloned()
            .collect::<Vec<_>>();
        clique.push(vertex.clone());
        bron_kerbosch(
            connected,
            clique,
            &mut next_candidates,
            &mut next_excluded,
            consumer,
        );
        clique.pop();

        // This vertex has yielded every clique it can, so any clique found later that could have
        // included it is not maximal.
        if let Some(position) = candidates.iter().position(|n| *n == vertex) {
            candidates.swap_remove(position);
        }
        excluded.push(vertex);
    }
}
