mod ex1 {
    use lazy_static::lazy_static;
    use pathfinding::prelude::*;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn successors(node: &u8) -> impl Iterator<Item = (u8, usize)> {
        lazy_static! {
            static ref SUCCESSORS: Vec<Vec<(u8, usize)>> = vec![
                vec![(1, 7), (2, 7), (3, 6)],
                vec![(0, 8), (6, 7)],
                vec![(5, 7)],
                vec![(7, 7)],
                vec![(4, 2)],
                vec![(1, 1)],
                vec![(2, 5), (4, 5), (5, 2)],
                vec![(5, 8)],
                vec![],
            ];
        }
        SUCCESSORS[*node as usize].iter().copied()
    }

    fn expected(target: u8) -> Option<(Vec<u8>, usize)> {
        match target {
            0 => Some((vec![1, 0], 8)),
            1 => Some((vec![1], 0)),
            2 => Some((vec![1, 6, 2], 12)),
            3 => Some((vec![1, 0, 3], 14)),
            4 => Some((vec![1, 6, 4], 12)),
            5 => Some((vec![1, 6, 5], 9)),
            6 => Some((vec![1, 6], 7)),
            7 => Some((vec![1, 0, 3, 7], 21)),
            8 => None,
            _ => panic!("no such node"),
        }
    }

    #[test]
    fn dijkstra_ok() {
        for target in 0..9 {
            assert_eq!(
                dijkstra(&1, successors, |&node| node == target),
                expected(target)
            );
        }
    }

    #[test]
    fn fringe_ok() {
        for target in 0..9 {
            assert_eq!(
                fringe(&1, successors, |_| 0, |&node| node == target),
                expected(target)
            );
        }
    }

    #[test]
    fn dfs_ok() {
        for target in 0..9 {
            match dfs(1, |n| successors(n).map(|(v, _)| v), |&node| node == target) {
                None => assert_eq!(expected(target), None, "path not found"),
                Some(path) => assert!(
                    expected(target).expect("non-existing path found").0.len() <= path.len()
                ),
            }
        }
    }

    #[test]
    fn djkstra_loop_ok() {
        assert_eq!(dijkstra(&1, |_| vec![(1, 1)], |&n| n == 2), None);
    }

    #[test]
    fn dfs_loop_ok() {
        assert_eq!(dfs(1, |_| vec![1], |&n| n == 2), None);
    }

    #[test]
    fn bfs_loops() {
        let successors = |n: &u8| successors(n).map(|(n, _)| n);
        assert_eq!(bfs_loop(&0, successors), Some(vec![0, 1, 0]));
        assert_eq!(bfs_loop(&1, successors), Some(vec![1, 0, 1]));
        assert_eq!(bfs_loop(&2, successors), Some(vec![2, 5, 1, 0, 2]));
        assert_eq!(bfs_loop(&8, successors), None);
    }

    #[test]
    fn bfs_reach_is_fused() {
        let mut it = bfs_reach(1, |&n| vec![n * 2, n * 3].into_iter().filter(|&x| x < 15)).skip(1);
        for _ in 0..7 {
            assert!(it.next().is_some());
        }
        for _ in 0..3 {
            assert!(it.next().is_none());
        }
    }

    #[test]
    fn bfs_remaining_nodes() {
        let mut it = bfs_reach(1, |&n| vec![n * 2, n * 3].into_iter().filter(|&x| x < 15));
        assert_eq!(1, it.remaining_nodes_low_bound()); // [1]
        assert_eq!(Some(1), it.next());
        assert_eq!(2, it.remaining_nodes_low_bound()); // [2, 3]
        assert_eq!(Some(2), it.next());
        assert_eq!(3, it.remaining_nodes_low_bound()); // [3, 4, 6]
        assert_eq!(Some(3), it.next());
        assert_eq!(3, it.remaining_nodes_low_bound()); // [4, 6, 9]
        assert_eq!(Some(4), it.next());
        assert_eq!(4, it.remaining_nodes_low_bound()); // [6, 9, 8, 12]
        assert_eq!(Some(6), it.next());
        assert_eq!(3, it.remaining_nodes_low_bound()); // [9, 8, 12]
        assert_eq!(Some(9), it.next());
        assert_eq!(2, it.remaining_nodes_low_bound()); // [8, 12]
        assert_eq!(Some(8), it.next());
        assert_eq!(1, it.remaining_nodes_low_bound()); // [12]
        assert_eq!(Some(12), it.next());
        assert_eq!(0, it.remaining_nodes_low_bound()); // []
        assert_eq!(None, it.next());
    }

    #[test]
    fn dfs_reach_is_fused() {
        let mut it = dfs_reach(1, |&n| vec![n * 2, n * 3].into_iter().filter(|&x| x < 15)).skip(1);
        for _ in 0..7 {
            assert!(it.next().is_some());
        }
        for _ in 0..3 {
            assert!(it.next().is_none());
        }
    }

    #[test]
    fn dfs_remaining_nodes() {
        let mut it = dfs_reach(1, |&n| vec![n * 2, n * 3].into_iter().filter(|&x| x < 15));
        assert_eq!(1, it.remaining_nodes_low_bound()); // [1]
        assert_eq!(Some(1), it.next());
        assert_eq!(2, it.remaining_nodes_low_bound()); // [2, 3]
        assert_eq!(Some(2), it.next());
        assert_eq!(3, it.remaining_nodes_low_bound()); // [4, 6, 3]
        assert_eq!(Some(4), it.next());
        assert_eq!(4, it.remaining_nodes_low_bound()); // [8, 12, 6, 3]
        assert_eq!(Some(8), it.next());
        assert_eq!(3, it.remaining_nodes_low_bound()); // [12, 6, 3]
        assert_eq!(Some(12), it.next());
        assert_eq!(2, it.remaining_nodes_low_bound()); // [6, 3]
        assert_eq!(Some(6), it.next());
        assert_eq!(1, it.remaining_nodes_low_bound()); // [3]
        assert_eq!(Some(3), it.next());
        assert_eq!(1, it.remaining_nodes_low_bound()); // [9]
        assert_eq!(Some(9), it.next());
        assert_eq!(0, it.remaining_nodes_low_bound()); // []
        assert_eq!(None, it.next());
    }
}

mod ex2 {
    use lazy_static::lazy_static;
    use pathfinding::prelude::*;

    const MAZE: &str = "\
#########
#.#.....#
###.##..#
#...#...#
#...#...#
#...#...#
#...#...#
#########
";

    lazy_static! {
        static ref OPEN: Vec<Vec<bool>> = MAZE
            .lines()
            .map(|l| l.chars().map(|c| c == '.').collect())
            .collect();
    }

    fn successors(&(x, y): &(usize, usize)) -> Vec<((usize, usize), usize)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(|(nx, ny)| OPEN[ny][nx].then_some(((nx, ny), 1)))
            .collect()
    }

    const fn distance(&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> usize {
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }

    #[test]
    fn astar_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let mut counter = 0;
        let (path, cost) = astar(
            &(2, 3),
            |n| {
                counter += 1;
                successors(n)
            },
            |n| distance(n, &GOAL),
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(counter, 11);
    }

    #[test]
    fn astar_bag_path_single_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let mut counter = 0;
        let (paths, cost) = astar_bag_collect(
            &(2, 3),
            |n| {
                counter += 1;
                successors(n)
            },
            |n| distance(n, &GOAL),
            |n| n == &GOAL,
        )
        .unwrap();
        assert_eq!(cost, 8);
        assert_eq!(paths.len(), 1);
        assert!(paths
            .iter()
            .all(|path| path.iter().all(|&(nx, ny)| OPEN[ny][nx])));
        assert_eq!(counter, 15);
    }

    #[test]
    fn astar_bag_path_multiple_ok() {
        const GOAL: (usize, usize) = (7, 3);
        let mut counter = 0;
        let (paths, cost) = astar_bag_collect(
            &(2, 3),
            |n| {
                counter += 1;
                successors(n)
            },
            |n| distance(n, &GOAL),
            |n| n == &GOAL,
        )
        .unwrap();
        assert_eq!(cost, 9);
        assert_eq!(paths.len(), 3);
        assert!(paths
            .iter()
            .all(|path| path.iter().all(|&(nx, ny)| OPEN[ny][nx])));
        assert_eq!(counter, 18);
    }

    #[test]
    fn astar_bag_iter_is_fused() {
        const GOAL: (usize, usize) = (7, 3);
        let (mut it, _) =
            astar_bag(&(2, 3), successors, |n| distance(n, &GOAL), |n| n == &GOAL).unwrap();
        for _ in 0..3 {
            assert!(it.next().is_some());
        }
        for _ in 0..3 {
            assert!(it.next().is_none());
        }
    }

    #[test]
    fn idastar_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let mut counter = 0;
        let (path, cost) = idastar(
            &(2, 3),
            |n| {
                counter += 1;
                successors(n)
            },
            |n| distance(n, &GOAL),
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(counter, 18);
    }

    #[test]
    fn fringe_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let mut counter = 0;
        let (path, cost) = fringe(
            &(2, 3),
            |n| {
                counter += 1;
                successors(n)
            },
            |n| distance(n, &GOAL),
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(counter, 14);
    }

    #[test]
    fn dijkstra_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let mut counter = 0;
        let (path, cost) = dijkstra(
            &(2, 3),
            |n| {
                counter += 1;
                successors(n)
            },
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(counter, 20);
    }

    #[test]
    fn bfs_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let path = bfs(
            &(2, 3),
            |n| successors(n).into_iter().map(|(n, _)| n),
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert_eq!(path.len(), 9);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
    }

    #[test]
    fn bfs_bidirectional_path_ok() {
        static SUCCESSORS: fn(&(usize, usize)) -> Vec<(usize, usize)> =
            |n| successors(n).into_iter().map(|(n, _)| n).collect();

        let path =
            bfs_bidirectional(&(2, 3), &(6, 3), SUCCESSORS, SUCCESSORS).expect("path not found");

        assert_eq!(path.len(), 9);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
    }

    #[test]
    fn dfs_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let path = dfs(
            (2, 3),
            |n| successors(n).into_iter().map(|(n, _)| n),
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert!(path.len() >= 9);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
    }

    #[test]
    fn iddfs_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let path = iddfs(
            (2, 3),
            |n| successors(n).into_iter().map(|(n, _)| n),
            |n| n == &GOAL,
        )
        .expect("path not found");
        assert_eq!(path.len(), 9);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
    }

    #[test]
    fn astar_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            astar(&(2, 3), successors, |n| distance(n, &GOAL), |n| n == &GOAL),
            None
        );
    }

    #[test]
    fn idastar_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            idastar(&(2, 3), successors, |n| distance(n, &GOAL), |n| n == &GOAL),
            None
        );
    }

    #[test]
    fn fringe_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            fringe(&(2, 3), successors, |n| distance(n, &GOAL), |n| n == &GOAL),
            None
        );
    }

    #[test]
    fn dijkstra_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(dijkstra(&(2, 3), successors, |n| n == &GOAL), None);
    }

    #[test]
    fn bfs_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            bfs(
                &(2, 3),
                |n| successors(n).into_iter().map(|(n, _)| n),
                |n| n == &GOAL,
            ),
            None
        );
    }

    #[test]
    fn bfs_bidirectional_no_path() {
        static SUCCESSORS: fn(&(usize, usize)) -> Vec<(usize, usize)> =
            |n| successors(n).into_iter().map(|(n, _)| n).collect();

        assert_eq!(
            bfs_bidirectional(&(2, 3), &(1, 1), SUCCESSORS, SUCCESSORS),
            None
        );
    }

    #[test]
    fn dfs_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            dfs(
                (2, 3),
                |n| successors(n).into_iter().map(|(n, _)| n),
                |n| n == &GOAL
            ),
            None
        );
    }

    #[test]
    fn iddfs_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            iddfs(
                (2, 3),
                |n| successors(n).into_iter().map(|(n, _)| n),
                |n| n == &GOAL,
            ),
            None
        );
    }
}
