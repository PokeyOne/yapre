use super::*;
use crate::space::*;
use super::Collidable;

fn create_test_triangle() -> Triangle {
    Triangle::new([
        Point::new(0.0, 1.0, 1.0),
        Point::new(1.0, -1.0, 1.0),
        Point::new(-1.0, -1.0, 1.0)
    ])
}

#[test]
fn basic_collision_test() {
    let triangle = create_test_triangle();
    let ray = Ray::new(ORIGIN, Point::new(0.0, 0.0, 1.0));

    match triangle.intersection_point(&ray) {
        None => panic!("Ray should intersect"),
        Some(Collision { point, distance }) => {
            assert_eq!(Point::new(0.0, 0.0, 1.0), point);
            assert_eq!(1.0, distance);
        }
    };
}

#[test]
fn ray_backwards_into_triangle() {
    let triangle = create_test_triangle();
    let ray = Ray::new(ORIGIN, Point::new(0.0, 0.0, -1.0));

    match triangle.intersection_point(&ray) {
        None => panic!("Ray should not miss"),
        Some(Collision { point, distance }) => {
            assert_eq!(Point::new(0.0, 0.0, 1.0), point);
            assert_eq!(-1.0, distance);
        }
    };
}
