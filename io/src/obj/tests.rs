use super::*;
use yapre_graphics_core::space::object::Mesh;
use yapre_graphics_core::space::*;

#[test]
fn point_registry_test() {
    let mut registry = PointRegistry::new();

    // Test with just one point
    let point = registry.add_point(ORIGIN);
    assert_eq!(registry.get_point(point), Some(ORIGIN));

    // Add another point and the old point should still be there
    let other_point = registry.add_point(Point::new(1.0, 2.0, 3.0));
    assert_eq!(
        registry.get_point(other_point),
        Some(Point::new(1.0, 2.0, 3.0))
    );
    assert_eq!(registry.get_point(point), Some(ORIGIN));

    // Asking for a non-existent point should return None
    assert_eq!(registry.get_point(point + other_point + 5), None);

    // Adding a point that already exists should return the existing point
    let point_again = registry.add_point(ORIGIN);
    assert_eq!(point_again, point);
}

#[test]
fn test_generate_obj_file_for_one_triangle() {
    let t = Triangle::new([
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0)
    ]);
    let mesh = Mesh::new(vec![t]);
    let object = ObjectCore::new(mesh);

    let generated_obj_file = generate_obj_file(&object);

    assert_eq!(
        generated_obj_file,
        "# OBJ file generated by yapre\n\
        v 0.000000 0.000000 0.000000\n\
        v 1.000000 0.000000 0.000000\n\
        v 0.000000 1.000000 0.000000\n\
        f 1 2 3\n"
    );
}
