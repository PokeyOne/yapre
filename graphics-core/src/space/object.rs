use super::{
    Triangle,
    transform::Transform
};

pub struct Mesh {
    pub triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh {
            triangles: triangles
        }
    }

    pub fn transformed_mesh(&self, transform: &Transform) -> Mesh {
        let mut new_triangles = Vec::new();
        for triangle in &self.triangles {
            new_triangles.push(triangle.transformed_triangle(transform));
        }
        Mesh {
            triangles: new_triangles
        }
    }
}

pub struct Object {
    pub mesh: Mesh,
    pub transform: Transform
}

impl Object {
    pub fn new(mesh: Mesh) -> Object {
        Object {
            mesh,
            transform: Transform::new()
        }
    }

    pub fn squash_transforms(&mut self) {
        self.mesh = self.mesh.transformed_mesh(&self.transform);
        self.transform = Transform::new();
    }

    // TODO: It may be a useful optimization at some point to cache the transformed mesh.
    pub fn triangles(&self) -> Vec<Triangle> {
        self.mesh.transformed_mesh(&self.transform).triangles
    }
}