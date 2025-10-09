//! Identify a cycle in an infinite sequence.

/// Identify a cycle in an infinite sequence using Floyd's algorithm (partial version).
/// Return the cycle size, an element in the cycle, and an upper bound on the index of
/// the first element.
///
/// This function computes the cycle length λ and returns an element within the cycle,
/// along with an upper bound μ̃ on the index of the first cycle element. The upper bound
/// μ̃ satisfies μ ≤ μ̃ < μ + λ, where μ is the minimal index.
///
/// This is faster than [`floyd`] as it skips the computation of the minimal μ.
/// The upper bound μ̃ is sufficient for many applications, such as computing f^n(x) for
/// large n, where knowing the exact starting point of the cycle is not necessary.
///
/// # Example
///
/// ```
/// use pathfinding::prelude::floyd_partial;
///
/// let (lam, _elem, mu_tilde) = floyd_partial(1, |x| (x * 2) % 7);
/// assert_eq!(lam, 3); // Cycle length
/// assert!(mu_tilde == 0); // Upper bound on mu (start is in cycle, so mu = 0)
/// ```
///
/// # Warning
///
/// If no cycle exist, this function loops forever.
#[expect(clippy::needless_pass_by_value)]
pub fn floyd_partial<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let mut tortoise = successor(start.clone());
    let mut hare = successor(successor(start.clone()));
    let mut tortoise_steps = 1;
    while tortoise != hare {
        (tortoise, hare) = (successor(tortoise), successor(successor(hare)));
        tortoise_steps += 1;
    }
    // tortoise and hare met at position tortoise_steps
    let mut lam = 1;
    hare = successor(tortoise.clone());
    while tortoise != hare {
        (hare, lam) = (successor(hare), lam + 1);
    }
    // Handle edge case where they meet at the start position (pure cycle, mu = 0)
    // In this case, tortoise_steps equals lam, and to satisfy mu_tilde < mu + lam,
    // we must return 0.
    let mu_tilde = if tortoise == start { 0 } else { tortoise_steps };
    (lam, tortoise, mu_tilde)
}

/// Identify a cycle in an infinite sequence using Floyd's algorithm.
/// Return the cycle size, the first element, and the index of first element.
///
/// # Warning
///
/// If no cycle exist, this function loops forever.
pub fn floyd<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let (lam, mut hare, _) = floyd_partial(start.clone(), &successor);
    // Find the exact mu
    let (mut mu, mut tortoise) = (0, start);
    while tortoise != hare {
        (tortoise, hare, mu) = (successor(tortoise), successor(hare), mu + 1);
    }
    (lam, tortoise, mu)
}

/// Identify a cycle in an infinite sequence using Brent's algorithm (partial version).
/// Return the cycle size, an element in the cycle, and an upper bound on the index of
/// the first element.
///
/// This function computes the cycle length λ and returns an element within the cycle,
/// along with an upper bound μ̃ on the index of the first cycle element. The upper bound
/// satisfies μ ≤ μ̃. Due to the nature of Brent's algorithm with its power-of-2 stepping,
/// the bound may be looser than `μ + λ` in some cases, but is still reasonable for
/// practical applications.
///
/// This is faster than [`brent`] as it skips the computation of the minimal μ.
/// The upper bound μ̃ is sufficient for many applications, such as computing f^n(x) for
/// large n, where knowing the exact starting point of the cycle is not necessary.
///
/// # Example
///
/// ```
/// use pathfinding::prelude::brent_partial;
///
/// let (lam, _elem, mu_tilde) = brent_partial(1, |x| (x * 2) % 7);
/// assert_eq!(lam, 3); // Cycle length
/// assert!(mu_tilde >= 1); // Upper bound on mu
/// ```
///
/// # Warning
///
/// If no cycle exist, this function loops forever.
pub fn brent_partial<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = start.clone();
    let mut hare = successor(start);
    let mut hare_steps = 1;
    while tortoise != hare {
        if power == lam {
            (tortoise, power, lam) = (hare.clone(), power * 2, 0);
        }
        (hare, lam) = (successor(hare), lam + 1);
        hare_steps += 1;
    }
    // Use hare_steps as the upper bound, as it represents where we detected the cycle.
    (lam, hare, hare_steps)
}

/// Identify a cycle in an infinite sequence using Brent's algorithm.
/// Return the cycle size, the first element, and the index of first element.
///
/// # Warning
///
/// If no cycle exist, this function loops forever.
pub fn brent<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let (lam, _hare, _hare_steps) = brent_partial(start.clone(), &successor);
    // Find the exact mu
    let mut mu = 0;
    let mut tortoise = start.clone();
    let mut hare = (0..lam).fold(start, |x, _| successor(x));
    while tortoise != hare {
        (tortoise, hare, mu) = (successor(tortoise), successor(hare), mu + 1);
    }
    (lam, hare, mu)
}
