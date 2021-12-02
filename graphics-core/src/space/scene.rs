use crate::{
    camera::Camera,
    space::{
        lighting::Light,
        object::Object
    }
};

pub struct Scene {
    pub primary_camera: &Camera,
    pub cameras: Vec<Camera>,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(cameras: Vec<Camera>, objects: Vec<Object>, lights: Vec<Light>) -> Self {
        if cameras.len() == 0 {
            cameras.push(Camera::new_default());
        }

        Scene {
            primary_camera: &cameras[0],
            cameras,
            objects,
            lights,
        }
    }

    pub fn new_empty() -> Self {
        Self::new()
    }
}