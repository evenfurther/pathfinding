use pathfinding::utils::*;

#[test]
fn in_direction_start_oob() {
    assert_eq!(move_in_direction((8, 8), (-1, -1), (8, 8)), None);
    assert_eq!(in_direction((8, 8), (-1, -1), (8, 8)).next(), None);
}

#[test]
fn in_direction_end_oob() {
    assert_eq!(move_in_direction((0, 0), (-1, -1), (8, 8)), None);
    assert_eq!(in_direction((0, 0), (-1, -1), (8, 8)).next(), None);
    assert_eq!(move_in_direction((7, 0), (1, -1), (8, 8)), None);
    assert_eq!(in_direction((0, 7), (-1, 1), (8, 8)).next(), None);
}

#[test]
fn in_direction_invalid() {
    assert_eq!(in_direction((0, 8), (-1, 1), (8, 8)).next(), None);
    assert_eq!(in_direction((8, 0), (-1, 1), (8, 8)).next(), None);
    assert_eq!(in_direction((0, 7), (0, 0), (8, 8)).next(), None);
}

#[test]
fn in_direction_valid() {
    assert_eq!(move_in_direction((1, 1), (1, 3), (2, 4)), None);
    assert_eq!(
        in_direction((1, 1), (1, 3), (8, 8)).collect::<Vec<_>>(),
        vec![(2, 4), (3, 7)]
    );
}
