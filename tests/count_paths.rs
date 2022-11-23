use pathfinding::directed::count_paths::count_paths;

#[test]
fn grid() {
    let n = count_paths(
        (0, 0),
        |&(x, y)| {
            [(x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|&(x, y)| x < 8 && y < 8)
        },
        |&c| c == (7, 7),
    );
    assert_eq!(n, 3432);
}
