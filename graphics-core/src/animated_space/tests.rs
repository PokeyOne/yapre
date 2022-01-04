use super::*;

#[test]
fn from_point() {
    let point = Point::new(1, 2, 3);
    let anim_point = AnimatedPoint::from_point(point);
}
