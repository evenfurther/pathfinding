use codspeed_criterion_compat::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use pathfinding::bertsekas::{bertsekas_aaap, Auction};
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
        let elements = size * size;

        group.throughput(Throughput::Elements(elements as u64));
        group.bench_function(BenchmarkId::new("Bertekas Auction", size), |b| {
            let (_, float_matrix) = black_box(create_matrices(*size));
            b.iter_with_large_drop(|| {
                let mut auction_data = Auction::new(&float_matrix);
                bertsekas_aaap(&mut auction_data);
            });
        });

        group.throughput(Throughput::Elements(elements as u64));
        group.bench_function(BenchmarkId::new("Hungarian Algorithm", size), |b| {
            let (int_64matrix, _) = black_box(create_matrices(*size));
            b.iter_with_large_drop(|| kuhn_munkres(&int_64matrix));
        });
    }

    // Configure the plot
    group.plot_config(
        codspeed_criterion_compat::PlotConfiguration::default()
            .summary_scale(codspeed_criterion_compat::AxisScale::Logarithmic),
    );

    group.finish();
}

criterion_group!(benches, compare_algorithms);
criterion_main!(benches);
