use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
    Pixel,
};

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct RenderBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl RenderBuffer {
    pub fn default_resolution() -> Self {
        Self {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            pixels: vec![0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize],
        }
    }

    pub fn pixels_raw(&self) -> &[u32] {
        &self.pixels
    }
}

impl OriginDimensions for RenderBuffer {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl DrawTarget for RenderBuffer {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0
                && coord.x < self.width as i32
                && coord.y >= 0
                && coord.y < self.height as i32
            {
                let raw = (color.r() as u32) << 16 | (color.g() as u32) << 8 | color.b() as u32;
                self.pixels[(coord.y as u32 * self.width + coord.x as u32) as usize] = raw;
            }
        }
        Ok(())
    }
}
