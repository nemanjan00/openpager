use minifb::{Window, WindowOptions};
use std::io;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

/// RGB565 pixel type
pub type Rgb565 = u16;

/// Convert RGB888 to RGB565
#[inline(always)]
pub fn rgb_to_565(r: u8, g: u8, b: u8) -> Rgb565 {
    ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3)
}

/// Convert RGB565 to RGB888 (for minifb which uses u32)
#[inline(always)]
fn rgb565_to_u32(pixel: Rgb565) -> u32 {
    let r = ((pixel >> 11) & 0x1F) as u32;
    let g = ((pixel >> 5) & 0x3F) as u32;
    let b = (pixel & 0x1F) as u32;
    // Expand to 8-bit
    let r = (r << 3) | (r >> 2);
    let g = (g << 2) | (g >> 4);
    let b = (b << 3) | (b >> 2);
    (r << 16) | (g << 8) | b
}

/// Windowed display for development
pub struct WindowDisplay {
    window: Window,
    buffer: Vec<u32>,
    width: u32,
    height: u32,
}

impl WindowDisplay {
    pub fn new(title: &str, width: u32, height: u32, scale: usize) -> io::Result<Self> {
        let window = Window::new(
            title,
            width as usize * scale,
            height as usize * scale,
            WindowOptions {
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                ..Default::default()
            },
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let buffer = vec![0u32; (width * height) as usize];

        Ok(Self {
            window,
            buffer,
            width,
            height,
        })
    }

    /// Create with default display resolution
    pub fn default_resolution(scale: usize) -> io::Result<Self> {
        Self::new("OpenPager", DISPLAY_WIDTH, DISPLAY_HEIGHT, scale)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Check if window is still open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// Clear to color
    pub fn clear(&mut self, color: Rgb565) {
        let color32 = rgb565_to_u32(color);
        self.buffer.fill(color32);
    }

    /// Set pixel
    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        if x < self.width && y < self.height {
            self.buffer[(y * self.width + x) as usize] = rgb565_to_u32(color);
        }
    }

    /// Update window with buffer contents
    pub fn update(&mut self) -> io::Result<()> {
        self.window
            .update_with_buffer(&self.buffer, self.width as usize, self.height as usize)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

/// Render buffer - same as framebuffer version
pub struct RenderBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Rgb565>,
}

impl RenderBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height) as usize],
        }
    }

    pub fn default_resolution() -> Self {
        Self::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    pub fn clear(&mut self, color: Rgb565) {
        self.pixels.fill(color);
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }

    /// Blit to window display (no rotation needed for dev)
    pub fn blit(&self, display: &mut WindowDisplay) {
        for y in 0..self.height.min(display.height()) {
            for x in 0..self.width.min(display.width()) {
                let color = self.pixels[(y * self.width + x) as usize];
                display.set_pixel(x, y, color);
            }
        }
    }
}
