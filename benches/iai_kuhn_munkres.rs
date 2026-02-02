use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use pathfinding::prelude::{Matrix, kuhn_munkres};
use rand::Rng as _;
use rand_core::SeedableRng as _;
use rand_xorshift::XorShiftRng;

// Adapter to make XorShiftRng (which implements rand_core 0.10 traits)
// compatible with rand 0.9.2 (which expects rand_core 0.9 traits)
struct RngAdapter<R>(R);

#[expect(deprecated)]
impl<R: rand_core::RngCore> rand::RngCore for RngAdapter<R> {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }
}

const RNG_SEED: [u8; 16] = [
    3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
];

#[library_benchmark]
fn kuhn_munkres_size_32() {
    let size = 32;
    let mut rng = RngAdapter(XorShiftRng::from_seed(RNG_SEED));
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
    let mut rng = RngAdapter(XorShiftRng::from_seed(RNG_SEED));
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
    let mut rng = RngAdapter(XorShiftRng::from_seed(RNG_SEED));
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
