# Draft issues and pull requests

Scratch file — not meant to be committed. Each section below is one issue and the pull
request that closes it. Branches are already created locally; nothing has been pushed.

Two of these are bug fixes that other work builds on, so a suggested order is given at the
end.

---

## 1. Test fixtures are corrupted by line-ending translation

**Branch:** `fix/crlf-test-fixtures`

### Issue

**Title:** `codejam-2017-a` tests fail on any checkout that translates line endings

**Body:**

On a checkout with `core.autocrlf` enabled — the default on Windows installs of Git for
Windows — both tests in `tests/codejam-2017-a.rs` fail immediately:

```
---- codejam_dense stdout ----
thread 'codejam_dense' panicked at tests\codejam-2017-a.rs:135:39:
cannot read number of test cases: Parse(ParseIntError { kind: InvalidDigit })
```

`tests/A-small-practice.in` is parsed byte for byte, and `tests/A-small-practice.out` is
compared against the produced output verbatim, so neither survives having its line endings
rewritten on checkout.

The tests pass as soon as both files are converted back to LF, so the code is fine; it is
only the checkout that is wrong.

### Pull request

**Title:** `fix(tests): stop CRLF translation from corrupting the codejam fixtures`

**Body:**

Closes #N.

Marks the two Code Jam fixtures as binary in `.gitattributes` so their bytes survive
checkout whatever the user's `core.autocrlf` setting is.

Only these two files are marked; everything else keeps the current behaviour.

---

## 2. `prim` does not always return a minimum spanning tree

**Branch:** `fix/prim-minimum-spanning-tree`

### Issue

**Title:** `prim` returns a non-minimal tree when an edge names the starting node second

**Body:**

`prim` seeds its priority queue with the edges whose **first** endpoint is the starting
node:

```rust
let mut priority_queue = edges
    .iter()
    .filter_map(|(n, n1, c)| (n == start).then_some(Reverse((c, n, n1))))
    .collect::<BinaryHeap<_>>();
```

Edges are undirected, so an edge touching the starting node may equally be written the
other way round. Those are never offered, and the main loop cannot recover them either: by
the time the other endpoint is visited, both of its checks require the far endpoint to be
unvisited, and the starting node is visited from the outset.

The starting node can therefore only ever be joined to the tree through edges that happen
to be written in one particular direction, and the result is not always minimal:

```rust
let edges = vec![(1, 0, 10), (2, 1, 1), (2, 0, 1)];
prim(&edges);                        // [(1, 0, 10), (0, 2, 1)]  weight 11
kruskal(&edges).collect::<Vec<_>>(); // [(2, 1, 1),  (2, 0, 1)]  weight 2
```

The three existing tests do not catch this because all of them happen to write the starting
node first.

### Pull request

**Title:** `fix(prim): use edges that name the starting node second`

**Body:**

Closes #N.

Seeds the queue with both orientations of the edges incident to the starting node.

Verified against `kruskal` over 200 random connected graphs whose edges are written in
either direction. The three existing tests are unaffected — they pin down the exact output
including tie-breaking, and it is unchanged.

---

## 3. `yen` reports inflated costs on graphs with parallel edges

**Branch:** `fix/yen-parallel-edge-cost`

### Issue

**Title:** `yen` charges a path for every parallel edge instead of the one it takes

**Body:**

The first path comes from `dijkstra_internal` and is costed correctly. Every subsequent
candidate has its cost recomputed by `make_cost`, which walks the path and adds up *every*
edge joining each pair of consecutive nodes:

```rust
for (n, c) in successors(&edge[0]) {
    if n == edge[1] {
        cost = cost + c;   // adds all parallel edges, not the one used
    }
}
```

Where a graph has more than one edge between the same pair of nodes, the reported cost is
larger than the path actually is. The inflated value is also what orders the candidate
against the others, so it can affect which paths are returned and in what order.

```rust
let successors = |&n: &char| -> Vec<(char, u32)> {
    match n {
        '0' => vec![('a', 1), ('b', 1)],
        'a' => vec![('c', 1)],
        'b' => vec![('c', 5)],
        'c' => vec![('d', 1), ('d', 9)],   // parallel edges
        _   => vec![],
    }
};
yen(&'0', successors, |&n| n == 'd', 2);
// ['0','b','c','d'] reported as costing 16; walking it costs 7
```

### Pull request

**Title:** `fix(yen): charge a spur path for one parallel edge, not all of them`

**Body:**

Closes #N.

Takes the cheapest edge between two consecutive nodes, which is the one a shortest path
uses, instead of the sum of all of them.

Simple graphs are unaffected: there is only ever one edge to choose from. Verified against
a brute-force enumeration of every loopless path over 150 random multigraphs.

---

## 4. `strongly_connected_components` overflows the stack

**Branch:** `fix/scc-stack-overflow`

### Issue

**Title:** `strongly_connected_components` overflows the stack on ordinary-sized graphs

**Body:**

`recurse_onto` descends one stack frame per node, so the recursion depth is the depth of
the graph. This is reached easily:

```rust
const N: usize = 200_000;
let nodes = (0..N).collect::<Vec<_>>();
strongly_connected_components(&nodes, |&i| (i + 1 < N).then_some(i + 1));
// thread has overflowed its stack
```

A random graph of 60 000 nodes with average degree 4 is also enough. There is no way for a
caller to work around this other than by running the call on a thread with a hand-sized
stack, which is not something the API suggests is necessary.

While looking at this, the traversal also repeats work on every edge: `scca` and
`preorders` are consulted separately to answer two halves of the same question, `p` holds
cloned nodes and looks their preorder numbers back up on each pop, and
`strongly_connected_components` picks its next unvisited node by scanning a hash map it is
emptying as it goes.

### Pull request

**Title:** `perf(scc): traverse iteratively and look each successor up once`

**Body:**

Closes #N.

Replaces the recursion with an explicit stack, so depth is bounded by memory rather than by
the thread's stack, and removes the repeated lookups described in the issue. A node whose
component has been emitted is marked in `preorders` itself, `p` holds preorder numbers
rather than nodes, and the top-level loop walks the caller's slice.

One visible consequence: the order of the returned components now follows the order of the
`nodes` argument instead of hash iteration order. It was never specified, but it is at
least deterministic now.

Correctness is checked against mutual reachability, for every pair of nodes, over 100
random graphs. A regression test runs a 200 000 node chain on a deliberately small 1 MiB
stack.

About 64% off a 60 000 node graph of degree 4.

---

## 5. `topological_sort` overflows the stack

**Branch:** `fix/topological-sort-stack-overflow`

### Issue

**Title:** `topological_sort` overflows the stack on deep graphs

**Body:**

Same shape of problem as the previous issue. `visit` recurses once per node:

```rust
const N: usize = 200_000;
let nodes = (0..N).collect::<Vec<_>>();
topological_sort(&nodes, |&i| (i + 1 < N).then_some(i + 1));
// thread has overflowed its stack
```

A random DAG of 60 000 nodes is also enough.

Separately, the bookkeeping does the same work more than once. `marked` and `temp` are
nested — a node is in `temp` once reached and in `marked` once finished — so every node is
hashed into both, and the outer loop repeatedly takes an arbitrary key out of a `HashSet`
that `visit` is emptying, rescanning the buckets already vacated. Both sets also use the
default hasher rather than the crate's.

### Pull request

**Title:** `perf(topological_sort): traverse iteratively and drop the redundant sets`

**Body:**

Closes #N.

Replaces the recursion with an explicit stack, merges `marked` and `temp` into one map from
node to "finished?", and walks the caller's slice instead of draining a set of roots.
`topological_sort_into_groups` keeps its structure and only changes hasher.

`successors` is still called exactly once per node — the existing `complexity` test covers
this. Orders are checked edge by edge against 200 random DAGs, and a regression test runs a
200 000 node chain on a 1 MiB stack.

On 60 000 nodes: about 76% off `topological_sort`, about 33% off
`topological_sort_into_groups`.

---

## 6. Bron-Kerbosch runs without a pivot

**Branch:** `perf/cliques-pivot`

### Issue

**Title:** `maximal_cliques` rediscovers each clique through many orderings

**Body:**

`bron_kerbosch` branches on every remaining candidate in turn. Without a pivot, a clique of
k vertices is reached through many of the orders its vertices can be visited in, and each
branch clones the candidate set and allocates two more hash sets on the way down. This is
the difference between the algorithm's stated bound and something much worse: on a
130-vertex random graph with edge probability 0.5, `maximal_cliques_collect` takes about
210 ms.

The pivoting rule is part of the original algorithm and is what the Wikipedia article
linked from the module documentation describes as the version to use.

### Pull request

**Title:** `perf(cliques): pivot in Bron-Kerbosch instead of branching on every candidate`

**Body:**

Closes #N.

Adds the pivot: a maximal clique either leaves the pivot out or contains one of its
neighbours, so the candidates adjacent to the pivot are not branched on at this level and
are reached through a deeper call instead. The same cliques come out, in a different order.

The three working sets become vectors — they are only filtered and scanned, never looked up
by key, and shrink quickly with depth.

One thing worth review: `connected` is a caller-supplied predicate and may report a vertex
as connected to itself. The previous code was indifferent to that; pivoting is not, because
a pivot that excluded itself from its own branch set could drop a candidate that is then
never branched on. A vertex is therefore never treated as its own neighbour. Both
conventions are covered by the tests.

Checked exhaustively against a brute-force enumeration of all 2^n subsets over 60 random
graphs.

About 88% off a 130-vertex graph with edge probability 0.5.

---

## 7. Fringe search rescans the fringe on every relaxed edge

**Branch:** `perf/fringe-lazy-deletion`

### Issue

**Title:** `fringe` is quadratic in the size of the fringe

**Body:**

When a node is reached by a cheaper path it must move back to the front of the current
list. `remove` finds it by walking `later` and then `now` from one end:

```rust
fn remove<T: Eq>(v: &mut VecDeque<T>, e: &T) -> bool {
    v.iter().position(|x| x == e).is_some_and(|index| { v.remove(index); true })
}
```

So every relaxed edge costs a pass over the fringe, and the fringe is largest for exactly
the searches where performance matters.

### Pull request

**Title:** `perf(fringe): stop scanning the whole fringe on every relaxed edge`

**Body:**

Closes #N.

Leaves stale entries in place rather than hunting for them. Each queue entry carries the
value `stamps[node]` held when it was queued; relaxing a node bumps its stamp, so older
entries are recognised and skipped when they surface. The node is still pushed to the front,
so it is still reached first, and the work per edge drops to a push.

The search itself is unchanged — the same nodes are expanded under the same limits. Paths
are checked against Bellman-Ford over 300 random graphs.

About 42% off a 200x200 weighted grid. The gap widens with the size of the fringe.

---

## 8. Edmonds-Karp repeats work it has already done

**Branch:** `perf/edmonds-karp-redundant-work`

### Issue

**Title:** `edmonds_karp` builds the flow list twice and clones the sparse map to read it

**Body:**

Three things in `augment` and below it are done more than once:

- `augment` calls `flows()` to derive the cut, then calls it again for the value it
  returns. Building that list walks the whole capacity structure — for `DenseCapacity`, the
  entire size-by-size matrix.
- `SparseCapacity::flows` clones the whole nested `BTreeMap` before iterating it,
  allocating a copy of every flow in order to read every flow.
- `update_flows` records the nodes still reachable from the source in a `BTreeSet<usize>`,
  cleared and refilled on every augmentation, although the nodes are numbered from zero and
  a `Vec<bool>` would index straight in.

`cancel_flow` also clones a path to zip it against itself.

### Pull request

**Title:** `perf(edmonds-karp): stop repeating work that is already done`

**Body:**

Closes #N.

Builds the flow list once and reads the cut off it, iterates the sparse map by reference,
indexes reachability by node number, and uses `windows(2)` for the path pairs.

`update_flows` changes return type, but it belongs to the private `EdmondsKarpInternal`
trait and does not escape the module.

No change to the flows, the value or the cut, all of which the existing tests check exactly,
including the two Code Jam ones.

On a layered 122-node network: about 6% off the dense representation, about 16% off the
sparse one, which does the most copying.

---

## 9. Per-node hashing and allocation in the search algorithms

**Branch:** `perf/search-per-node-allocations`

### Issue

**Title:** Several searches keep hash structures whose answers are already available

**Body:**

- `astar_bag` gives every node a `HashSet<usize>` of optimal parents and collects the goals
  into another. A node has as many optimal parents as it has incoming edges, so these are
  tiny sets that are only cleared, appended to and walked — one table allocation and a hash
  per node, for nothing a vector would not do.
- `dijkstra_bidirectional` keeps an `FxHashSet<N>` of settled nodes per direction. An entry
  is only queued when it strictly improves on the node's recorded cost, so the costs queued
  for a node strictly decrease and the existing `cost > c` check already rejects every
  stale entry. The sets cost a clone of every settled node plus two hashes, and cannot
  reject anything the check does not.
- `dijkstra_reach` hashes dense node indices into an `FxHashSet<usize>`.
- `dfs_reach` builds a temporary vector per visited node just to reverse its successors.

### Pull request

**Title:** `perf: drop per-node hashing and allocation the searches do not need`

**Body:**

Closes #N.

Vectors for the `astar_bag` parent sets, no settled sets in `dijkstra_bidirectional`, a
`Vec<bool>` for `dijkstra_reach`, and an in-place reverse for `dfs_reach`.

The part worth review is `dijkstra_bidirectional`. Dropping the settled sets also removes
the question they answered — whether the opposite side had settled the popped node — so
termination moves to the standard criterion of stopping once the two frontier costs sum to
the best path found, which needs no lookup at all. The meeting node is recorded as a pair
of indices, so rebuilding the path does not look it up either. Paths and costs are checked
against Bellman-Ford over 300 random graphs.

```
dfs_reach, 300x300 grid:                 about 30% off
astar_bag, 120x120 grid:                 about 22% off
dijkstra_bidirectional, 500x500 grid:    about 15% off
dijkstra_reach, 300x300 weighted grid:   about  9% off
```

`dijkstra` and `astar` themselves are untouched and measure unchanged.

---

## 10. Containers copied only to be walked

**Branch:** `perf/redundant-copies`

### Issue

**Title:** `components` clones every group, and `kuhn_munkres` hashes a list it never looks up

**Body:**

`ConnectedComponents::components` clones each group in full in order to iterate it by
value:

```rust
for e in groups[i].clone() {
    set.insert(e);
}
```

The impl already requires `&It: IntoIterator`, so the group can be borrowed and only the
elements actually kept need cloning.

Separately, `kuhn_munkres` holds the x nodes of the alternating path in an `FxIndexSet`,
but each root adds a given node at most once and the set is only ever cleared, appended to
and iterated — nothing ever looks a node up.

### Pull request

**Title:** `perf: stop copying containers that are only walked`

**Body:**

Closes #N.

Borrows the group in `components`, and uses a `Vec` in `kuhn_munkres`.

About 8% off `components` over 20 000 groups of 8. The `kuhn_munkres` half is a
simplification rather than a speed-up — it does not measure as a change on a 250x250
assignment — and can be dropped from this PR if you would rather keep it to one thing.

---

## 11. Prim rescans the whole edge list per node

**Branch:** `perf/prim-adjacency` — **contains the fix from #2; open after it, or as one PR**

### Issue

**Title:** `prim` is O(V*E) before the queue does any work

**Body:**

Each time a node joins the tree, the loop walks the entire edge list looking for the edges
that touch it:

```rust
for (n2, n3, c) in edges {
    if n1 == n2 && !visited.contains(n3) { ... }
    else if n1 == n3 && !visited.contains(n2) { ... }
}
```

Almost every iteration examines an edge touching neither endpoint. Building an MST
therefore costs one pass over the edges per node added, before any heap work. On a
1500-node, 9500-edge graph this is about 7.5 ms, against roughly 1 ms for `kruskal` on a
graph an order of magnitude larger.

### Pull request

**Title:** `perf(prim): index the edges by endpoint instead of rescanning them`

**Body:**

Closes #N.

Indexes the edges by endpoint once, up front, and offers only the edges leaving the node
just added.

The same candidates reach the queue in the same order, so the tree and the order of its
edges are unchanged, including the tie-breaking the existing tests pin down. Checked against
`kruskal` over 200 random connected graphs.

About 86% off a 1500-node, 9500-edge graph, and the saving grows with the edge count since
the work per node no longer depends on it.

> Built on the fix for #(prim MST bug), which is included here as the first commit.

---

## 12. Yen recomputes spur paths an ancestor already produced

**Branch:** `perf/yen-lawler` — **contains the fix from #3; open after it, or as one PR**

### Issue

**Title:** `yen` re-runs spur searches whose candidates were already generated and rejected

**Body:**

Every accepted route is spurred at each of its nodes in turn. But a route coincides with
the one it was derived from up to the point where it deviates, and for any spur index before
that point the root path and the set of banned edges are the same as they were for the
parent. The search runs again and produces a candidate that has already been generated.

For k = 20 on a 60x60 grid the paths are around 118 nodes long, so this is on the order of
two thousand Dijkstra runs, a large fraction of them redundant.

`make_cost` also re-walks the graph for every candidate to recompute a cost the search
already knows.

### Pull request

**Title:** `perf(yen): skip the deviations an ancestor has already produced`

**Body:**

Closes #N.

Records where each route deviated from its parent and starts spurring from there — Lawler's
refinement of Yen's algorithm.

The candidate cost no longer needs `make_cost` either. Filtering removes whole node pairs
rather than individual parallel edges, so the cheapest edge between two surviving nodes is
the same in the filtered graph as in the full one, and the root cost plus the cost the
search reports for the spur is exactly the cost of the concatenation. Prefix costs are
computed once per route instead of once per candidate.

Checked against a brute-force enumeration of every loopless path over 150 random graphs,
comparing the k cheapest costs.

About 39% off twenty shortest paths across a 60x60 weighted grid.

> Built on the fix for #(yen parallel edge bug), which is included here as the first commit.

---

## 13. Benchmarks are too small to measure with

**Branch:** `bench/large-inputs`

### Issue

**Title:** The benchmark suite runs on inputs too small to tell a change from noise

**Body:**

`algos.rs` searches a 65x65 grid. Several of the algorithms clear that in well under a
millisecond, which is small enough that ordinary run-to-run variance covers most differences
worth making, and it never exercises the behaviour that only shows up at scale — deep
recursion, large fringes, hash maps big enough to miss cache.

Yen, cliques, Prim, strongly connected components, topological sort and Edmonds-Karp have
no criterion coverage at all outside the iai benches.

### Pull request

**Title:** `bench: add a suite on inputs large enough to measure`

**Body:**

Closes #N.

Adds `benches/large.rs`, covering the same ground as `algos.rs` on inputs from ten thousand
to a quarter of a million nodes, plus the algorithms the existing suites do not reach.

Also adds `ab.sh`, which drives it. Comparing against a criterion baseline saved earlier in
a session does not work: the machine drifts, and whichever binary runs second is measured
warm — both worth more than the differences being looked for. It builds the baseline commit
in a worktree so that both versions exist at once, runs them back to back, and swaps the
order every other pass, so a real change keeps its sign in both directions and a thermal
artefact does not.

Worth knowing before trusting any single figure: code placement alone moves individual
benchmarks by over 10% here. Editing one function can shift an unrelated one by that much,
stably across repeated passes and in both orders. The script's header says so.

---

## 14. The tests check behaviour but not correctness

**Branch:** `test/random-graph-differential` — **contains the fixes from #2 and #3**

### Issue

**Title:** Nothing checks the graph algorithms against an independent implementation

**Body:**

The suite covers each algorithm against hand-written cases. Those are good at catching a
change in behaviour and poor at catching a wrong answer: they are all simple graphs, most
are small, and they were written from the same reasoning as the code they check.

Two bugs presently in the tree — the `prim` and `yen` ones — are invisible to all of them,
because no test anywhere in the suite has two edges between the same pair of nodes.

### Pull request

**Title:** `test: check the graph algorithms against reference implementations`

**Body:**

Closes #N.

Adds `tests/random_graphs.rs`. Random graphs are generated with parallel edges and
self-loops, and each algorithm is checked against something obviously correct rather than
against itself:

- shortest paths (`dijkstra`, `dijkstra_bidirectional`, `astar`, `astar_bag`, `fringe`,
  `dijkstra_all`, `dijkstra_reach`, and `bfs`/`bfs_bidirectional` at unit cost) against
  Bellman-Ford, with every returned path re-walked in the graph to confirm it is a real
  loopless walk of the announced cost
- `bfs_reach` and `dfs_reach` against the reachable set, each node yielded exactly once
- `strongly_connected_components` against mutual reachability for every pair of nodes
- `topological_sort` and `topological_sort_into_groups` against every edge
- `maximal_cliques_collect` against a brute-force enumeration of all 2^n subsets, with
  `connected` reflexive on half the rounds and irreflexive on the other half
- `prim` against `kruskal`
- `yen` against a brute-force enumeration of every loopless path

Everything is seeded, so failures reproduce.

This is what found the two bugs.

> Depends on the `prim` and `yen` fixes; both are included here as the first two commits and
> will drop out once those land.

---

## Suggested order

Independent of everything, can go any time:

- 1 (CRLF fixtures), 13 (benchmarks)

Bug fixes, worth landing first:

- 2 (prim MST), 3 (yen cost), 4 (SCC stack), 5 (topological sort stack)

Then, in any order:

- 6 (cliques pivot), 7 (fringe), 8 (Edmonds-Karp), 9 (search allocations), 10 (copies)

Then the two that build on the fixes, and the tests:

- 11 (prim adjacency, after 2), 12 (yen Lawler, after 3), 14 (differential tests, after 2
  and 3)

If fourteen is too many to open at once, 11 and 12 can be folded into 2 and 3, and 4 and 5
can go together as one "make the recursive traversals iterative" PR, which brings it to
eleven.
