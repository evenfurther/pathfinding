//! This example demonstrates the experimental complexity analysis of the Kuhn-Munkres algorithm.
//!
//! The Kuhn-Munkres algorithm (also known as the Hungarian algorithm) is documented to have
//! O(n³) time complexity, where n is the size of the input matrix. This example runs the
//! algorithm with various input sizes and measures the execution time to experimentally
//! verify this complexity.

use pathfinding::prelude::{Matrix, kuhn_munkres};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::time::Instant;

/// Test sizes to use for complexity analysis
const TEST_SIZES: &[usize] = &[5, 10, 20, 30, 40, 50, 60, 80, 100, 120, 150, 200];

/// Number of iterations to average for each size
const ITERATIONS: usize = 20;

/// Structure to hold timing results for a specific size
struct TimingResult {
    size: usize,
    avg_time_micros: f64,
}

fn main() {
    println!("Kuhn-Munkres Algorithm - Experimental Complexity Analysis");
    println!("=========================================================\n");
    println!("Testing with matrix sizes: {TEST_SIZES:?}");
    println!("Iterations per size: {ITERATIONS}\n");

    let mut results = Vec::new();

    // Run tests for each size
    for &size in TEST_SIZES {
        println!("Testing size {size}x{size}...");

        let mut total_time_micros = 0u128;

        // Use a fixed seed for reproducibility
        let mut rng = XorShiftRng::from_seed([
            3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
        ]);

        for iteration in 0..ITERATIONS {
            // Generate a random square matrix with weights between 1 and 100
            let weights = Matrix::square_from_vec(
                (0..(size * size))
                    .map(|_| rng.random_range(1..=100))
                    .collect::<Vec<_>>(),
            )
            .unwrap();

            // Measure execution time
            let start = Instant::now();
            let _result = kuhn_munkres(&weights);
            let elapsed = start.elapsed();

            total_time_micros += elapsed.as_micros();

            if iteration == 0 {
                println!("  First iteration: {elapsed:.2?}");
            }
        }

        #[expect(clippy::cast_precision_loss)]
        let avg_time_micros = total_time_micros as f64 / ITERATIONS as f64;
        println!("  Average time: {avg_time_micros:.2} μs\n");

        results.push(TimingResult {
            size,
            avg_time_micros,
        });
    }

    // Analyze complexity
    println!("\nComplexity Analysis");
    println!("===================\n");
    println!("Expected complexity: O(n³)");
    println!(
        "{:>6} {:>15} {:>15} {:>15}",
        "Size", "Time (μs)", "n³", "Time/n³"
    );
    println!("{}", "-".repeat(54));

    for result in &results {
        #[expect(clippy::cast_precision_loss)]
        let n_cubed = (result.size.pow(3)) as f64;
        let ratio = result.avg_time_micros / n_cubed;
        println!(
            "{:>6} {:>15.2} {:>15.0} {:>15.6}",
            result.size, result.avg_time_micros, n_cubed, ratio
        );
    }

    // Calculate growth rates between consecutive sizes
    println!("\nGrowth Rate Analysis");
    println!("====================\n");
    println!("If the algorithm is O(n³), when size doubles, time should increase by ~8x (2³ = 8)");
    println!(
        "{:>10} {:>15} {:>15}",
        "Size Ratio", "Time Ratio", "Expected"
    );
    println!("{}", "-".repeat(42));

    for i in 1..results.len() {
        #[expect(clippy::cast_precision_loss)]
        let size_ratio = results[i].size as f64 / results[i - 1].size as f64;
        let time_ratio = results[i].avg_time_micros / results[i - 1].avg_time_micros;
        let expected_ratio = size_ratio.powi(3);
        println!("{size_ratio:>10.2}x {time_ratio:>14.2}x {expected_ratio:>14.2}x");
    }

    println!("\nConclusion");
    println!("==========");
    println!("The Time/n³ ratio should remain relatively constant if the algorithm is O(n³).");
    println!("The actual time ratio should be close to the expected ratio for size doublings.");
}
