//! Single-source shortest paths via the BMSSP algorithm of Duan, Mao, Mao,
//! Shu and Yin ([arXiv:2504.17033](https://arxiv.org/abs/2504.17033)).
//!
//! The public functions have the same shape as [`dijkstra_all`](super::dijkstra::dijkstra_all)
//! and [`dijkstra`](super::dijkstra::dijkstra). Distances are exact. The internal
//! frontier queue is a balanced tree rather than the block structure of the
//! paper, so this implementation does not claim the \(O(m\log^{2/3}n)\) bound.
//! A final repair pass fixes nodes left stale when many paths share a length
//! (the paper assumes unique path lengths). That pass is label-correcting and
//! re-enqueues a node whenever its distance improves, so it is not a single
//! sweep: in the worst case it behaves like Bellman-Ford, and no linear bound
//! is claimed for it either.

use crate::FxIndexMap;
use indexmap::map::Entry::{Occupied, Vacant};
use num_traits::Zero;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;

/// Compute a shortest path using the [BMSSP](https://arxiv.org/abs/2504.17033)
/// single-source algorithm.
///
/// Same result as [`dijkstra`](super::dijkstra::dijkstra) on a **finite**
/// reachable graph: the path from `start` to a cheapest node for which
/// `success` is true, together with its cost.
///
/// This computes all distances first. Prefer
/// [`dijkstra`](super::dijkstra::dijkstra) when the successor graph is
/// unbounded or you only need one target.
///
/// # Example
///
/// ```
/// use pathfinding::prelude::sssp;
///
/// fn successors(&n: &u32) -> Vec<(u32, usize)> {
///     if n <= 4 {
///         vec![(n * 2, 10), (n * 2 + 1, 10)]
///     } else {
///         vec![]
///     }
/// }
///
/// let (path, cost) = sssp(&1, successors, |&n| n == 9).expect("no path");
/// assert_eq!(cost, 30);
/// assert_eq!(path, vec![1, 2, 4, 9]);
/// ```
pub fn sssp<N, C, FN, IN, FS>(start: &N, successors: FN, mut success: FS) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    if success(start) {
        return Some((vec![start.clone()], Zero::zero()));
    }
    let parents = sssp_all(start, successors);
    let (target, cost) = parents
        .iter()
        .filter(|(node, _)| success(node))
        .min_by_key(|(_, (_, cost))| *cost)
        .map(|(node, (_, cost))| (node.clone(), *cost))?;
    Some((super::dijkstra::build_path(&target, &parents), cost))
}

/// Determine all reachable nodes from a starting point, and an optimal parent
/// and cost for each, using BMSSP ([arXiv:2504.17033](https://arxiv.org/abs/2504.17033)).
///
/// Same result type as [`dijkstra_all`](super::dijkstra::dijkstra_all): every
/// reachable node except `start` maps to `(parent, cost_from_start)`. Use
/// [`build_path`](super::dijkstra::build_path) to recover a path.
///
/// The reachable graph must be finite.
///
/// # Example
///
/// ```
/// use pathfinding::prelude::sssp_all;
///
/// fn successors(&n: &u32) -> Vec<(u32, usize)> {
///     if n <= 4 {
///         vec![(n * 2, 10), (n * 2 + 1, 10)]
///     } else {
///         vec![]
///     }
/// }
///
/// let reachables = sssp_all(&1, successors);
/// assert_eq!(reachables.len(), 8);
/// assert_eq!(reachables[&2], (1, 10));
/// assert_eq!(reachables[&9], (4, 30));
/// ```
pub fn sssp_all<N, C, FN, IN>(start: &N, successors: FN) -> HashMap<N, (N, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let (nodes, adj) = materialize(start, successors);
    let n = nodes.len();
    if n == 0 {
        return HashMap::new();
    }

    let mut dist = vec![None; n];
    let mut pred = vec![None; n];
    dist[0] = Some(Zero::zero());

    let (k, t, lmax) = parameters(n);
    let mut engine = Engine {
        adj: &adj,
        dist: &mut dist,
        pred: &mut pred,
        k,
        t,
    };
    engine.bmssp(lmax, None, &[0]);
    engine.repair();

    let mut out = HashMap::with_capacity(n.saturating_sub(1));
    for i in 1..n {
        if let (Some(p), Some(cost)) = (pred[i], dist[i]) {
            out.insert(nodes[i].clone(), (nodes[p].clone(), cost));
        }
    }
    out
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn parameters(n: usize) -> (usize, usize, usize) {
    let n = n.max(2) as f64;
    let log_n = n.log2();
    let k = log_n.powf(1.0 / 3.0).floor() as usize;
    let t = log_n.powf(2.0 / 3.0).floor() as usize;
    let k = k.max(1);
    let t = t.max(1);
    let lmax = (log_n / t as f64).ceil() as usize;
    (k, t, lmax.max(1))
}

fn materialize<N, C, FN, IN>(start: &N, mut successors: FN) -> (Vec<N>, Vec<Vec<(usize, C)>>)
where
    N: Eq + Hash + Clone,
    C: Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut nodes: FxIndexMap<N, ()> = FxIndexMap::default();
    nodes.insert(start.clone(), ());
    let mut adj = Vec::new();
    let mut i = 0;
    while i < nodes.len() {
        let node = nodes.get_index(i).unwrap().0.clone();
        let mut edges = Vec::new();
        for (succ, cost) in successors(&node) {
            let j = match nodes.entry(succ) {
                Vacant(e) => {
                    let idx = e.index();
                    e.insert(());
                    idx
                }
                Occupied(e) => e.index(),
            };
            edges.push((j, cost));
        }
        adj.push(edges);
        i += 1;
    }
    (nodes.into_iter().map(|(n, ())| n).collect(), adj)
}

fn exp2_cap(exp: usize, cap: usize) -> usize {
    if exp >= usize::BITS as usize {
        cap
    } else {
        (1usize << exp).min(cap)
    }
}

fn less_than<C: Ord>(value: &C, bound: Option<C>) -> bool {
    match bound {
        None => true,
        Some(b) => *value < b,
    }
}

struct Engine<'a, C> {
    adj: &'a [Vec<(usize, C)>],
    dist: &'a mut [Option<C>],
    pred: &'a mut [Option<usize>],
    k: usize,
    t: usize,
}

impl<C> Engine<'_, C>
where
    C: Zero + Ord + Copy,
{
    fn relax(&mut self, u: usize, v: usize, weight: C) -> Option<C> {
        if u == v {
            // A self-loop is never part of a shortest path when weights are non-negative, and
            // accepting one at zero weight would make a node its own parent: the tie-break
            // below takes the lower-numbered parent, so a node whose parent is numbered above
            // it would adopt itself, and walking the parents back from it never terminates.
            return None;
        }
        let du = self.dist[u]?;
        let cand = du + weight;
        let valid = self.dist[v].is_none_or(|old| cand <= old);
        if !valid {
            return None;
        }
        let better = self.dist[v]
            .is_none_or(|old| cand < old || (cand == old && self.pred[v].is_none_or(|p| u < p)));
        if better {
            self.dist[v] = Some(cand);
            self.pred[v] = Some(u);
        }
        Some(cand)
    }

    fn bmssp(
        &mut self,
        level: usize,
        bound: Option<C>,
        sources: &[usize],
    ) -> (Option<C>, Vec<usize>) {
        if sources.is_empty() {
            return (bound, Vec::new());
        }
        if level == 0 {
            return self.base_case(bound, sources[0]);
        }

        let (mut pivots, witnessed) = self.find_pivots(bound, sources);
        if pivots.is_empty() {
            pivots.extend(sources.iter().copied());
        }
        let m = exp2_cap((level - 1).saturating_mul(self.t), self.adj.len().max(1));
        let mut queue = PartialQueue::new(bound, m.max(1));
        for &x in &pivots {
            if let Some(dx) = self.dist[x] {
                queue.insert(x, dx);
            }
        }

        let limit = self.k.saturating_mul(exp2_cap(
            level.saturating_mul(self.t),
            self.adj.len().max(1),
        ));
        let mut completed = FxHashSet::default();
        let mut last_prime = bound;

        while completed.len() < limit && !queue.is_empty() {
            let (si, bi) = queue.pull();
            if si.is_empty() {
                break;
            }
            let (bi_prime, ui) = self.bmssp(level - 1, bi, &si);
            last_prime = bi_prime;
            completed.extend(ui.iter().copied());

            let mut batch = Vec::new();
            for &u in &ui {
                for &(v, weight) in &self.adj[u] {
                    let Some(cand) = self.relax(u, v, weight) else {
                        continue;
                    };
                    if !less_than(&cand, bound) {
                        continue;
                    }
                    // A vertex already completed at this level has a settled distance. Offering
                    // it that same distance again only puts it back in the queue to be pulled
                    // and completed once more, which a zero-weight self-loop does for ever: the
                    // pull removed it from the queue, so nothing rejects the re-insertion.
                    if completed.contains(&v) {
                        continue;
                    }
                    if less_than(&cand, bi) {
                        batch.push((v, cand));
                    } else {
                        queue.insert(v, cand);
                    }
                }
            }
            for &x in &si {
                // Likewise for the sources: one the recursion has already finished must not be
                // handed back to the queue, or the same call repeats unchanged.
                if completed.contains(&x) {
                    continue;
                }
                if let Some(dx) =
                    self.dist[x].filter(|dx| !less_than(dx, bi_prime) && less_than(dx, bi))
                {
                    batch.push((x, dx));
                }
            }
            queue.batch_prepend(&batch);
        }

        let b_prime = match (last_prime, bound) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (None, b) | (b, None) => b,
        };
        for &x in &witnessed {
            if self.dist[x].is_some_and(|dx| less_than(&dx, b_prime)) {
                completed.insert(x);
            }
        }
        (b_prime, completed.into_iter().collect())
    }

    /// Settle vertices out of `source` in order of distance and report the boundary reached,
    /// together with the vertices now known to be final below it.
    ///
    /// The paper assumes every shortest path length is distinct, which lets the base case stop
    /// once `k + 1` vertices are settled and take the largest of their distances as the new
    /// boundary. Ties break that: when the settled vertices all lie at the same distance,
    /// nothing is strictly below the boundary, the caller receives an empty set, its
    /// `completed` set never grows, and it re-queues the same sources forever. A zero-cost
    /// edge reaches that state immediately, but any tie at the cutoff will do it.
    ///
    /// Settling therefore continues until the next vertex is strictly further away than
    /// everything already settled. Every vertex nearer than that one has been settled, so it
    /// is a sound boundary, everything returned lies strictly below it, and the set is never
    /// empty.
    fn base_case(&mut self, bound: Option<C>, source: usize) -> (Option<C>, Vec<usize>) {
        let mut settled = Vec::new();
        let mut done = FxHashSet::default();
        let mut heap = BinaryHeap::new();
        if let Some(ds) = self.dist[source] {
            heap.push(Reverse((ds, source)));
        }
        // The largest distance among the vertices settled so far.
        let mut largest: Option<C> = None;

        while let Some(&Reverse((next, _))) = heap.peek() {
            if settled.len() > self.k && largest.is_some_and(|l| next > l) {
                return (Some(next), settled);
            }
            let Some(Reverse((cost, u))) = heap.pop() else {
                break;
            };
            let Some(du) = self.dist[u] else {
                continue;
            };
            if cost > du || !less_than(&du, bound) {
                continue;
            }
            if !done.insert(u) {
                continue;
            }
            settled.push(u);
            largest = Some(largest.map_or(du, |l: C| if du > l { du } else { l }));
            for &(v, weight) in &self.adj[u] {
                let Some(cand) = self.relax(u, v, weight) else {
                    continue;
                };
                if less_than(&cand, bound) {
                    heap.push(Reverse((cand, v)));
                }
            }
        }

        // Everything reachable below `bound` has been settled, so `bound` is itself the
        // boundary. The source stands in when it was not reachable at all, so that the caller
        // is never handed an empty set.
        if settled.is_empty() {
            settled.push(source);
        }
        (bound, settled)
    }

    /// Propagate leftover improvements. BMSSP can leave a child stale when a
    /// parent is later corrected; that happens on graphs with many equal-cost
    /// paths, which the paper excludes by assuming unique path lengths.
    ///
    /// This is a label-correcting pass, not a single sweep: a node goes back on
    /// the queue every time its distance improves, so a node and its edges can
    /// be processed more than once and the worst case is that of Bellman-Ford.
    /// In practice it settles quickly, because it starts from distances BMSSP
    /// has already very nearly finished.
    fn repair(&mut self) {
        let n = self.adj.len();
        let mut queue = VecDeque::new();
        let mut queued = vec![false; n];
        for (u, queued_flag) in queued.iter_mut().enumerate() {
            if self.dist[u].is_some() {
                queue.push_back(u);
                *queued_flag = true;
            }
        }
        while let Some(u) = queue.pop_front() {
            queued[u] = false;
            let Some(du) = self.dist[u] else {
                continue;
            };
            for &(v, weight) in &self.adj[u] {
                let cand = du + weight;
                let better = self.dist[v].is_none_or(|old| cand < old);
                if better {
                    self.dist[v] = Some(cand);
                    self.pred[v] = Some(u);
                    if !queued[v] {
                        queued[v] = true;
                        queue.push_back(v);
                    }
                }
            }
        }
    }

    fn find_pivots(&mut self, bound: Option<C>, sources: &[usize]) -> (Vec<usize>, Vec<usize>) {
        let mut witnessed = FxHashSet::default();
        witnessed.extend(sources.iter().copied());
        let mut layer: Vec<usize> = sources.to_vec();

        for _ in 0..self.k {
            let mut next = Vec::new();
            for &u in &layer {
                for &(v, weight) in &self.adj[u] {
                    let Some(cand) = self.relax(u, v, weight) else {
                        continue;
                    };
                    if less_than(&cand, bound) {
                        next.push(v);
                        witnessed.insert(v);
                    }
                }
            }
            if witnessed.len() > self.k.saturating_mul(sources.len()) {
                return (sources.to_vec(), witnessed.into_iter().collect());
            }
            if next.is_empty() {
                break;
            }
            layer = next;
        }

        let in_w: FxHashSet<usize> = witnessed.iter().copied().collect();
        let mut children: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
        let mut has_parent = FxHashSet::default();
        for &v in &witnessed {
            if let Some(u) = self.pred[v].filter(|u| in_w.contains(u)) {
                children.entry(u).or_default().push(v);
                has_parent.insert(v);
            }
        }

        let mut memo = FxHashMap::default();
        let mut pivots = Vec::new();
        for &u in sources {
            if !in_w.contains(&u) || has_parent.contains(&u) {
                continue;
            }
            if tree_size(u, &children, &mut memo) >= self.k {
                pivots.push(u);
            }
        }
        (pivots, witnessed.into_iter().collect())
    }
}

fn tree_size(
    u: usize,
    children: &FxHashMap<usize, Vec<usize>>,
    memo: &mut FxHashMap<usize, usize>,
) -> usize {
    if let Some(&sz) = memo.get(&u) {
        return sz;
    }
    let mut total = 1;
    if let Some(vs) = children.get(&u) {
        for &v in vs {
            total += tree_size(v, children, memo);
        }
    }
    memo.insert(u, total);
    total
}

/// Partial-order frontier: insert, batch-prepend, and pull the next `m` keys.
struct PartialQueue<C> {
    by_value: BTreeMap<C, Vec<usize>>,
    values: FxHashMap<usize, C>,
    bound: Option<C>,
    m: usize,
}

impl<C: Ord + Copy> PartialQueue<C> {
    fn new(bound: Option<C>, m: usize) -> Self {
        Self {
            by_value: BTreeMap::new(),
            values: FxHashMap::default(),
            bound,
            m,
        }
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn insert(&mut self, key: usize, value: C) {
        if !less_than(&value, self.bound) {
            return;
        }
        if let Some(&old) = self.values.get(&key) {
            if value >= old {
                return;
            }
            self.remove_from_bucket(old, key);
        }
        self.values.insert(key, value);
        self.by_value.entry(value).or_default().push(key);
    }

    fn batch_prepend(&mut self, items: &[(usize, C)]) {
        for &(key, value) in items {
            self.insert(key, value);
        }
    }

    fn pull(&mut self) -> (Vec<usize>, Option<C>) {
        let mut taken = Vec::new();
        while taken.len() < self.m {
            let Some((&value, _)) = self.by_value.first_key_value() else {
                break;
            };
            let mut bucket = self.by_value.remove(&value).unwrap_or_default();
            while let Some(key) = bucket.pop() {
                if self.values.get(&key) != Some(&value) {
                    continue;
                }
                self.values.remove(&key);
                taken.push(key);
                if taken.len() == self.m {
                    if !bucket.is_empty() {
                        self.by_value.insert(value, bucket);
                    }
                    let sep = self
                        .by_value
                        .first_key_value()
                        .map(|(&v, _)| v)
                        .or(self.bound);
                    return (taken, sep);
                }
            }
        }
        let sep = self
            .by_value
            .first_key_value()
            .map(|(&v, _)| v)
            .or(self.bound);
        (taken, sep)
    }

    fn remove_from_bucket(&mut self, value: C, key: usize) {
        if let Some(bucket) = self.by_value.get_mut(&value) {
            if let Some(i) = bucket.iter().position(|&k| k == key) {
                bucket.swap_remove(i);
            }
            if bucket.is_empty() {
                self.by_value.remove(&value);
            }
        }
    }
}
