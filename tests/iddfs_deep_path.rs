//! A search along a long path must not overflow the call stack.

use pathfinding::prelude::iddfs;

#[test]
fn iddfs_deep_path() {
    // Far more nodes than a recursive search can go through with a 16 KiB stack. Each
    // iteration explores the whole path again, so this is kept smaller than for other searches.
    const DEPTH: u32 = 1_000;
    let path = std::thread::Builder::new()
        .stack_size(16 * 1024)
        .spawn(|| iddfs(0, |&n| [n + 1], |&n| n == DEPTH))
        .unwrap()
        .join()
        .unwrap();
    assert_eq!(path, Some((0..=DEPTH).collect()));
}
