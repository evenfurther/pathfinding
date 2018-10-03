#[macro_use]
extern crate itertools;
extern crate pathfinding;
extern crate rand;

use itertools::Itertools;
use pathfinding::grid::Grid;
use rand::prelude::*;

#[test]
fn empty_grid() {
    let mut g = Grid::new(0, 0);
    assert_eq!(g.iter().next(), None);
    assert!(g.is_empty());
    assert!(g.is_full());
    assert!(!g.clear());
    assert!(!g.fill());
    assert_eq!(g.iter().next(), None);
    assert!(g.is_empty());
    assert!(g.is_full());
    g.invert();
    assert_eq!(g.iter().next(), None);
    assert!(g.is_empty());
    assert!(g.is_full());
    assert_eq!(g.edges().next(), None);
}

#[test]
fn one_point_grid() {
    let mut g = Grid::new(1, 1);
    assert_eq!(g.iter().next(), None);
    g.fill();
    assert_eq!(g.iter().collect_vec(), vec![(0, 0)]);
    assert_eq!(g.neighbours(&(0, 0)), vec![]);
    assert_eq!(g.edges().next(), None);
}

#[test]
fn diagonal_mode() {
    let mut g = Grid::new(3, 3);
    assert_eq!(g.iter().count(), 0);
    g.fill();
    assert_eq!(g.iter().count(), 9);
    let mut ns = g.neighbours(&(1, 1));
    ns.sort();
    assert_eq!(ns, vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
    g.enable_diagonal_mode();
    let mut ns = g.neighbours(&(1, 1));
    ns.sort();
    assert_eq!(
        ns,
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ]
    );
    g.disable_diagonal_mode();
    let mut ns = g.neighbours(&(1, 1));
    ns.sort();
    assert_eq!(ns, vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
}

#[test]
fn resize() {
    let mut g = Grid::new(3, 3);
    assert_eq!(g.vertices_len(), 0);
    assert!(!g.resize(4, 4));
    assert_eq!(g.vertices_len(), 0);
    assert!(!g.resize(3, 3));
    assert_eq!(g.vertices_len(), 0);
    g.fill();
    assert_eq!(g.vertices_len(), 9);
    assert!(!g.resize(4, 4));
    assert_eq!(g.vertices_len(), 9);
    assert!(!g.resize(3, 3));
    assert_eq!(g.vertices_len(), 9);
    assert!(g.resize(2, 2));
    assert_eq!(g.vertices_len(), 4);
    assert!(!g.resize(3, 3));
    assert_eq!(g.vertices_len(), 4);
    assert!(!g.resize(10, 10));
    assert_eq!(g.vertices_len(), 4);
    let mut ns = g.neighbours(&(1, 1));
    ns.sort();
    assert_eq!(ns, vec![(0, 1), (1, 0)]);
    for _ in 0..2 {
        let ns = g.neighbours(&(1, 1));
        for n in iproduct!(0..g.width, 0..g.height) {
            let present = ns.contains(&n);
            assert_eq!(g.has_edge(&(1, 1), &n), present);
            assert_eq!(g.has_edge(&n, &(1, 1)), present);
        }
        g.enable_diagonal_mode();
    }
}

#[test]
fn dimensions() {
    let mut g = Grid::new(3, 4);
    assert_eq!(g.width, 3);
    assert_eq!(g.height, 4);
    assert_eq!(g.size(), 12);
    g.resize(2, 7);
    assert_eq!(g.width, 2);
    assert_eq!(g.height, 7);
    assert_eq!(g.size(), 14);
}

#[test]
fn add_remove() {
    let mut g = Grid::new(3, 3);
    g.fill();
    for _ in 0..2 {
        assert!(g.has_vertex(&(1, 1)));
        assert!(g.remove_vertex(&(1, 1)));
        assert!(!g.has_vertex(&(1, 1)));
        assert!(!g.remove_vertex(&(1, 1)));
        assert!(!g.has_vertex(&(1, 1)));
        assert!(g.add_vertex((1, 1)));
        assert!(g.has_vertex(&(1, 1)));
        assert!(!g.add_vertex((1, 1)));
        assert!(g.has_vertex(&(1, 1)));
        g.resize(1000, 1000);
    }
}

#[test]
fn fill_clear_invert_empty_full() {
    let mut g = Grid::new(3, 3);
    assert_eq!(g.iter().count(), 0);
    assert!(g.is_empty());
    assert!(!g.is_full());
    assert!(g.fill());
    assert!(!g.fill());
    assert_eq!(g.iter().count(), 9);
    assert!(!g.is_empty());
    assert!(g.is_full());
    assert!(g.clear());
    assert!(!g.clear());
    assert_eq!(g.iter().count(), 0);
    assert!(g.is_empty());
    assert!(!g.is_full());
    g.invert();
    assert_eq!(g.iter().count(), 9);
    assert!(!g.is_empty());
    assert!(g.is_full());
    g.resize(3, 4);
    assert!(!g.is_empty());
    assert!(!g.is_full());
}

#[test]
fn iterators() {
    let mut rng = StdRng::from_entropy();
    for _ in 0..100 {
        let mut g = Grid::new(10, 20);
        if rng.gen_bool(0.5) {
            g.fill();
        }
        for _ in 0..1000 {
            let x = rng.next_u64() as usize % g.width;
            let y = rng.next_u64() as usize % g.height;
            if rng.gen_bool(0.5) {
                g.add_vertex((x, y));
            } else {
                g.remove_vertex(&(x, y));
            }
        }
        if rng.gen_bool(0.2) {
            g.invert();
        }
        let mut ns1 = g.iter().collect_vec();
        ns1.sort();
        let mut ns2 = g.into_iter().collect_vec();
        ns2.sort();
        assert_eq!(ns1, ns2);
    }
}

#[test]
fn collect() {
    let g = vec![(1, 7), (1, 8), (3, 4), (1, 7)]
        .into_iter()
        .collect::<Grid>();
    assert_eq!(g.width, 4);
    assert_eq!(g.height, 9);
    assert_eq!(g.vertices_len(), 3);
}

#[test]
fn is_inside() {
    let g = Grid::new(2, 4);
    for x in 0..2 {
        for y in 0..4 {
            assert!(g.is_inside(&(x, y)));
        }
    }
    for x in 0..2 {
        assert!(!g.is_inside(&(x, 4)));
    }
    for y in 0..4 {
        assert!(!g.is_inside(&(2, y)));
    }
}

#[test]
fn add_outside_vertex() {
    let mut g = Grid::new(10, 10);
    assert!(!g.add_vertex((10, 10)));
}

#[test]
fn remove_outside_vertex() {
    let mut g = Grid::new(10, 10);
    assert!(!g.remove_vertex(&(10, 10)));
}

#[test]
fn test_outside_vertex() {
    let mut g = Grid::new(10, 10);
    assert!(!g.has_vertex(&(10, 10)));
    g.fill();
    assert!(!g.has_vertex(&(10, 10)));
}

#[test]
fn neighbours_outside_vertex() {
    let mut g = Grid::new(10, 10);
    assert_eq!(g.neighbours(&(10, 10)), vec![]);
    g.fill();
    assert_eq!(g.neighbours(&(10, 10)), vec![]);
}

#[test]
fn totally_empty() {
    let g = Grid::new(0, 0);
    assert_eq!(g.vertices_len(), 0);
}

#[test]
fn empty() {
    let g = Grid::new(0, 4);
    assert_eq!(g.vertices_len(), 0);
}

#[test]
fn add_borders() {
    let mut g = Grid::new(3, 4);
    assert_eq!(g.add_borders(), 10);
    assert_eq!(g.vertices_len(), 10);
    for x in 0..3 {
        assert!(g.has_vertex(&(x, 0)));
        assert!(g.has_vertex(&(x, 3)));
    }
    for y in 0..4 {
        assert!(g.has_vertex(&(0, y)));
        assert!(g.has_vertex(&(2, y)));
    }
    assert_eq!(g.add_borders(), 0);
}

#[test]
fn add_borders_empty() {
    let mut g = Grid::new(4, 0);
    assert_eq!(g.add_borders(), 0);
}

#[test]
fn add_borders_flat() {
    let mut g = Grid::new(4, 1);
    assert_eq!(g.add_borders(), 4);
}

#[test]
fn remove_borders() {
    let mut g = Grid::new(3, 4);
    g.fill();
    assert_eq!(g.remove_borders(), 10);
    assert_eq!(g.vertices_len(), 2);
    for x in 0..3 {
        assert!(!g.has_vertex(&(x, 0)));
        assert!(!g.has_vertex(&(x, 3)));
    }
    for y in 0..4 {
        assert!(!g.has_vertex(&(0, y)));
        assert!(!g.has_vertex(&(2, y)));
    }
    assert_eq!(g.remove_borders(), 0);
}

#[test]
fn remove_borders_empty() {
    let mut g = Grid::new(0, 0);
    assert_eq!(g.vertices_len(), 0);
    g.fill();
    assert_eq!(g.vertices_len(), 0);
    assert_eq!(g.remove_borders(), 0);
    assert_eq!(g.vertices_len(), 0);
}

#[test]
fn remove_borders_flat() {
    let mut g = Grid::new(4, 1);
    assert_eq!(g.remove_borders(), 0);
    g.fill();
    assert_eq!(g.remove_borders(), 4);
}

#[test]
fn edges() {
    let mut g = Grid::new(2, 2);
    g.fill();
    let mut edges = g.edges().collect::<Vec<_>>();
    edges.sort();
    assert_eq!(
        edges,
        vec![
            ((0, 0), (0, 1)),
            ((0, 0), (1, 0)),
            ((0, 1), (1, 1)),
            ((1, 0), (1, 1))
        ]
    );
    g.enable_diagonal_mode();
    let mut edges = g.edges().collect::<Vec<_>>();
    edges.sort();
    assert_eq!(
        edges,
        vec![
            ((0, 0), (0, 1)),
            ((0, 0), (1, 0)),
            ((0, 0), (1, 1)),
            ((0, 1), (1, 1)),
            ((1, 0), (0, 1)),
            ((1, 0), (1, 1))
        ]
    );
    let mut g = Grid::new(3, 3);
    g.fill();
    g.remove_vertex(&(1, 1));
    let mut edges = g.edges().collect::<Vec<_>>();
    edges.sort();
    assert_eq!(
        edges,
        vec![
            ((0, 0), (0, 1)),
            ((0, 0), (1, 0)),
            ((0, 1), (0, 2)),
            ((0, 2), (1, 2)),
            ((1, 0), (2, 0)),
            ((1, 2), (2, 2)),
            ((2, 0), (2, 1)),
            ((2, 1), (2, 2))
        ]
    );
}
