#![feature(test)]
extern crate test;

use test::Bencher;
use yapre_graphics_core::{
    space::{
        Point, ORIGIN, Triangle,
        transform::TransformStep,
        object::{Mesh, Object},
        scene::Scene
    },
    camera::{Renderer, OrthographicCamera, Camera},
    images::Color,
    material::Material
};

fn test_diamond() -> Object {
    let mut triangles = Vec::new();
    let points = vec![
        Point::new(1.0, 0.0, 0.0),
        Point::new(-1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
        Point::new(0.0, -1.0, 0.0),
        Point::new(0.0, 0.0, 1.0),
        Point::new(0.0, 0.0, -1.0)
    ];
    let trip = vec![
        (0, 2, 3),
        (0, 3, 4),
        (0, 4, 5),
        (0, 5, 2),
        (1, 2, 3),
        (1, 3, 4),
        (1, 4, 5),
        (1, 5, 2)
    ];

    for t in trip {
        let mut triangle = Triangle::new([points[t.0], points[t.1], points[t.2]]);
        triangle.set_material(Material::new(Color::random()));
        triangles.push(triangle);
    }

    Object::new(Mesh::new(triangles))
}

#[bench]
fn small_image_with_single_object(b: &mut Bencher) {
    let cam = OrthographicCamera::new(ORIGIN, 3.0, 3.0);
    let mut scene = Scene::new(vec![Camera::Ortho(cam)], Vec::new(), Vec::new());
    let mut obj = test_diamond();
    obj.transform.add_step(TransformStep::Translate(Point::new(0.0, 0.0, 5.0)));
    obj.squash_transforms();
    scene.add_object(obj);

    b.iter(|| scene.get_primary_camera().render(&scene, (100, 100)));
}

#[bench]
fn megapixel_image_with_single_object(b: &mut Bencher) {
    let cam = OrthographicCamera::new(ORIGIN, 3.0, 3.0);
    let mut scene = Scene::new(vec![Camera::Ortho(cam)], Vec::new(), Vec::new());
    let mut obj = test_diamond();
    obj.transform.add_step(TransformStep::Translate(Point::new(0.0, 0.0, 5.0)));
    obj.squash_transforms();
    scene.add_object(obj);

    b.iter(|| scene.get_primary_camera().render(&scene, (1000, 1000)));
}
