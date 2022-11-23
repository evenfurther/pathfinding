use pathfinding::directed::count_paths::count_paths;

#[test]
fn part2() {
    let mut adapters: Vec<i32> = include_str!("aoc-2020-10.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    adapters.sort_unstable();

    let &last = adapters.last().unwrap();

    let n = count_paths(
        0,
        |&x| {
            adapters
                .iter()
                .filter(move |&&y| y > x && y <= x + 3)
                .copied()
        },
        |&x| x == last,
    );

    assert_eq!(n, 19208);
}
