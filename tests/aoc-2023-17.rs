use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

static TEST_INPUT: &str = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

static TEST_INPUT_2: &str = "
111111111111
999999999991
999999999991
999999999991
999999999991
";

fn part1(input: &str) -> u32 {
    part(input, 1, 3)
}

fn part2(input: &str) -> u32 {
    part(input, 4, 10)
}

fn part(input: &str, min_move: usize, max_move: usize) -> u32 {
    let grid = Matrix::from_rows(
        input
            .trim()
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10))),
    )
    .unwrap();
    dijkstra(
        &((0, 0), (0, 0), 0),
        |&(pos, (dr, dc), l)| {
            let mut next = Vec::with_capacity(3);
            let mut e = |dir, l| {
                next.extend(
                    &grid
                        .move_in_direction(pos, dir)
                        .map(|t| ((t, dir, l), grid[t])),
                );
            };
            if l < max_move {
                e((dr, dc), l + 1);
            }
            if l >= min_move {
                e((-dc, -dr), 1);
                e((dc, dr), 1);
            } else if l == 0 {
                e((1, 0), 1);
                e((0, 1), 1);
            }
            next
        },
        |&(pos, _, l)| pos == (grid.rows - 1, grid.columns - 1) && l >= min_move,
    )
    .unwrap()
    .1
}

#[test]
fn aoc_2023_17() {
    assert_eq!(102, part1(TEST_INPUT));
    assert_eq!(94, part2(TEST_INPUT));
    assert_eq!(71, part2(TEST_INPUT_2));
}
