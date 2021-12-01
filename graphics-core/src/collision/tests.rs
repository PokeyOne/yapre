use super::*;
use crate::space::*;
use super::Collidable;

#[test]
fn basic_collision_test() {
    let triangle = Triangle::new([
        Point::new(0.0, 1.0, 1.0),
        Point::new(1.0, -1.0, 1.0),
        Point::new(-1.0, -1.0, 1.0)
    ]);

    let ray = Ray::new(ORIGIN, Point::new(0.0, 0.0, 1.0));

    match triangle.intersection_point(&ray) {
        None => panic!("Ray should intersect"),
        Some(loc) => {
            assert_eq!(Point::new(0.0, 0.0, 1.0), loc)
        }
    };
}
