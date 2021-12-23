use std::path::Path;
use super::*;
use crate::camera::OrthographicCamera;
use crate::camera::Renderer;
use crate::images::Color;
use crate::material::Material;
use crate::space::object::{Mesh, Object};
use crate::space::transform::TransformStep;
use crate::space::{Point, Triangle, ORIGIN};

fn test_diamond() -> Object {
    let mut triangles = Vec::new();
    let points = vec![
        Point::new(1.0, 0.0, 0.0),
        Point::new(-1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
        Point::new(0.0, -1.0, 0.0),
        Point::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
    ];
    let trip = vec![
        (0, 2, 3),
        (0, 3, 4),
        (0, 4, 5),
        (0, 5, 2),
        (1, 2, 3),
        (1, 3, 4),
        (1, 4, 5),
        (1, 5, 2),
    ];

    for t in trip {
        let mut triangle = Triangle::new([points[t.0], points[t.1], points[t.2]]);
        triangle.set_material(Material::new(Color::random()));
        triangles.push(triangle);
    }

    Object::new(Mesh::new(triangles))
}

#[test]
fn test_scene_diamond_render() {
    let cam = OrthographicCamera::new(ORIGIN, 3.0, 3.0);
    let mut scene = Scene::new(vec![Camera::Ortho(cam)], Vec::new(), Vec::new());
    let mut obj = test_diamond();
    obj.transform
        .add_step(TransformStep::Translate(Point::new(0.0, 0.0, 5.0)));
    obj.squash_transforms();
    scene.add_object(obj);

    let img = scene.get_primary_camera().render(&scene, (100, 100));
    match img.save_to_temp_path() {
        Ok(p) => {
            println!("Saved to {}", p);
            let path = Path::new(&p);
            assert!(path.exists());
            std::fs::remove_file(path).unwrap();
        }
        Err(e) => panic!("{}", e),
    };
}
