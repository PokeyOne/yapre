use crate::space::{
    Point, Triangle, Vector,
    scene::Scene
};
use crate::images::{RawImage, Color, WHITE, BLACK};
use crate::collision::{Collidable, Ray, Collision};
use std::rc::Rc;

pub enum Camera {
    Ortho(OrthographicCamera)
}

impl Camera {
    pub fn new_default() -> Camera {
        // TODO: The default camera should be perspective, but that doesn't exist yet.
        Camera::Ortho(OrthographicCamera::new_default())
    }
}

pub struct OrthographicCamera {
    location: Point,
    width: f64,
    height: f64 // TODO: Direction, OutputImageSpec
}

impl OrthographicCamera {
    pub fn new(location: Point, width: f64, height: f64) -> Self {
        OrthographicCamera { location, width, height }
    }

    pub fn new_default() -> Self {
        OrthographicCamera::new(Point::new(0.0, 0.0, 0.0), 1.0, 1.0)
    }
}

pub trait Renderer {
    fn render(&self, scene: &Scene, image_size: (usize, usize)) -> RawImage;
}

impl Renderer for Camera {
    fn render(&self, scene: &Scene, image_size: (usize, usize)) -> RawImage {
        match self {
            Camera::Ortho(camera) => camera.render(scene, image_size)
        }
    }
}

impl Renderer for OrthographicCamera {
    fn render(&self, scene: &Scene, image_size: (usize, usize)) -> RawImage {
        let mut output_image = RawImage::new(image_size.0, image_size.1);

        for j in 0..(image_size.1) {
            let y: f64 = ((-(j as f64) / (image_size.1 as f64)) + 0.5) * self.height;
            for i in 0..(image_size.0) {
                let x: f64 = (((i as f64) / (image_size.0 as f64)) - 0.5) * self.width;

                let ray = Ray::new(Point::new(x, y, 0.0) + self.location, Point::new(0.0, 0.0, 1.0));

                let mut closest_collision: Option<Collision> = None;
                for obj in &scene.objects {
                    for tri in obj.triangles() {
                        match tri.intersection_point(&ray) {
                            None => {},
                            Some(cl) => match &closest_collision {
                                None => closest_collision = Some(cl),
                                Some(closest) => if cl.distance < closest.distance {
                                    closest_collision = Some(cl)
                                }
                            }
                        };
                    }
                }
                match closest_collision {
                    None => output_image.set_pixel(BLACK, j, i),
                    Some(_) => output_image.set_pixel(WHITE, j, i)
                }
            }
        }

        output_image
    }
}
