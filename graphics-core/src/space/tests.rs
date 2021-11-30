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

// TODO: test triangle equality with duplicate points

#[test]
fn test_ref_triangle_into_triangle() {
    let expected_triangle = Triangle::new([
        ORIGIN,
        Point::new(1.0, 1.0, 1.0),
        Point::new(-1.0, 1.0, 1.0)
    ]);
    let tref = RefTriangle::new([
        &expected_triangle.points[0],
        &expected_triangle.points[1],
        &expected_triangle.points[2]
    ]);

    assert_eq!(expected_triangle.points, tref.into_triangle().points);
}
