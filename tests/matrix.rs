#![cfg(test)]

use pathfinding::{matrix, matrix::Matrix, utils::absdiff};

#[test]
fn sm() {
    let mut m = Matrix::new(2, 2, 0usize);
    m[&(0, 0)] = 0;
    m[&(0, 1)] = 1;
    m[&(1, 0)] = 10;
    m[&(1, 1)] = 11;
    m[&(0, 1)] = 2;
    assert_eq!(m[&(0, 0)], 0);
    assert_eq!(m[&(0, 1)], 2);
    assert_eq!(m[&(1, 0)], 10);
    assert_eq!(m[&(1, 1)], 11);
    m.fill(33);
    assert_eq!(m[&(0, 0)], 33);
    assert_eq!(m[&(0, 1)], 33);
    assert_eq!(m[&(1, 0)], 33);
    assert_eq!(m[&(1, 1)], 33);
}

#[test]
fn from_vec() {
    let m = Matrix::from_vec(2, 3, vec![10, 20, 30, 40, 50, 60]);
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 3);
    assert!(!m.is_square());
    assert_eq!(m[&(0, 0)], 10);
    assert_eq!(m[&(0, 1)], 20);
    assert_eq!(m[&(0, 2)], 30);
    assert_eq!(m[&(1, 0)], 40);
    assert_eq!(m[&(1, 1)], 50);
    assert_eq!(m[&(1, 2)], 60);
}

#[test]
fn to_vec() {
    let mut m = Matrix::new(2, 2, 0usize);
    m[&(0, 0)] = 0;
    m[&(0, 1)] = 1;
    m[&(1, 0)] = 10;
    m[&(1, 1)] = 11;
    assert_eq!(m.to_vec(), vec![0, 1, 10, 11]);
}

#[test]
fn square_from_vec() {
    let m = Matrix::square_from_vec(vec![10, 20, 30, 40]);
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 2);
    assert!(m.is_square());
    assert_eq!(m[&(0, 0)], 10);
    assert_eq!(m[&(0, 1)], 20);
    assert_eq!(m[&(1, 0)], 30);
    assert_eq!(m[&(1, 1)], 40);
}

#[test]
#[should_panic]
fn from_vec_panic() {
    Matrix::from_vec(2, 3, vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn square_from_vec_panic() {
    Matrix::square_from_vec(vec![1, 2, 3]);
}

#[test]
fn square_rotate() {
    // 0 1 => 2 0 => 3 2  => 1 3
    // 2 3    3 1    1 0     0 2
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3]);
    let m2 = Matrix::square_from_vec(vec![2, 0, 3, 1]);
    let m3 = Matrix::square_from_vec(vec![3, 2, 1, 0]);
    let m4 = Matrix::square_from_vec(vec![1, 3, 0, 2]);
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
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let m2 = Matrix::square_from_vec(vec![6, 3, 0, 7, 4, 1, 8, 5, 2]);
    let m3 = Matrix::square_from_vec(vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let m4 = Matrix::square_from_vec(vec![2, 5, 8, 1, 4, 7, 0, 3, 6]);
    assert_eq!(m1.rotated_cw(0), m1);
    assert_eq!(m1.rotated_cw(1), m2);
    assert_eq!(m1.rotated_cw(2), m3);
    assert_eq!(m1.rotated_cw(3), m4);
    assert_eq!(m1.rotated_ccw(0), m1);
    assert_eq!(m1.rotated_ccw(1), m4);
    assert_eq!(m1.rotated_ccw(2), m3);
    assert_eq!(m1.rotated_ccw(3), m2);
    // Same with 4
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let m2 = Matrix::square_from_vec(vec![12, 8, 4, 0, 13, 9, 5, 1, 14, 10, 6, 2, 15, 11, 7, 3]);
    let m3 = Matrix::square_from_vec(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let m4 = Matrix::square_from_vec(vec![3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12]);
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
    ]);
    let m2 = Matrix::square_from_vec(vec![
        20, 15, 10, 5, 0, 21, 16, 11, 6, 1, 22, 17, 12, 7, 2, 23, 18, 13, 8, 3, 24, 19, 14, 9, 4,
    ]);
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
fn flip() {
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3]);
    let m2 = Matrix::square_from_vec(vec![1, 0, 3, 2]);
    let m3 = Matrix::square_from_vec(vec![2, 3, 0, 1]);
    assert_eq!(m1.flipped_lr(), m2);
    assert_eq!(m1.flipped_ud(), m3);
    let m1 = Matrix::square_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let m2 = Matrix::square_from_vec(vec![2, 1, 0, 5, 4, 3, 8, 7, 6]);
    let m3 = Matrix::square_from_vec(vec![6, 7, 8, 3, 4, 5, 0, 1, 2]);
    assert_eq!(m1.flipped_lr(), m2);
    assert_eq!(m1.flipped_ud(), m3);
}

#[test]
fn transpose() {
    let m1 = Matrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
    let m2 = Matrix::from_vec(3, 2, vec![0, 3, 1, 4, 2, 5]);
    assert_eq!(m1.transposed(), m2);
    assert_eq!(m2.transposed(), m1);
}

fn sum(slice: &[usize]) -> usize {
    slice.iter().sum::<usize>()
}

#[test]
fn as_ref() {
    let m1 = Matrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(sum(m1.as_ref()), 15);
}

#[test]
fn as_mut() {
    let mut m1 = Matrix::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(sum(m1.as_ref()), 15);
    m1.as_mut()[2] = 10;
    assert_eq!(sum(m1.as_ref()), 23);
}

#[test]
fn slice() {
    let m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ]);
    let m2 = m1.slice(1..3, 2..5);
    assert_eq!(m2.rows, 2);
    assert_eq!(m2.columns, 3);
    assert_eq!(m2.as_ref().to_vec(), [7, 8, 9, 12, 13, 14]);
}

#[test]
fn set_slice() {
    let mut m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ]);
    let m2 = Matrix::from_vec(3, 2, vec![10, 20, 30, 40, 50, 60]);
    m1.set_slice(&(2, 3), &m2);
    assert_eq!(
        m1.as_ref().to_vec(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 10, 20, 15, 16, 17, 30, 40, 20, 21, 22, 50,
            60,
        ]
    );
    let mut m1 = Matrix::square_from_vec(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ]);
    let m2 = Matrix::from_vec(4, 3, vec![10, 20, 22, 30, 40, 44, 50, 60, 66, 70, 80, 88]);
    m1.set_slice(&(2, 3), &m2);
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
    m.extend(&[0, 1, 2]);
    m.extend(&[3, 4, 5]);
    assert_eq!(m.columns, 3);
    assert_eq!(m.rows, 2);
    let mut i = 0;
    for row in 0..m.rows {
        for column in 0..m.columns {
            assert_eq!(m[&(row, column)], i);
            i += 1;
        }
    }
}

#[test]
#[should_panic]
fn extend_bad_size_panic() {
    let mut m = Matrix::new_empty(3);
    m.extend(&[0, 1]);
}

#[test]
fn matrix_macro() {
    let m = matrix![[0, 1, 2], [3, 4, 5]];
    assert_eq!(m.columns, 3);
    assert_eq!(m.rows, 2);
    let mut i = 0;
    for row in 0..m.rows {
        for column in 0..m.columns {
            assert_eq!(m[&(row, column)], i);
            i += 1;
        }
    }
}

#[test]
#[should_panic]
fn matrix_macro_inconsistent_panic() {
    matrix![[0, 1, 2], [3, 4]];
}

#[test]
fn neighbours() {
    let m = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    for r in 0..3 {
        for c in 0..3 {
            for diagonal in vec![false, true] {
                let mut neighbours = m.neighbours(&(r, c), diagonal).collect::<Vec<_>>();
                neighbours.sort();
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
fn from_rows() {
    let m = Matrix::from_rows((1..3).map(|n| (1..5).map(move |x| x * n))).unwrap();
    assert_eq!(m.rows, 2);
    assert_eq!(m.columns, 4);
    assert_eq!(m.to_vec(), vec![1, 2, 3, 4, 2, 4, 6, 8]);
    let m = Matrix::from_rows((1..3).map(|n| (1..n).map(move |x| x * n)));
    assert!(m.is_err());
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
