use crate::images::Color;

// TODO: This will probably need more than just color
#[derive(Debug, Clone)]
pub struct Material {
    color: Color
}

impl Material {
    pub fn new(color: Color) -> Material {
        Material { color }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn default() -> Material {
        Material::new(Color::from_rgba(0xAAAAAAAAu32))
    }
}
