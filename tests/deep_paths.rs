//! Searches along long paths must not overflow the call stack.

use pathfinding::prelude::*;

/// Number of nodes on the path, far more than a recursive search can go through with
/// `STACK_SIZE` bytes of stack.
const DEPTH: u32 = 50_000;
const STACK_SIZE: usize = 256 * 1024;

fn on_small_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap()
}

#[test]
fn idastar_deep_path() {
    let result = on_small_stack(|| {
        idastar(
            &0,
            |&n| [(n + 1, 1)],
            |&n| DEPTH - n.min(DEPTH),
            |&n| n == DEPTH,
        )
    });
    let (path, cost) = result.unwrap();
    assert_eq!(cost, DEPTH);
    assert_eq!(path, (0..=DEPTH).collect::<Vec<_>>());
}
