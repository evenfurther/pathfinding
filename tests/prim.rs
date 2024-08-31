use pathfinding::undirected::prim::prim;

#[test]
// Simple example taken from the test used in kruskal implementation
fn base_test() {
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
        prim(&edges),
        vec![
            (&'a', &'e', 1),
            (&'a', &'b', 3),
            (&'b', &'c', 5),
            (&'c', &'d', 2),
        ]
    );
}

#[test]
// Taken from https://www.geeksforgeeks.org/prims-minimum-spanning-tree-mst-greedy-algo-5/
fn geeksforgeeks() {
    let edges = vec![
        (0, 1, 4),
        (0, 7, 8),
        (1, 2, 8),
        (1, 7, 11),
        (2, 3, 7),
        (2, 5, 4),
        (2, 8, 2),
        (3, 4, 9),
        (3, 5, 14),
        (4, 5, 10),
        (5, 6, 2),
        (6, 7, 1),
        (6, 8, 6),
        (7, 8, 7),
    ];
    assert_eq!(
        prim(&edges),
        vec![
            (&0, &1, 4),
            (&0, &7, 8),
            (&7, &6, 1),
            (&6, &5, 2),
            (&5, &2, 4),
            (&2, &8, 2),
            (&2, &3, 7),
            (&3, &4, 9),
        ]
    );
}

// Order of edges is not important in the result, except for starting edge, because always
// starting vertex of the first edge will be selected as the start in algorithm
#[test]
fn another_test() {
    let edges = vec![
        ('B', 'C', 10),
        ('B', 'D', 4),
        ('C', 'D', 2),
        ('A', 'C', 3),
        ('C', 'D', 2),
        ('D', 'E', 1),
        ('C', 'E', 6),
        ('D', 'E', 1),
    ];
    assert_eq!(
        prim(&edges),
        vec![
            (&'B', &'D', 4),
            (&'D', &'E', 1),
            (&'D', &'C', 2),
            (&'C', &'A', 3),
        ]
    );
}
