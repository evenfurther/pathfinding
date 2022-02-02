#![cfg(test)]

use pathfinding::{matrix, matrix::Matrix, utils::absdiff};

#[test]
fn sm() {
    let mut m = Matrix::new(2, 2, 0usize);
    m[(0, 0)] = 0;
    m[(0, 1)] = 1;
    m[(1, 0)] = 10;
    m[(1, 1)] = 11;
    m[(0, 1)] = 2;
    assert_eq!(m[(0, 0)], 0);
    assert_eq!(m[(0, 1)], 2);
    assert_eq!(m[(1, 0)], 10);
    assert_eq!(m[(1, 1)], 11);
    m.fill(33);
    assert_eq!(m[(0, 0)], 33);
    assert_eq!(m[(0, 1)], 33);
    assert_eq!(m[(1, 0)], 33);
    assert_eq!(m[(1, 1)], 33);
}

#[test]
fn from_vec() {
    let m = Matrix::from_vec(2, 3, vec![10, 20, 30, 40, 50, 60]).unwrap();
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 3);
    assert!(!m.is_square());
    assert_eq!(m[(0, 0)], 10);
    assert_eq!(m[(0, 1)], 20);
    assert_eq!(m[(0, 2)], 30);
    assert_eq!(m[(1, 0)], 40);
    assert_eq!(m[(1, 1)], 50);
    assert_eq!(m[(1, 2)], 60);
}

#[test]
fn from_vec_error() {
    assert!(Matrix::from_vec(2, 3, vec![20, 30, 40, 50, 60]).is_err());
}

#[test]
#[should_panic]
fn new_empty_row_panic() {
    let _ = Matrix::new(1, 0, 42);
}

#[test]
fn to_vec() {
    let mut m = Matrix::new(2, 2, 0usize);
    m[(0, 0)] = 0;
    m[(0, 1)] = 1;
    m[(1, 0)] = 10;
    m[(1, 1)] = 11;
    assert_eq!(m.to_vec(), vec![0, 1, 10, 11]);
}

#[test]
fn square_from_vec() {
    let m = Matrix::square_from_vec(vec![10, 20, 30, 40]).unwrap();
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 2);
    assert!(m.is_square());
    assert_eq!(m[(0, 0)], 10);
    assert_eq!(m[(0, 1)], 20);
    assert_eq!(m[(1, 0)], 30);
    assert_eq!(m[(1, 1)], 40);
}

#[test]
#[should_panic]
fn from_vec_panic() {
    Matrix::from_vec(2, 3, vec![1, 2, 3]).unwrap();
}

#[test]
#[should_panic]
fn square_from_vec_panic() {
    Matrix::square_from_vec(vec![1, 2, 3]).unwrap();
}

#[test]
fn square_rotate() {
    // 0 1 => 2 0 => 3 2  => 1 3
    // 2 3    3 1    1 0     0 2
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3]).unwrap();
    let m2 = Matrix::square_from_vec(vec![2, 0, 3, 1]).unwrap();
    let m3 = Matrix::square_from_vec(vec![3, 2, 1, 0]).unwrap();
    let m4 = Matrix::square_from_vec(vec![1, 3, 0, 2]).unwrap();
    assert_eq!(m1.rotated_cw(0), m1);
    assert_eq!(m1.rotated_cw(1), m2);
    assert_eq!(m1.rotated_cw(2), m3);
    assert_eq!(m1.rotated_cw(3), m4);
    assert_eq!(m1.rotated_ccw(0), m1);
    assert_eq!(m1.rotated_ccw(1), m4);
    assert_eq!(m1.rotated_ccw(2), m3);
    assert_eq!(m1.rotated_ccw(3), m2);
    // 0 1 2    6 3 0    8 7 6    2 5 8
    // 3 4 5 => 7 4 1 => 5 4 3 => 1 4 7
    // 6 7 8    8 5 2    2 1 0    0 3 6
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    let m2 = Matrix::square_from_vec(vec![6, 3, 0, 7, 4, 1, 8, 5, 2]).unwrap();
    let m3 = Matrix::square_from_vec(vec![8, 7, 6, 5, 4, 3, 2, 1, 0]).unwrap();
    let m4 = Matrix::square_from_vec(vec![2, 5, 8, 1, 4, 7, 0, 3, 6]).unwrap();
    assert_eq!(m1.rotated_cw(0), m1);
    assert_eq!(m1.rotated_cw(1), m2);
    assert_eq!(m1.rotated_cw(2), m3);
    assert_eq!(m1.rotated_cw(3), m4);
    assert_eq!(m1.rotated_ccw(0), m1);
    assert_eq!(m1.rotated_ccw(1), m4);
    assert_eq!(m1.rotated_ccw(2), m3);
    assert_eq!(m1.rotated_ccw(3), m2);
    // Same with 4
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
        .unwrap();
    let m2 = Matrix::square_from_vec(vec![12, 8, 4, 0, 13, 9, 5, 1, 14, 10, 6, 2, 15, 11, 7, 3])
        .unwrap();
    let m3 = Matrix::square_from_vec(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
        .unwrap();
    let m4 = Matrix::square_from_vec(vec![3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12])
        .unwrap();
    assert_eq!(m1.rotated_cw(0), m1);
    assert_eq!(m1.rotated_cw(1), m2);
    assert_eq!(m1.rotated_cw(2), m3);
    assert_eq!(m1.rotated_cw(3), m4);
    assert_eq!(m1.rotated_ccw(0), m1);
    assert_eq!(m1.rotated_ccw(1), m4);
    assert_eq!(m1.rotated_ccw(2), m3);
    assert_eq!(m1.rotated_ccw(3), m2);
    // Same with 5
    let m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ])
    .unwrap();
    let m2 = Matrix::square_from_vec(vec![
        20, 15, 10, 5, 0, 21, 16, 11, 6, 1, 22, 17, 12, 7, 2, 23, 18, 13, 8, 3, 24, 19, 14, 9, 4,
    ])
    .unwrap();
    assert_eq!(m1.rotated_cw(0), m1);
    assert_eq!(m1.rotated_cw(1), m2);
    assert_eq!(m2.rotated_cw(3), m1);
}

#[test]
fn non_square_rotate() {
    let m0 = matrix![[10, 20, 30], [40, 50, 60]];
    let m1 = matrix![[40, 10], [50, 20], [60, 30]];
    let m2 = matrix![[60, 50, 40], [30, 20, 10]];
    let m3 = matrix![[30, 60], [20, 50], [10, 40]];
    assert_eq!(m0.rotated_cw(0), m0);
    assert_eq!(m0.rotated_cw(1), m1);
    assert_eq!(m0.rotated_cw(2), m2);
    assert_eq!(m0.rotated_cw(3), m3);
    assert_eq!(m0.rotated_cw(4), m0);
    assert_eq!(m0.rotated_ccw(0), m0);
    assert_eq!(m0.rotated_ccw(1), m3);
    assert_eq!(m0.rotated_ccw(2), m2);
    assert_eq!(m0.rotated_ccw(3), m1);
    assert_eq!(m0.rotated_ccw(4), m0);
}

#[test]
#[should_panic]
fn no_rows_rotated_cw_panic() {
    let _ = Matrix::<u32>::new_empty(10).rotated_cw(1);
}

#[test]
#[should_panic]
fn no_rows_rotated_ccw_panic() {
    let _ = Matrix::<u32>::new_empty(10).rotated_ccw(1);
}

#[test]
fn no_rows_rotated_twice() {
    let _ = Matrix::<u32>::new_empty(10).rotated_cw(2);
    let _ = Matrix::<u32>::new_empty(10).rotated_ccw(2);
}

#[test]
fn flip() {
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3]).unwrap();
    let m2 = Matrix::square_from_vec(vec![1, 0, 3, 2]).unwrap();
    let m3 = Matrix::square_from_vec(vec![2, 3, 0, 1]).unwrap();
    assert_eq!(m1.flipped_lr(), m2);
    assert_eq!(m1.flipped_ud(), m3);
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    let m2 = Matrix::square_from_vec(vec![2, 1, 0, 5, 4, 3, 8, 7, 6]).unwrap();
    let m3 = Matrix::square_from_vec(vec![6, 7, 8, 3, 4, 5, 0, 1, 2]).unwrap();
    assert_eq!(m1.flipped_lr(), m2);
    assert_eq!(m1.flipped_ud(), m3);
}

#[test]
fn transpose() {
    let m1 = Matrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]).unwrap();
    let m2 = Matrix::from_vec(3, 2, vec![0, 3, 1, 4, 2, 5]).unwrap();
    assert_eq!(m1.transposed(), m2);
    assert_eq!(m2.transposed(), m1);
}

#[test]
#[should_panic]
fn no_rows_transposed_panic() {
    let _ = Matrix::<u32>::new_empty(10).transposed();
}

fn sum(slice: &[usize]) -> usize {
    slice.iter().sum::<usize>()
}

#[test]
fn as_ref() {
    let m1 = Matrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]).unwrap();
    assert_eq!(sum(m1.as_ref()), 15);
}

#[test]
fn as_mut() {
    let mut m1 = Matrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]).unwrap();
    assert_eq!(sum(m1.as_ref()), 15);
    m1.as_mut()[2] = 10;
    assert_eq!(sum(m1.as_ref()), 23);
}

#[test]
fn slice() {
    let m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ])
    .unwrap();
    let m2 = m1.slice(1..3, 2..5).unwrap();
    assert_eq!(m2.rows, 2);
    assert_eq!(m2.columns, 3);
    assert_eq!(m2.as_ref().to_vec(), [7, 8, 9, 12, 13, 14]);
}

#[test]
fn slice_err() {
    let m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ])
    .unwrap();
    assert!(m1.slice(1..3, 2..6).is_err());
}

#[test]
fn set_slice() {
    let mut m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ])
    .unwrap();
    let m2 = Matrix::from_vec(3, 2, vec![10, 20, 30, 40, 50, 60]).unwrap();
    m1.set_slice((2, 3), &m2);
    assert_eq!(
        m1.as_ref().to_vec(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 10, 20, 15, 16, 17, 30, 40, 20, 21, 22, 50,
            60,
        ]
    );
    let mut m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ])
    .unwrap();
    let m2 = Matrix::from_vec(4, 3, vec![10, 20, 22, 30, 40, 44, 50, 60, 66, 70, 80, 88]).unwrap();
    m1.set_slice((2, 3), &m2);
    assert_eq!(
        m1.as_ref().to_vec(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 10, 20, 15, 16, 17, 30, 40, 20, 21, 22, 50,
            60,
        ]
    );
}

#[test]
fn empty_extend() {
    let mut m = Matrix::new_empty(3);
    m.extend(&[0, 1, 2]).unwrap();
    m.extend(&[3, 4, 5]).unwrap();
    assert_eq!(m.columns, 3);
    assert_eq!(m.rows, 2);
    let mut i = 0;
    for row in 0..m.rows {
        for column in 0..m.columns {
            assert_eq!(m[(row, column)], i);
            i += 1;
        }
    }
}

#[test]
fn extend_bad_size_error() {
    let mut m = Matrix::new_empty(3);
    assert!(m.extend(&[0, 1]).is_err());
}

#[test]
fn extend_empty_row_error() {
    let mut m: Matrix<u32> = matrix![];
    assert!(m.extend(&[]).is_err());
}

#[test]
fn matrix_macro() {
    let m = matrix![[0, 1, 2], [3, 4, 5]];
    assert_eq!(m.columns, 3);
    assert_eq!(m.rows, 2);
    let mut i = 0;
    for row in 0..m.rows {
        for column in 0..m.columns {
            assert_eq!(m[(row, column)], i);
            i += 1;
        }
    }
    let other_m = matrix![0, 1, 2; 3, 4, 5];
    assert_eq!(m, other_m);
}

#[test]
#[should_panic]
fn matrix_macro_inconsistent_panic() {
    matrix![[0, 1, 2], [3, 4]];
}

#[test]
#[should_panic]
fn matrix_macro_inconsistent_panic_2() {
    matrix![0, 1, 2; 3, 4];
}

#[test]
fn macro_trailing_comma() {
    // A trailing comma must be accepted
    let m1 = matrix!([1, 2, 3], [4, 5, 6]);
    let m2 = matrix!([1, 2, 3], [4, 5, 6],);
    assert_eq!(m1, m2);
    let m3 = matrix!(1, 2, 3; 4, 5, 6,);
    assert_eq!(m1, m3);
    let m4 = matrix!(1, 2, 3; 4, 5, 6;);
    assert_eq!(m1, m4);
    let m5 = matrix!(1, 2, 3; 4, 5, 6,;);
    assert_eq!(m1, m5);
}

#[test]
fn neighbours() {
    let m = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    for r in 0..3 {
        for c in 0..3 {
            for &diagonal in &[false, true] {
                let mut neighbours = m.neighbours((r, c), diagonal).collect::<Vec<_>>();
                neighbours.sort_unstable();
                let mut manual = Vec::new();
                for rr in 0..3 {
                    for cc in 0..3 {
                        let dr = absdiff(r, rr);
                        let dc = absdiff(c, cc);
                        if dr + dc == 1 || (diagonal && dr == 1 && dc == 1) {
                            manual.push((rr, cc));
                        }
                    }
                }
                assert_eq!(neighbours, manual);
            }
        }
    }
}

#[test]
fn empty_neighbours() {
    let m: Matrix<u32> = matrix![];
    assert_eq!(m.neighbours((0, 0), false).collect::<Vec<_>>(), vec![]);
    assert_eq!(m.neighbours((0, 0), true).collect::<Vec<_>>(), vec![]);
    let m = Matrix::new(10, 10, 42);
    assert_eq!(m.neighbours((10, 10), false).collect::<Vec<_>>(), vec![]);
    assert_eq!(m.neighbours((10, 10), true).collect::<Vec<_>>(), vec![]);
}

#[test]
#[allow(deprecated)]
fn reachable() {
    let m = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    let indices = m.reachable((1, 0), false, |n| m[n] % 4 != 0);
    assert_eq!(
        indices.into_iter().collect::<Vec<_>>(),
        vec![(1, 0), (2, 0), (2, 1)]
    );

    let indices = m.reachable((1, 0), true, |n| m[n] % 4 != 0);
    assert_eq!(
        indices.into_iter().collect::<Vec<_>>(),
        vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]
    );
}

#[test]
fn bfs_reachable() {
    let m = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    let indices = m.bfs_reachable((1, 0), false, |n| m[n] % 4 != 0);
    assert_eq!(
        indices.into_iter().collect::<Vec<_>>(),
        vec![(1, 0), (2, 0), (2, 1)]
    );

    let indices = m.bfs_reachable((1, 0), true, |n| m[n] % 4 != 0);
    assert_eq!(
        indices.into_iter().collect::<Vec<_>>(),
        vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]
    );
}

#[test]
fn dfs_reachable() {
    let m = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    let indices = m.dfs_reachable((1, 0), false, |n| m[n] % 4 != 0);
    assert_eq!(
        indices.into_iter().collect::<Vec<_>>(),
        vec![(1, 0), (2, 0), (2, 1)]
    );

    let indices = m.dfs_reachable((1, 0), true, |n| m[n] % 4 != 0);
    assert_eq!(
        indices.into_iter().collect::<Vec<_>>(),
        vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]
    );
}

#[test]
fn bounds_test() {
    let mut m = matrix![[0, 1, 2], [3, 4, 5]];
    assert!(m.within_bounds((1, 2)));
    assert!(!m.within_bounds((1, 3)));
    assert!(!m.within_bounds((2, 2)));

    assert_eq!(m.get((1, 0)), Some(&3));
    assert_eq!(m.get((3, 0)), None);

    assert_eq!(m.get_mut((1, 0)), Some(&mut 3));
    assert_eq!(m.get_mut((3, 0)), None);
}

#[test]
fn from_rows() {
    let m = Matrix::from_rows((1..3).map(|n| (1..5).map(move |x| x * n))).unwrap();
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 4);
    assert_eq!(m.to_vec(), vec![1, 2, 3, 4, 2, 4, 6, 8]);
    let m = Matrix::from_rows((1..3).map(|n| (1..n).map(move |x| x * n)));
    assert!(m.is_err());
}

#[test]
fn from_iter() {
    let m = (1..3)
        .map(|n| (1..5).map(move |x| x * n))
        .collect::<Matrix<_>>();
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 4);
    assert_eq!(m.to_vec(), vec![1, 2, 3, 4, 2, 4, 6, 8]);
}

#[test]
#[should_panic]
fn from_iter_error() {
    let _ = (1..3)
        .map(|n| (1..n).map(move |x| x * n))
        .collect::<Matrix<_>>();
}

#[test]
fn iter() {
    let m = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mut i = m.iter();
    assert_eq!(i.next().unwrap(), &[0, 1, 2]);
    assert_eq!(i.next().unwrap(), &[3, 4, 5]);
    assert_eq!(i.next().unwrap(), &[6, 7, 8]);
    assert_eq!(i.next(), None);
}

#[test]
fn into_iter() {
    let m = matrix![[0, 1, 2], [2, 1, 0], [1, 0, 2]];
    for c in &m {
        assert_eq!(c.iter().sum::<u32>(), 3);
    }
}

#[test]
fn into_iter_is_fused() {
    let m = matrix![[0, 1, 2], [2, 1, 0], [1, 0, 2]];
    let mut it = m.iter();
    for _ in 0..3 {
        assert!(it.next().is_some());
    }
    for _ in 0..3 {
        assert!(it.next().is_none());
    }
}

#[test]
fn indices() {
    let m = matrix![[0, 1, 2], [2, 1, 0]];
    assert_eq!(
        m.indices().collect::<Vec<_>>(),
        vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
    );
}

#[test]
fn values() {
    let m = matrix![[0, 1, 2], [2, 1, 0]];
    assert_eq!(
        m.values().cloned().collect::<Vec<_>>(),
        vec![0, 1, 2, 2, 1, 0]
    );
}

#[test]
fn values_mut() {
    let mut m = matrix![[0, 1, 2], [2, 1, 0]];
    *m.values_mut().nth(3).unwrap() = 5;
    let mut iter = m.values_mut();
    iter.next();
    *iter.next().unwrap() = 4;
    assert_eq!(m, matrix![[0, 4, 2], [5, 1, 0]]);
}

#[test]
fn in_direction() {
    let m = Matrix::new_square(8, 0);
    assert_eq!(m.in_direction((1, 1), (0, 0)).collect::<Vec<_>>(), vec![]);
    assert_eq!(m.in_direction((10, 10), (0, 0)).collect::<Vec<_>>(), vec![]);
    assert_eq!(
        m.in_direction((10, 10), (-1, -1)).collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(
        m.in_direction((4, 4), (-2, 0)).collect::<Vec<_>>(),
        vec![(2, 4), (0, 4)]
    );
    assert_eq!(
        m.in_direction((4, 4), (-3, 0)).collect::<Vec<_>>(),
        vec![(1, 4)]
    );
    assert_eq!(
        m.in_direction((4, 4), (2, 0)).collect::<Vec<_>>(),
        vec![(6, 4)]
    );
    assert_eq!(
        m.in_direction((4, 4), (3, 0)).collect::<Vec<_>>(),
        vec![(7, 4)]
    );
    assert_eq!(
        m.in_direction((4, 4), (0, -2)).collect::<Vec<_>>(),
        vec![(4, 2), (4, 0)]
    );
    assert_eq!(
        m.in_direction((4, 4), (0, -3)).collect::<Vec<_>>(),
        vec![(4, 1)]
    );
    assert_eq!(
        m.in_direction((4, 4), (0, 2)).collect::<Vec<_>>(),
        vec![(4, 6)]
    );
    assert_eq!(
        m.in_direction((4, 4), (0, 3)).collect::<Vec<_>>(),
        vec![(4, 7)]
    );
}

#[test]
fn uninit() {
    struct NonClonable(usize);
    let mut mat = Matrix::<NonClonable>::new_uninit(3, 3);
    for row in 0..mat.rows {
        for column in 0..mat.columns {
            mat[(row, column)].write(NonClonable(3 * row + column));
        }
    }
    let mat = unsafe { mat.assume_init() };
    for row in 0..mat.rows {
        for column in 0..mat.columns {
            assert_eq!(mat[(row, column)].0, row * 3 + column);
        }
    }
}

#[test]
fn map() {
    let m = Matrix::new(3, 3, 10);
    let m = m.map({
        let mut counter = 0;
        move |x| {
            counter += 1;
            x + counter
        }
    });
    assert_eq!(
        m,
        Matrix::square_from_vec(vec![11, 12, 13, 14, 15, 16, 17, 18, 19]).unwrap()
    );
}
