//! The `matrix!` macro must not depend on the crate being reachable as `pathfinding` at
//! the call site: here, that name refers to a local module instead.

use ::pathfinding::matrix;

mod pathfinding {}

#[test]
fn matrix_macro_ignores_local_pathfinding_item() {
    let m = matrix![[1, 2], [3, 4]];
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 2);
    let empty: ::pathfinding::matrix::Matrix<u8> = matrix![];
    assert!(empty.is_empty());
}
