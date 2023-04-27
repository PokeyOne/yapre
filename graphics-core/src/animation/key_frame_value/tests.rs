use super::*;

#[test]
fn test_integer_interpolation() {
    assert_eq!(1_i32.interpolate(&2_i32, 0.5), 2_i32);
}

#[test]
fn test_float_interpolation() {
    assert_eq!(1_f32.interpolate(&2_f32, 0.5), 1.5_f32);
}
