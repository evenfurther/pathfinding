use pathfinding::cycle_detection::*;

#[test]
fn test_floyd() {
    assert_eq!(floyd(-10, |x| (x + 5) % 6 + 3), (3, 6, 2));
}

#[test]
fn test_brent() {
    assert_eq!(brent(-10, |x| (x + 5) % 6 + 3), (3, 6, 2));
}
