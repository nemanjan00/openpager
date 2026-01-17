use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::RawData,
    Pixel,
};

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

/// Render buffer that implements embedded-graphics DrawTarget
pub struct RenderBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u16>,
}

impl RenderBuffer {
    pub fn default_resolution() -> Self {
        Self {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            pixels: vec![0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize],
        }
    }

    pub fn pixels_raw(&self) -> &[u16] {
        &self.pixels
    }
}

#[inline(always)]
fn raw_rgb565(color: Rgb565) -> u16 {
    RawU16::from(color).into_inner()
}

impl OriginDimensions for RenderBuffer {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl DrawTarget for RenderBuffer {
    type Color = Rgb565;
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
                let raw = raw_rgb565(color);
                self.pixels[(coord.y as u32 * self.width + coord.x as u32) as usize] = raw;
            }
        }
        Ok(())
    }
}
