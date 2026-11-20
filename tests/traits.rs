//! Check the standard traits implemented by the public types.

use pathfinding::prelude::*;

#[expect(clippy::trivially_copy_pass_by_ref)]
fn successors(n: &u32) -> Vec<u32> {
    if *n < 5 { vec![n + 1, n + 2] } else { vec![] }
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn weighted_successors(n: &u32) -> Vec<(u32, u32)> {
    successors(n).into_iter().map(|s| (s, 1)).collect()
}

#[test]
fn reachable_iterators_are_clone_and_debug() {
    let mut bfs = bfs_reach(0, successors);
    bfs.next();
    let copy = bfs.clone();
    assert_eq!(bfs.collect::<Vec<_>>(), copy.collect::<Vec<_>>());
    assert!(format!("{:?}", bfs_reach(0, successors)).starts_with("BfsReachable"));

    let mut dfs = dfs_reach(0, successors);
    dfs.next();
    let copy = dfs.clone();
    assert_eq!(dfs.collect::<Vec<_>>(), copy.collect::<Vec<_>>());
    assert!(format!("{:?}", dfs_reach(0, successors)).starts_with("DfsReachable"));

    let mut dijkstra = dijkstra_reach(&0, weighted_successors);
    dijkstra.next();
    let copy = dijkstra.clone();
    assert_eq!(dijkstra.collect::<Vec<_>>(), copy.collect::<Vec<_>>());
    assert!(
        format!("{:?}", dijkstra_reach(&0, weighted_successors)).starts_with("DijkstraReachable")
    );
}

#[test]
fn astar_solution_is_debug() {
    let (solution, _) = astar_bag(&0, weighted_successors, |_| 0, |n| *n == 5).unwrap();
    assert!(format!("{solution:?}").starts_with("AstarSolution"));
}

#[test]
fn grid_traits() {
    assert_eq!(Grid::default(), Grid::new(0, 0));
    let mut g = Grid::new(3, 3);
    g.add_vertex((1, 1));
    g.add_vertex((1, 2));
    let mut it = g.iter();
    let copy = it.clone();
    assert_eq!(it.next(), copy.clone().next());
    assert!(format!("{copy:?}").starts_with("GridIterator"));
    let edges = g.edges();
    assert_eq!(edges.clone().count(), 1);
    assert!(format!("{edges:?}").starts_with("EdgesIterator"));
    let into = g.into_iter();
    assert_eq!(into.clone().count(), 2);
    assert!(format!("{into:?}").starts_with("GridIntoIterator"));
}

#[test]
fn matrix_iterators_traits() {
    // `NoTraits` implements neither `Clone` nor `Debug`, which the iterators must not require.
    struct NoTraits;
    let m = Matrix::from_vec(2, 3, (0..6).map(|_| NoTraits).collect()).unwrap();
    let rows = m.iter();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows.clone().count(), 2);
    assert_eq!(format!("{rows:?}"), "RowIterator { front: 0, back: 2, .. }");
    let mut columns = m.column_iter();
    assert_eq!(columns.len(), 3);
    columns.next_back();
    assert_eq!(columns.len(), 2);
    assert_eq!(columns.clone().count(), 2);
    assert_eq!(
        format!("{columns:?}"),
        "ColumnIterator { front: 0, back: 2, .. }"
    );
}

#[test]
fn matrix_format_error_traits() {
    let e = Matrix::from_vec(2, 2, vec![1]).unwrap_err();
    assert_eq!(e, MatrixFormatError::WrongLength);
    let copy = e;
    assert_eq!(copy.clone(), e);
}
