use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::surface::Surface;

pub struct RawImage<'a> {
    canvas: Canvas<Surface<'a>>
}

impl RawImage<'_> {
    pub fn new(width: u32, height: u32, pixel_format: PixelFormatEnum) -> Result<Self, String> {
        let surface = Surface::new(width, height, pixel_format)?;
        let canvas = surface.into_canvas()?;

        Ok(RawImage { canvas })
    }
}
