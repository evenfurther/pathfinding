use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use pathfinding::matrix::Matrix;

#[allow(clippy::missing_panics_doc)]
pub fn transpose_benchmark(c: &mut Criterion) {
    // Generate a 100 x 100 square matrix with entries from 1 to 100^2
    let data: Vec<i32> = (0..100 * 100).collect();
    let mut m = Matrix::square_from_vec(data).unwrap();

    c.bench_function("transpose", |b| b.iter(|| m.transpose()));
}

#[allow(clippy::missing_panics_doc)]
pub fn transpose_non_square_benchmark(c: &mut Criterion) {
    // Generate a 1000 x 10 square matrix with entries from 1 to 100^2
    let data: Vec<i32> = (0..100 * 100).collect();
    let mut m = Matrix::from_vec(1000, 10, data).unwrap();

    c.bench_function("transpose_non_square", |b| b.iter(|| m.transpose()));
}

criterion_group!(benches, transpose_benchmark, transpose_non_square_benchmark);
criterion_main!(benches);
