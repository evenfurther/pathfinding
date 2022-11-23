use itertools::Itertools;
use pathfinding::directed::count_paths::count_paths;

#[test]
fn part2() {
    let mut adapters: Vec<i32> = include_str!("aoc-2020-10.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    adapters.sort();

    dbg!(&adapters);

    let last = adapters[adapters.len() - 1];

    let n = count_paths(
        0,
        |&x| {
            adapters
                .iter()
                .copied()
                .filter(|&y| y > x && y <= x + 3)
                .collect_vec()
        },
        |&x| x == last,
    );

    assert_eq!(n, 19208);
}
