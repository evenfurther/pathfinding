use pathfinding::directed::dfs::dfs_reach;

#[test]
fn issue_511() {
    let it = dfs_reach(0, |&n| [n + 1, n + 5].into_iter().filter(|&x| x <= 10));
    assert_eq!(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        it.collect::<Vec<_>>()
    );
}

#[test]
fn issue_511_branches() {
    let it = dfs_reach(0, |&n| [n + 2, n + 5].into_iter().filter(|&x| x <= 10));
    assert_eq!(vec![0, 2, 4, 6, 8, 10, 9, 7, 5], it.collect::<Vec<_>>());
}

/// Test that `dfs_reach` does not stack overflow when many duplicate nodes
/// pile up in the `to_see` stack (would previously recurse for each duplicate).
#[test]
fn no_stack_overflow_with_duplicates() {
    // Each node has N successors all pointing to the same next node, creating
    // many duplicates in to_see. With the old recursive implementation, the
    // recursion depth could equal the number of duplicates, causing stack overflow.
    let n = 200_usize;
    // Node 0 -> [1, 1, 1, ...] (n copies of 1)
    // Node k -> [k+1, k+1, k+1, ...] (n copies of k+1) for k < n
    // Node n -> []
    let result: Vec<usize> =
        dfs_reach(0usize, |&k| if k < n { vec![k + 1; n] } else { vec![] }).collect();
    let expected: Vec<usize> = (0..=n).collect();
    assert_eq!(result, expected);
}
