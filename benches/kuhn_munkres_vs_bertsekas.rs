use codspeed_criterion_compat::Throughput;
use codspeed_criterion_compat::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pathfinding::bertsekas::{forward, Auction};
use pathfinding::prelude::{kuhn_munkres, Matrix};
use rand::Rng;

fn create_matrices(size: usize) -> (Matrix<i64>, Matrix<f64>) {
    let mut rng = rand::thread_rng();
    let int_matrix: Matrix<i64> = Matrix::from_fn(size, size, |_| rng.gen_range(0..100));
    let float_matrix = int_matrix.clone().map(|value| value as f64);
    (int_matrix, float_matrix)
}

fn compare_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("Assignment Problem");

    let sizes = [10, 20, 50, 100, 200, 500, 1000];

    for size in sizes.iter() {
        // Bertekas Auction - Time
        group.bench_with_input(
            BenchmarkId::new("Bertekas Auction Time", size),
            size,
            |b, &size| {
                let (_, float_matrix) = create_matrices(size);
                let mut auction_data = Auction::new(float_matrix);
                b.iter(|| {
                    forward(&mut auction_data);
                });
            },
        );

        // Hungarian Algorithm - Time
        group.bench_with_input(
            BenchmarkId::new("Hungarian Algorithm Time", size),
            size,
            |b, &size| {
                let (int_64matrix, _) = create_matrices(size);
                b.iter(|| {
                    kuhn_munkres(&int_64matrix);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, compare_algorithms);
criterion_main!(benches);
