use codspeed_criterion_compat::{BenchmarkId, Criterion, criterion_group, criterion_main};
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

fn compare_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare kuhn_munkres with different input sizes");
    for size in 5..10 {
        let size = 1 << size;
        let mut rng = RngAdapter(XorShiftRng::from_seed([
            3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
        ]));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let weights = Matrix::square_from_vec(
                (0..(size * size))
                    .map(|_| rng.random_range(1..=100))
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            b.iter(|| kuhn_munkres(&weights));
        });
    }
    group.finish();
}

criterion_group!(benches, compare_size);
criterion_main!(benches);
