use pathfinding::matrix;

fn main() {
    // Empty matrix must be rejected
    _ = matrix!();
    // Single comma and multiple commas at the end must be rejected
    _ = matrix!(,);
    _ = matrix!( [1, 2], [3, 4], ,);
}
