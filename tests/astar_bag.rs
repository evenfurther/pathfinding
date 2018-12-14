use itertools::Itertools;
use pathfinding::directed::astar::astar_bag;

#[test]
fn multiple_sinks() {
    // 1 --> 2 --> 4
    //   --> 3 --> 4
    //
    // 2 --> 5 --> 6 --> 7
    // 3 --> 5 --> 6 --> 7
    let (solutions, cost) = astar_bag(
        &1,
        |&n| match n {
            1 => vec![(2, 1), (3, 1)],
            2 | 3 => vec![(4, 3), (5, 1)],
            5 => vec![(6, 1)],
            6 => vec![(7, 1)],
            _ => vec![],
        },
        |_| 0,
        |&n| n == 4 || n == 7,
    )
    .unwrap();
    assert_eq!(cost, 4);
    assert_eq!(
        solutions.sorted().collect_vec(),
        vec![
            vec![1, 2, 4],
            vec![1, 2, 5, 6, 7],
            vec![1, 3, 4],
            vec![1, 3, 5, 6, 7],
        ]
    );
}

#[test]
fn numerous_solutions() {
    const N: usize = 10;
    const GOAL: usize = 3 * N;
    //     ---> 1 --
    //    /     |   \
    // 0--      |    --> 3 … --> 3*N with 2^N paths and a cost of 2*N
    //    \     v   /            (path from 1 to 2 is unused)
    //     ---> 2 --
    let (solutions, cost) = astar_bag(
        &0,
        |&n| match n {
            x if x % 3 == 2 => vec![(x + 1, 1)],
            x => vec![(x + 1, 1), (x + 2, 1)],
        },
        |&n| GOAL.saturating_sub(n) / 2,
        |&n| n == GOAL,
    )
    .unwrap();
    assert_eq!(cost, N * 2);
    assert_eq!(solutions.count(), 1 << N);
}
