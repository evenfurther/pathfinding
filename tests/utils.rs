use pathfinding::utils::*;

#[test]
fn absdiff_signed() {
    assert_eq!(1i32, absdiff(3i32, 2i32));
    assert_eq!(1i32, absdiff(2i32, 3i32));
}

#[test]
fn absdiff_unsigned() {
    assert_eq!(1u32, absdiff(3u32, 2u32));
    assert_eq!(1u32, absdiff(2u32, 3u32));
}

#[test]
fn absdiff_float() {
    assert!((absdiff(3f32, 2f32) - 1f32).abs() < 1e-10);
    assert!((absdiff(2f32, 3f32) - 1f32).abs() < 1e-10);
}
