use minifb::{Key, Window, WindowOptions};
use std::io;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

/// Convert RGB565 to RGB888 (for minifb which uses u32)
#[inline(always)]
fn rgb565_to_u32(pixel: u16) -> u32 {
    let r = ((pixel >> 11) & 0x1F) as u32;
    let g = ((pixel >> 5) & 0x3F) as u32;
    let b = (pixel & 0x1F) as u32;
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

    pub fn default_resolution(scale: usize) -> io::Result<Self> {
        Self::new("OpenPager", DISPLAY_WIDTH, DISPLAY_HEIGHT, scale)
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn get_keys(&self) -> Option<Vec<Key>> {
        let keys = self.window.get_keys();
        if keys.is_empty() {
            None
        } else {
            Some(keys)
        }
    }

    /// Blit from raw RGB565 buffer (no rotation for dev)
    pub fn blit(&mut self, src: &[u16], src_w: u32, src_h: u32) {
        for y in 0..src_h.min(self.height) {
            for x in 0..src_w.min(self.width) {
                let src_idx = (y * src_w + x) as usize;
                let dst_idx = (y * self.width + x) as usize;
                self.buffer[dst_idx] = rgb565_to_u32(src[src_idx]);
            }
        }
    }

    pub fn update(&mut self) -> io::Result<()> {
        self.window
            .update_with_buffer(&self.buffer, self.width as usize, self.height as usize)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
