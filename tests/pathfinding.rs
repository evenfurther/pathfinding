#[macro_use]
extern crate lazy_static;
extern crate pathfinding;

mod ex1 {

    use pathfinding::*;

    fn neighbours(node: &u8) -> Vec<(u8, usize)> {
        lazy_static! {
            static ref NEIGHBOURS: Vec<Vec<(u8, usize)>> = vec![
            vec![(1, 7), (2, 7), (3, 6)],
            vec![(0, 8), (6, 7)],
            vec![(5, 7)],
            vec![(7, 7)],
            vec![(4, 2)],
            vec![(1, 1)],
            vec![(2, 5), (4, 5), (5, 2)],
            vec![(5, 8)],
            vec![]];
        }
        NEIGHBOURS[*node as usize].clone()
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
                dijkstra(&1, neighbours, |&node| node == target),
                expected(target)
            );
        }
    }

    #[test]
    fn fringe_ok() {
        for target in 0..9 {
            assert_eq!(
                fringe(&1, neighbours, |_| 0, |&node| node == target),
                expected(target)
            );
        }
    }

    #[test]
    fn dfs_ok() {
        for target in 0..9 {
            match dfs(1, |n| neighbours(n).into_iter().map(|(v, _)| v), |&node| {
                node == target
            }) {
                None => assert_eq!(expected(target), None, "path not found"),
                Some(path) => {
                    assert!(
                        expected(target).expect("non-existant path found").0.len() <= path.len()
                    )
                }
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
}

mod ex2 {

    use pathfinding::*;
    use std::cell::RefCell;

    const MAZE: &'static str = "\
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
        static ref OPEN: Vec<Vec<bool>> =
            MAZE.lines().map(|l| l.chars().map(|c| c == '.').collect()).collect();
    }

    fn neighbours(&(x, y): &(usize, usize)) -> Vec<((usize, usize), usize)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(|(nx, ny)| if OPEN[ny][nx] {
                Some(((nx, ny), 1))
            } else {
                None
            })
            .collect()
    }

    macro_rules! absdiff { ($a:expr, $b:expr) => { if $a >= $b { $a - $b } else { $b - $a } } }

    fn distance(&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> usize {
        absdiff!(x1, x2) + absdiff!(y1, y2)
    }

    #[test]
    fn astar_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let counter = RefCell::new(0);
        let neighbours_counter = |n: &(usize, usize)| {
            *counter.borrow_mut() += 1;
            neighbours(n)
        };
        let (path, cost) = astar(&(2, 3), neighbours_counter, |n| distance(n, &GOAL), |n| {
            n == &GOAL
        }).expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(*counter.borrow(), 14);
    }

    #[test]
    fn fringe_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let counter = RefCell::new(0);
        let neighbours_counter = |n: &(usize, usize)| {
            *counter.borrow_mut() += 1;
            neighbours(n)
        };
        let (path, cost) = fringe(&(2, 3), neighbours_counter, |n| distance(n, &GOAL), |n| {
            n == &GOAL
        }).expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(*counter.borrow(), 14);
    }

    #[test]
    fn dijkstra_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let counter = RefCell::new(0);
        let neighbours_counter = |n: &(usize, usize)| {
            *counter.borrow_mut() += 1;
            neighbours(n)
        };
        let (path, cost) = dijkstra(&(2, 3), neighbours_counter, |n| n == &GOAL)
            .expect("path not found");
        assert_eq!(cost, 8);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
        assert_eq!(*counter.borrow(), 20);
    }

    #[test]
    fn bfs_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let path = bfs(
            &(2, 3),
            |n| neighbours(n).into_iter().map(|(n, _)| n),
            |n| n == &GOAL,
        ).expect("path not found");
        assert_eq!(path.len(), 9);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
    }

    #[test]
    fn dfs_path_ok() {
        const GOAL: (usize, usize) = (6, 3);
        let path = dfs((2, 3), |n| neighbours(n).into_iter().map(|(n, _)| n), |n| {
            n == &GOAL
        }).expect("path not found");
        assert!(path.len() >= 9);
        assert!(path.iter().all(|&(nx, ny)| OPEN[ny][nx]));
    }

    #[test]
    fn astar_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            astar(&(2, 3), neighbours, |n| distance(n, &GOAL), |n| n == &GOAL),
            None
        );
    }

    #[test]
    fn fringe_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            fringe(&(2, 3), neighbours, |n| distance(n, &GOAL), |n| n == &GOAL),
            None
        );
    }

    #[test]
    fn dijkstra_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(dijkstra(&(2, 3), neighbours, |n| n == &GOAL), None);
    }

    #[test]
    fn bfs_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            bfs(
                &(2, 3),
                |n| neighbours(n).into_iter().map(|(n, _)| n),
                |n| n == &GOAL,
            ),
            None
        );
    }

    #[test]
    fn dfs_no_path() {
        const GOAL: (usize, usize) = (1, 1);
        assert_eq!(
            dfs((2, 3), |n| neighbours(n).into_iter().map(|(n, _)| n), |n| {
                n == &GOAL
            }),
            None
        );
    }

}
