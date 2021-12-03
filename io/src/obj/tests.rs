use super::*;
use yapre_graphics_core::space::*;

#[test]
fn point_registry_test() {
    let mut registry = PointRegistry::new();
    let point = registry.add_point(ORIGIN);
    assert_eq!(registry.get_point(point), Some(ORIGIN));
    let other_point = registry.add_point(Point::new(1.0, 2.0, 3.0));
    assert_eq!(registry.get_point(other_point), Some(Point::new(1.0, 2.0, 3.0)));
    assert_eq!(registry.get_point(point), Some(ORIGIN));
}