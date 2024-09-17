use pathfinding::bertsekas::{bertsekas_aaap, Auction};
use pathfinding::matrix::Matrix;
use rand::Rng;

fn generate_random_matrix(rows: usize, cols: usize) -> Matrix<f64> {
    let mut rng = rand::thread_rng();
    Matrix::from_fn(rows, cols, |_| rng.gen_range(0.0..100.0))
}

fn main() {
    // let sizes = vec![100, 250, 500];

    let size = 500;
    let matrix = generate_random_matrix(size, size);
    let mut auction_data = Auction::new(&matrix);

    bertsekas_aaap(&mut auction_data);
}
