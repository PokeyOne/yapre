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