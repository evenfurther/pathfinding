use pathfinding::bertsekas::{bertsekas_aaap, Auction};
use pathfinding::kuhn_munkres::kuhn_munkres;
use pathfinding::matrix::Matrix;
use rand::Rng;
use std::time::Instant;

fn generate_random_matrices(rows: usize, cols: usize) -> (Matrix<f64>, Matrix<i64>) {
    let mut rng = rand::thread_rng();
    let random_numbers: Vec<i64> = (0..rows * cols).map(|_| rng.gen_range(0..100)).collect();

    let matrix_int = Matrix::from_vec(rows, cols, random_numbers.clone()).unwrap();
    let matrix_float = Matrix::from_vec(
        rows,
        cols,
        random_numbers.into_iter().map(|x| x as f64).collect(),
    )
    .unwrap();

    (matrix_float, matrix_int)
}

fn main() {
    let sizes = vec![5, 10, 50, 100, 250, 500, 1000];

    for &size in &sizes {
        println!("Matrix size: {size} x {size}");
        let (f_matrix, i_matrix) = generate_random_matrices(size, size);

        // Bertsekas solver
        let start = Instant::now();
        let mut auction_data = Auction::new(&f_matrix);
        bertsekas_aaap(&mut auction_data);
        let score = auction_data.score().unwrap();
        let duration = start.elapsed();
        println!(
            "Bertsekas algo time elapsed: {:?} with score: {score}",
            duration
        );

        // Kuhn-Munkres solver
        let start = Instant::now();
        let (score, _) = kuhn_munkres(&i_matrix);
        let duration = start.elapsed();
        println!(
            "Hungarian algo time elapsed: {:?} with score: {score}",
            duration
        );
    }
}
