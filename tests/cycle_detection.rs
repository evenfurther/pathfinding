use pathfinding::directed::cycle_detection::*;

#[test]
fn floyd_works() {
    assert_eq!(floyd(-10, |x| (x + 5) % 6 + 3), (3, 6, 2));
}

#[test]
fn brent_works() {
    assert_eq!(brent(-10, |x| (x + 5) % 6 + 3), (3, 6, 2));
}
