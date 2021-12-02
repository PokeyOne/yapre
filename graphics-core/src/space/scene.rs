#[cfg(test)]
mod tests;

use crate::{
    camera::Camera,
    space::{
        lighting::Light,
        object::Object
    }
};

pub struct Scene {
    pub primary_camera: Camera,
    pub cameras: Vec<Camera>,
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new(mut cameras: Vec<Camera>, objects: Vec<Object>, lights: Vec<Box<dyn Light>>) -> Self {
        if cameras.len() == 0 {
            cameras.push(Camera::new_default());
        }

        Scene {
            primary_camera: cameras.remove(0),
            cameras,
            objects,
            lights,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(Vec::new(), Vec::new(), Vec::new())
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_primary_camera(&self) -> &Camera {
        &self.primary_camera
    }
}