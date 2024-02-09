use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use pathfinding::matrix::Matrix;

#[allow(clippy::missing_panics_doc)]
pub fn transpose_benchmark(c: &mut Criterion) {
    // Generate a 100 x 100 square matrix with entries from 1 to 100^2
    let data: Vec<i32> = (0..100 * 100).collect();
    let mut m = Matrix::square_from_vec(data).unwrap();

    c.bench_function("transpose", |b| b.iter(|| m.transpose()));
}

criterion_group!(benches, transpose_benchmark);
criterion_main!(benches);
