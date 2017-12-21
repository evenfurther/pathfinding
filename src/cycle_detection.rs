//! Detect the existence of a cycle in an infinite sequence.

/// Detect the existence of a cycle in an infinite sequence if one exists
/// using Floyd's algorithm.
/// Return the cycle size, the first element, and the index of first element.
pub fn floyd<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let mut tortoise = successor(start.clone());
    let mut hare = successor(successor(start.clone()));
    while tortoise != hare {
        (tortoise, hare) = (successor(tortoise), successor(successor(hare)));
    }
    let mut mu = 0;
    tortoise = start;
    while tortoise != hare {
        (tortoise, hare, mu) = (successor(tortoise), successor(hare), mu + 1);
    }
    let mut lam = 1;
    hare = successor(tortoise.clone());
    while tortoise != hare {
        (hare, lam) = (successor(hare), lam + 1);
    }
    (lam, tortoise, mu)
}

/// Detect the existence of a cycle in an infinite sequence if one exists
/// using Brent's algorithm.
/// Return the cycle size, the first element, and the index of first element.
pub fn brent<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = start.clone();
    let mut hare = successor(start.clone());
    while tortoise != hare {
        if power == lam {
            (tortoise, power, lam) = (hare.clone(), power * 2, 0);
        }
        (hare, lam) = (successor(hare), lam + 1);
    }
    let mut mu = 0;
    (tortoise, hare) = (start.clone(), (0..lam).fold(start, |x, _| successor(x)));
    while tortoise != hare {
        (tortoise, hare, mu) = (successor(tortoise), successor(hare), mu + 1);
    }
    (lam, hare, mu)
}
