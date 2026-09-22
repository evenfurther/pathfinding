//! A path cost that does not fit in the cost type must not be reported as if it did.
//!
//! Debug builds have always panicked on such an addition. Release builds used to wrap, which
//! turned an impossible-to-represent cost into a small plausible-looking one. These tests pin
//! the behaviour down in both profiles.

use pathfinding::prelude::*;

// The panic message differs by profile: debug builds trip the built-in "attempt to add with
// overflow" inside `+` before the library's own check runs, while release builds reach the
// check and report "cost overflow". Both contain "overflow", which is what these tests pin.

/// Three of these do not fit in a `u32`: the wrapped sum would be 2147483645.
const HUGE: u32 = u32::MAX / 2;

// The successor and success callbacks are handed a reference by the search functions.
#[expect(clippy::trivially_copy_pass_by_ref)]
fn expensive(n: &u32) -> Vec<(u32, u32)> {
    vec![(n + 1, HUGE)]
}

// The successor and success callbacks are handed a reference by the search functions.
#[expect(clippy::trivially_copy_pass_by_ref)]
fn cheap(n: &u32) -> Vec<(u32, u32)> {
    vec![(n + 1, 10)]
}

// The successor and success callbacks are handed a reference by the search functions.
#[expect(clippy::trivially_copy_pass_by_ref)]
const fn at_three(n: &u32) -> bool {
    *n == 3
}

#[test]
#[should_panic(expected = "overflow")]
fn astar_refuses_to_wrap() {
    astar(&0, expensive, |_| 0, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn astar_bag_refuses_to_wrap() {
    astar_bag(&0, expensive, |_| 0, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn dijkstra_refuses_to_wrap() {
    dijkstra(&0, expensive, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn dijkstra_all_refuses_to_wrap() {
    dijkstra_all(&0, |n: &u32| if *n < 4 { expensive(n) } else { vec![] });
}

#[test]
#[should_panic(expected = "overflow")]
fn dijkstra_reach_refuses_to_wrap() {
    dijkstra_reach(&0, |n: &u32| if *n < 4 { expensive(n) } else { vec![] }).for_each(drop);
}

#[test]
#[should_panic(expected = "overflow")]
fn fringe_refuses_to_wrap() {
    fringe(&0, expensive, |_| 0, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn idastar_refuses_to_wrap() {
    idastar(&0, expensive, |_| 0, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn yen_refuses_to_wrap() {
    yen(&0, expensive, at_three, 2);
}

/// The reported case: the path cost is representable, but adding the heuristic is not.
#[test]
#[should_panic(expected = "overflow")]
fn astar_checks_the_heuristic_too() {
    astar(&0, cheap, |_| u32::MAX, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn fringe_checks_the_heuristic_too() {
    fringe(&0, cheap, |_| u32::MAX, at_three);
}

#[test]
#[should_panic(expected = "overflow")]
fn idastar_checks_the_heuristic_too() {
    idastar(&0, cheap, |_| u32::MAX, at_three);
}

/// A cost type whose addition saturates is the documented way to opt out of the panic. It
/// must keep working: a saturating sum never compares smaller, so it never trips the check.
mod saturating {
    use super::{HUGE, at_three};
    use pathfinding::prelude::*;

    // The panic message differs by profile: debug builds trip the built-in "attempt to add with
    // overflow" inside `+` before the library's own check runs, while release builds reach the
    // check and report "cost overflow". Both contain "overflow", which is what these tests pin.

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Saturating(u32);

    impl std::ops::Add for Saturating {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self(self.0.saturating_add(other.0))
        }
    }

    impl num_traits::Zero for Saturating {
        fn zero() -> Self {
            Self(0)
        }
        fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }

    #[test]
    fn a_saturating_cost_type_does_not_panic() {
        let (path, cost) = astar(
            &0,
            |n: &u32| vec![(n + 1, Saturating(HUGE))],
            |_| Saturating(0),
            at_three,
        )
        .unwrap();
        assert_eq!(path, vec![0, 1, 2, 3]);
        assert_eq!(cost, Saturating(u32::MAX));
    }
}

/// Costs that fit must be entirely unaffected.
#[test]
fn ordinary_costs_are_untouched() {
    assert_eq!(astar(&0, cheap, |_| 0, at_three).unwrap().1, 30);
    assert_eq!(dijkstra(&0, cheap, at_three).unwrap().1, 30);
    assert_eq!(fringe(&0, cheap, |_| 0, at_three).unwrap().1, 30);
    assert_eq!(idastar(&0, cheap, |_| 0, at_three).unwrap().1, 30);
    assert_eq!(yen(&0, cheap, at_three, 1)[0].1, 30);
}
