use itertools::Itertools;
use pathfinding::prelude::*;

#[test]
fn grid_lines() {
    let mut g = Grid::new(2, 2);
    g.fill();
    let weighted_edges = g
        .edges()
        .map(|((x1, y1), (x2, y2))| ((x1, y1), (x2, y2), y1.min(y2)))
        .collect::<Vec<_>>();
    assert_eq!(weighted_edges.len(), 4);
    let mst = kruskal(&weighted_edges).sorted().collect_vec();
    assert_eq!(
        mst,
        vec![
            (&(0, 0), &(0, 1), 0),
            (&(0, 0), &(1, 0), 0),
            (&(1, 0), &(1, 1), 0)
        ]
    );
}

#[test]
fn wikipedia() {
    // Example from https://en.wikipedia.org/wiki/Kruskal's_algorithm
    let edges = vec![
        ('a', 'b', 3),
        ('a', 'e', 1),
        ('b', 'c', 5),
        ('b', 'e', 4),
        ('c', 'd', 2),
        ('c', 'e', 6),
        ('d', 'e', 7),
    ];
    assert_eq!(
        kruskal(&edges).collect::<Vec<_>>(),
        vec![
            (&'a', &'e', 1),
            (&'c', &'d', 2),
            (&'a', &'b', 3),
            (&'b', &'c', 5)
        ]
    );
}
