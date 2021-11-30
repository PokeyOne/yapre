use super::*;

#[test]
fn test_point_construction() {
    let p = Point::new(1.0, 2.0, 3.0);

    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    assert_eq!(p.z, 3.0);
}

#[test]
fn test_point_origin() {
    assert_eq!(0.0, ORIGIN.x);
    assert_eq!(0.0, ORIGIN.y);
    assert_eq!(0.0, ORIGIN.z);
}

#[test]
fn test_point_add() {
    let a = Point::new(1.0, 2.0, 3.0);
    let b = Point::new(2.0, 1.0, 0.0);
    let a_b_expected = Point::new(3.0, 3.0, 3.0);

    assert_eq!(a, a.clone() + ORIGIN);
    assert_eq!(a, a + ORIGIN);
    assert_eq!(a_b_expected, a + b);
}

#[test]
fn test_point_sub() {
    let a = Point::new(1.0, 2.0, 3.0);
    let b = Point::new(2.0, 1.0, 0.0);
    let a_b_expected = Point::new(-1.0, 1.0, 3.0);
    let b_a_expected = Point::new(1.0, -1.0, -3.0);

    assert_eq!(a, a.clone() - ORIGIN);
    assert_eq!(a, a - ORIGIN);
    assert_eq!(a_b_expected, a - b);
    assert_eq!(b_a_expected, b - a);
}

#[test]
fn test_point_scaling_function() {
    let a = Point::new(1.0, 2.0, 3.0);

    assert_eq!(a, a.scale(1.0));
    assert_eq!(ORIGIN, a.scale(0.0));
    assert_eq!(ORIGIN, ORIGIN.scale(1000.0));
    assert_eq!(Point { x: 3.0, y: 6.0, z: 9.0 }, a.scale(3.0));
}

#[test]
fn test_point_scaling_op() {
    let a = Point::new(1.0, 2.0, 3.0);

    assert_eq!(a, a * 1.0);
    assert_eq!(ORIGIN, a * 0.0);
    assert_eq!(ORIGIN, ORIGIN * 1000.0);
    assert_eq!(Point { x: 3.0, y: 6.0, z: 9.0 }, a * 3.0);
}

#[test]
fn point_magnitude() {
    let a = Point::new(0.0, 3.0, 4.0);

    assert_eq!(ORIGIN.length(), 0.0);
    assert_eq!(a.length(), 5.0);
}

#[test]
fn test_point_equality() {
    let a = ORIGIN;
    let b = ORIGIN;
    let c = Point::new(1.0, 2.0, 3.0);
    let d = Point::new(1.0, 2.0, 3.0);
    let e = Point::new(1.0, 3.0, 2.0);

    assert_eq!(a, b);
    assert_eq!(c, d);
    assert_ne!(b, c);
    assert_ne!(e, c);
}

#[test]
fn test_triangle_equality_with_same_order_points() {
    let a = Triangle::new([
        ORIGIN,
        Point::new(1.0, 1.0, 1.0),
        Point::new(-1.0, 1.0, 1.0)
    ]);
    let b = a.clone();

    assert_eq!(a, b);
}

#[test]
fn test_triangle_equality_with_different_order_points() {
    let a = Triangle::new([
        ORIGIN,
        Point::new(1.0, 1.0, 1.0),
        Point::new(-1.0, 1.0, 1.0)
    ]);
    let b = Triangle::new([
        Point::new(1.0, 1.0, 1.0),
        ORIGIN,
        Point::new(-1.0, 1.0, 1.0)
    ]);

    assert_eq!(a, b);
}

#[test]
fn test_triangle_equality_not_equal() {
    let a = Triangle::new([
        ORIGIN,
        Point::new(1.0, 1.0, 1.0),
        Point::new(-1.0, 1.0, 1.0)
    ]);
    let b = Triangle::new([
        ORIGIN,
        Point::new(1.0, 4.0, 1.0),
        Point::new(-1.0, 1.0, 1.0)
    ]);

    assert_ne!(a, b);
}

#[test]
#[should_panic]
fn triangle_with_duplicate_points_should_panic() {
    let a = Triangle::new([
        ORIGIN,
        ORIGIN,
        Point::new(1.0, 0.0, 0.0)
    ]);
}
