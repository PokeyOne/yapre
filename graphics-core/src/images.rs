#[cfg(test)]
mod tests;

use std::path::Path;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::rect::Point as SdlPoint;

use uuid::Uuid;

pub struct RawImage<'a> {
    canvas: Canvas<Surface<'a>>
}

impl RawImage<'_> {
    pub fn new(width: u32, height: u32, pixel_format: PixelFormatEnum) -> Result<Self, String> {
        let surface = Surface::new(width, height, pixel_format)?;
        let canvas = surface.into_canvas()?;

        Ok(RawImage { canvas })
    }

    pub fn set_pixel(&mut self, c: Color, x: i32, y: i32) -> Result<(), String> {
        self.canvas.set_draw_color(c);
        self.canvas.draw_point(SdlPoint::new(x, y))?;

        Ok(())
    }

    /// Output the image to a file in a temporary directory then return the
    /// path. This method returns a Result and if the file it save it will
    /// return the path to the file, and if it fails then it will be a string
    /// with an error method.
    pub fn save_to_temp_path(&self) -> Result<String, String> {
        let path_string = generate_random_temp_path();
        let path = Path::new(&path_string);
        println!("Path would be {:?}", path_string);

        Err(String::from("Not implemented"))
    }
}

fn generate_random_temp_path() -> String {
    let uuid = Uuid::new_v4();
    format!("./tmp-yapre/temp_image_{}.bmp", uuid)
}
