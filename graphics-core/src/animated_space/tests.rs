use super::*;
use crate::animation::KeyFrame;

#[test]
fn test_from_point() {
    let point = Point::new(1, 2, 3);
    let anim_point = AnimatedPoint::from_point(point);

    for value in anim_point.values {
        assert!(value.is_constant());
    }
}

#[test]
fn get_value_of_constant() {
    let input_point = Point::new(1, 2, 3);
    let c = AnimatedPoint::from_point(input_point.clone());
    assert_eq!(c.get_value(0), input_point);
    assert_eq!(c.get_value(1), input_point);
}

#[test]
fn get_value_of_linear() {
    let a = AnimatedPoint::new([
        AnimatedValue::linear(vec![KeyFrame::new(0, 0.0), KeyFrame::new(1, 1.0)]),
        AnimatedValue::linear(vec![KeyFrame::new(0, 1.0), KeyFrame::new(1, 2.0)]),
        AnimatedValue::linear(vec![KeyFrame::new(0, 2.0), KeyFrame::new(1, 3.0)])
    ]);
    assert_eq!(a.get_value(0), Point::new(0, 1, 2));
    assert_eq!(a.get_value(1), Point::new(1, 2, 3));
}
