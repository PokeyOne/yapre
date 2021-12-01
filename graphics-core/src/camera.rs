use crate::space::{Point, Triangle};
use crate::images::{RawImage, Color, WHITE, BLACK};
use crate::collision::{Collidable, Ray};

pub enum Camera {
    Ortho(OrthographicCamera)
}

pub trait Renderer {
    // TODO: In the future this should be a Scene that has Mesh objects
    fn render(&self, tri: Triangle, image_size: (usize, usize)) -> RawImage;
}

pub struct OrthographicCamera {
    location: Point,
    width: f64,
    height: f64
}

impl Renderer for OrthographicCamera {
    fn render(&self, tri: Triangle, image_size: (usize, usize)) -> RawImage {
        let mut output_image = RawImage::new(image_size.0, image_size.1);

        for j in 0..(image_size.1) {
            let y: f64 = ((-(j as f64) / (image_size.1 as f64)) + 0.5) * self.height;
            for i in 0..(image_size.0) {
                let x: f64 = (((i as f64) / (image_size.0 as f64)) - 0.5) * self.width;

                let ray = Ray::new(Point::new(x, y, 0.0) + self.location, Point::new(0.0, 0.0, 1.0));
                match tri.intersection_point(&ray) {
                    None => output_image.set_pixel(BLACK, j, i),
                    Some(_) => output_image.set_pixel(WHITE, j, i)
                };
            }
        }

        output_image
    }
}
