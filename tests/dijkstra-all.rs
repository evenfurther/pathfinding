use pathfinding::prelude::*;
use rand::{rngs, Rng as _};

fn build_network(size: usize) -> Matrix<usize> {
    let mut network = Matrix::new(size, size, 0);
    let mut rng = rngs::OsRng;
    for a in 0..size {
        for b in 0..size {
            if rng.gen_ratio(2, 3) {
                network[(a, b)] = rng.r#gen::<u16>() as usize;
            }
        }
    }
    network
}

fn neighbours(network: Matrix<usize>) -> impl FnMut(&usize) -> Vec<(usize, usize)> {
    move |&a| {
        (0..network.rows)
            .filter_map(|b| match network[(a, b)] {
                0 => None,
                p => Some((b, p)),
            })
            .collect()
    }
}

#[test]
fn all_paths() {
    const SIZE: usize = 30;
    let network = build_network(SIZE);
    for start in 0..SIZE {
        let paths = dijkstra_all(&start, neighbours(network.clone()));
        for target in 0..SIZE {
            if let Some((path, cost)) =
                dijkstra(&start, neighbours(network.clone()), |&n| n == target)
            {
                if start == target {
                    assert!(
                        !paths.contains_key(&target),
                        "path {start} -> {target} is present in {network:?}"
                    );
                } else {
                    assert!(
                        paths.contains_key(&target),
                        "path {start} -> {target} is not found in {network:?}"
                    );
                    assert_eq!(
                        cost, paths[&target].1,
                        "cost differ in path {start} -> {target} in {network:?}"
                    );
                    let other_path = build_path(&target, &paths);
                    // There might be several paths, but we know that internally we use the
                    // same algorithm so the comparison holds.
                    assert_eq!(path, other_path, "path {start} -> {target} differ in {network:?}: {path:?} vs {other_path:?}");
                }
            } else {
                assert!(
                    !paths.contains_key(&target),
                    "path {start} -> {target} is present in {network:?}"
                );
            }
        }
    }
}

#[test]
fn partial_paths() {
    const SIZE: usize = 100;
    let network = build_network(SIZE);
    for start in 0..SIZE {
        let (paths, reached) = dijkstra_partial(&start, neighbours(network.clone()), |&n| {
            start != 0 && n != 0 && n != start && n % start == 0
        });
        if let Some(target) = reached {
            assert!(target % start == 0, "bad stop condition");
            // We cannot compare other paths since there is no guarantee that the
            // paths variable is up-to-date as the algorithm stopped prematurely.
            let cost = paths[&target].1;
            let (path, dijkstra_cost) =
                dijkstra(&start, neighbours(network.clone()), |&n| n == target).unwrap();
            assert_eq!(
                cost, dijkstra_cost,
                "costs {start} -> {target} differ in {network:?}"
            );
            let other_path = build_path(&target, &paths);
            // There might be several paths, but we know that internally we use the
            // same algorithm so the comparison holds.
            assert_eq!(
                path, other_path,
                "path {start} -> {target} differ in {network:?}: {path:?} vs {other_path:?}"
            );
        } else if start != 0 && start <= (SIZE - 1) / 2 {
            for target in 1..(SIZE / start) {
                assert!(
                    dijkstra(&start, neighbours(network.clone()), |&n| n == target).is_none(),
                    "path {start} -> {target} found in {network:?}"
                );
            }
        }
    }
}
