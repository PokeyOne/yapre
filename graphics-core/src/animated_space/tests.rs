use super::*;

#[test]
fn test_from_point() {
    let point = Point::new(1, 2, 3);
    let anim_point = AnimatedPoint::from_point(point);

    for value in anim_point.values {
        assert!(value.is_constant());
    }
}
