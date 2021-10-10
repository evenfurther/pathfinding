use pathfinding::matrix;

fn main() {
    // Empty matrix must be rejected
    let _ = matrix!();
    // Single comma and multiple commas at the end must be rejected
    let _ = matrix!(,);
    let _ = matrix!( [1, 2], [3, 4], ,);
}
