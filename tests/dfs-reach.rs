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
