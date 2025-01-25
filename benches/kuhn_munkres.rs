use codspeed_criterion_compat::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pathfinding::prelude::{kuhn_munkres, Matrix};
use rand::{Rng as _, SeedableRng as _};
use rand_xorshift::XorShiftRng;

fn compare_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare kuhn_munkres with different input sizes");
    for size in 5..10 {
        let size = 1 << size;
        let mut rng = XorShiftRng::from_seed([
            3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
        ]);
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let weights = Matrix::square_from_vec(
                (0..(size * size))
                    .map(|_| rng.gen_range(1..=100))
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
