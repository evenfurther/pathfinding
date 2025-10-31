use pathfinding::directed::cycle_detection::*;

#[test]
fn floyd_works() {
    assert_eq!(floyd(-10, |x| (x + 5) % 6 + 3), (3, 6, 2));
}

#[test]
fn brent_works() {
    assert_eq!(brent(-10, |x| (x + 5) % 6 + 3), (3, 6, 2));
}

#[test]
fn floyd_partial_works() {
    let (lam, elem, mu_tilde) = floyd_partial(-10, |x| (x + 5) % 6 + 3);
    // Check that we get the correct cycle length
    assert_eq!(lam, 3);
    // Check that elem is in the cycle (cycle is 6, 8, 4)
    assert!([4, 6, 8].contains(&elem));
    // Check that mu_tilde is an upper bound: mu <= mu_tilde < mu + lam
    // We know mu = 2 from the full algorithm
    assert!(2 <= mu_tilde);
    assert!(mu_tilde < 2 + 3);
}

#[test]
fn brent_partial_works() {
    let (lam, elem, mu_tilde) = brent_partial(-10, |x| (x + 5) % 6 + 3);
    // Check that we get the correct cycle length
    assert_eq!(lam, 3);
    // Check that elem is in the cycle (cycle is 6, 8, 4)
    assert!([4, 6, 8].contains(&elem));
    // Check that mu_tilde is an upper bound: mu <= mu_tilde
    // We know mu = 2 from the full algorithm
    assert!(2 <= mu_tilde);
}

#[test]
fn partial_functions_match_full_functions() {
    // Test that partial functions return the same lambda as full functions
    let f = |x| (x + 5) % 6 + 3;

    let (lam_floyd, elem_floyd, mu_floyd) = floyd(-10, f);
    let (lam_floyd_partial, elem_floyd_partial, mu_tilde_floyd) = floyd_partial(-10, f);

    let (lam_brent, elem_brent, mu_brent) = brent(-10, f);
    let (lam_brent_partial, elem_brent_partial, mu_tilde_brent) = brent_partial(-10, f);

    // Lambda should be the same
    assert_eq!(lam_floyd, lam_floyd_partial);
    assert_eq!(lam_brent, lam_brent_partial);
    assert_eq!(lam_floyd, lam_brent);

    // Elements should be in the cycle
    assert!([4, 6, 8].contains(&elem_floyd));
    assert!([4, 6, 8].contains(&elem_floyd_partial));
    assert!([4, 6, 8].contains(&elem_brent));
    assert!([4, 6, 8].contains(&elem_brent_partial));

    // Mu values from full algorithms should be the same
    assert_eq!(mu_floyd, mu_brent);

    // Mu_tilde should be valid upper bounds
    assert!(mu_floyd <= mu_tilde_floyd);
    assert!(mu_tilde_floyd < mu_floyd + lam_floyd);
    assert!(mu_brent <= mu_tilde_brent);
}

#[test]
fn test_longer_cycle() {
    // Test with a longer cycle: sequence from 0 to 99, then cycles
    let f = |x: i32| (x + 1) % 100;

    let (lam_floyd, elem_floyd, mu_floyd) = floyd(0, f);
    let (lam_floyd_partial, elem_floyd_partial, mu_tilde_floyd) = floyd_partial(0, f);

    let (lam_brent, elem_brent, mu_brent) = brent(0, f);
    let (lam_brent_partial, elem_brent_partial, mu_tilde_brent) = brent_partial(0, f);

    // Cycle length should be 100 (0, 1, 2, ..., 99, 0, ...)
    assert_eq!(lam_floyd, 100);
    assert_eq!(lam_floyd_partial, 100);
    assert_eq!(lam_brent, 100);
    assert_eq!(lam_brent_partial, 100);

    // Mu should be 0 (cycle starts immediately)
    assert_eq!(mu_floyd, 0);
    assert_eq!(mu_brent, 0);

    // Elements should be in the cycle
    assert!((0..100).contains(&elem_floyd));
    assert!((0..100).contains(&elem_floyd_partial));
    assert!((0..100).contains(&elem_brent));
    assert!((0..100).contains(&elem_brent_partial));

    // Mu_tilde should be valid upper bounds
    assert!(mu_floyd <= mu_tilde_floyd);
    assert!(mu_tilde_floyd < mu_floyd + lam_floyd);
    assert!(mu_brent <= mu_tilde_brent);
}

#[test]
fn test_short_cycle_large_mu() {
    // Sequence starting from -100, adds 1 each step,
    // but when value reaches 10, it resets to 0
    // This creates: -100, -99, ..., -1, 0, 1, ..., 9, 10, 0, 1, ..., 9, 10, 0, ...
    // mu = 100 (steps to reach 0, which is the start of the cycle), lambda = 11 (0 to 10 inclusive)
    let f = |x: i32| if x == 10 { 0 } else { x + 1 };

    let (lam_floyd, elem_floyd, mu_floyd) = floyd(-100, f);
    let (lam_floyd_partial, elem_floyd_partial, mu_tilde_floyd) = floyd_partial(-100, f);

    let (lam_brent, elem_brent, mu_brent) = brent(-100, f);
    let (lam_brent_partial, elem_brent_partial, mu_tilde_brent) = brent_partial(-100, f);

    // Cycle length should be 11 (0, 1, 2, ..., 9, 10, 0, ...)
    assert_eq!(lam_floyd, 11);
    assert_eq!(lam_floyd_partial, 11);
    assert_eq!(lam_brent, 11);
    assert_eq!(lam_brent_partial, 11);

    // Mu should be 100 (it takes 100 steps to get from -100 to 0, then cycles)
    assert_eq!(mu_floyd, 100);
    assert_eq!(mu_brent, 100);

    // Elements should be in the cycle (0 to 10)
    assert!((0..=10).contains(&elem_floyd));
    assert!((0..=10).contains(&elem_floyd_partial));
    assert!((0..=10).contains(&elem_brent));
    assert!((0..=10).contains(&elem_brent_partial));

    // Mu_tilde should be valid upper bounds
    assert!(mu_floyd <= mu_tilde_floyd);
    assert!(mu_tilde_floyd < mu_floyd + lam_floyd);
    assert!(mu_brent <= mu_tilde_brent);
}
