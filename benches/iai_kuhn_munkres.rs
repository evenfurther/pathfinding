use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use pathfinding::prelude::{Matrix, kuhn_munkres};
use rand::{Rng as _, SeedableRng as _};
use rand_xorshift::XorShiftRng;

const RNG_SEED: [u8; 16] = [
    3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
];

#[library_benchmark]
fn kuhn_munkres_size_32() {
    let size = 32;
    let mut rng = XorShiftRng::from_seed(RNG_SEED);
    let weights = Matrix::square_from_vec(
        (0..(size * size))
            .map(|_| rng.random_range(1..=100))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    kuhn_munkres(&weights);
}

#[library_benchmark]
fn kuhn_munkres_size_64() {
    let size = 64;
    let mut rng = XorShiftRng::from_seed(RNG_SEED);
    let weights = Matrix::square_from_vec(
        (0..(size * size))
            .map(|_| rng.random_range(1..=100))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    kuhn_munkres(&weights);
}

#[library_benchmark]
fn kuhn_munkres_size_128() {
    let size = 128;
    let mut rng = XorShiftRng::from_seed(RNG_SEED);
    let weights = Matrix::square_from_vec(
        (0..(size * size))
            .map(|_| rng.random_range(1..=100))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    kuhn_munkres(&weights);
}

library_benchmark_group!(
    name = kuhn_munkres_benches;
    benchmarks = kuhn_munkres_size_32, kuhn_munkres_size_64, kuhn_munkres_size_128
);

main!(library_benchmark_groups = kuhn_munkres_benches);
