use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pathfinding::prelude::{kuhn_munkres, Matrix};
use rand::Rng;

fn compare_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare kuhn_munkres with different input sizes");
    let mut rng = rand::thread_rng();
    for size in 5..10 {
        let size = 1 << size;
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
