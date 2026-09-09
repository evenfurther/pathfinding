use pathfinding::directed::count_paths::count_paths;

#[test]
fn grid() {
    let n = count_paths(
        (0, 0),
        |&(x, y)| {
            [(x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|&(x, y)| x < 8 && y < 8)
        },
        |&c| c == (7, 7),
    );
    assert_eq!(n, 3432);
}

#[test]
fn deep_graph_does_not_exhaust_the_stack() {
    // A single chain: one path, but formerly one stack frame per node. The thread is given a
    // deliberately small stack so that a return to a recursive walk fails here rather than
    // silently on someone else's machine.
    const N: usize = 200_000;
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(|| {
            let n = count_paths(0usize, |&n| (n + 1 < N).then_some(n + 1), |&n| n == N - 1);
            assert_eq!(n, 1);
        })
        .expect("cannot spawn thread")
        .join()
        .expect("counting exhausted the stack");
}

#[test]
fn counts_are_shared_between_paths() {
    // Every node of a diamond lattice is reachable by several routes; the count is only
    // correct, and only cheap, if each node is counted once and reused.
    let mut visited = 0;
    let n = count_paths(
        (0u32, 0u32),
        |&(x, y)| {
            visited += 1;
            [(x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|&(x, y)| x < 20 && y < 20)
        },
        |&c| c == (19, 19),
    );
    // Central binomial coefficient C(38, 19).
    assert_eq!(n, 35_345_263_800);
    // One expansion per node, not one per path.
    assert!(visited <= 400, "successors called {visited} times");
}

#[test]
#[should_panic(expected = "loop")]
fn a_loop_is_reported() {
    count_paths(0u32, |&n| [(n + 1) % 4], |_| false);
}
