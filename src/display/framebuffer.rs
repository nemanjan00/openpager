use framebuffer::Framebuffer as Fb;
use std::io;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

/// RGB565 pixel type
pub type Rgb565 = u16;

/// Convert RGB888 to RGB565
#[inline(always)]
pub fn rgb_to_565(r: u8, g: u8, b: u8) -> Rgb565 {
    ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3)
}

/// Linux framebuffer display wrapper
pub struct Framebuffer {
    fb: Fb,
}

impl Framebuffer {
    /// Open framebuffer device (default: /dev/fb0)
    pub fn new(device: Option<&str>) -> io::Result<Self> {
        let path = device.unwrap_or("/dev/fb0");
        let fb = Fb::new(path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Self { fb })
    }

    /// Get framebuffer width in pixels
    pub fn width(&self) -> u32 {
        self.fb.var_screen_info.xres
    }

    /// Get framebuffer height in pixels
    pub fn height(&self) -> u32 {
        self.fb.var_screen_info.yres
    }

    /// Get bits per pixel
    pub fn bpp(&self) -> u32 {
        self.fb.var_screen_info.bits_per_pixel
    }

    /// Get line length in bytes
    pub fn line_length(&self) -> u32 {
        self.fb.fix_screen_info.line_length
    }

    /// Check if display is 16-bit RGB565
    pub fn is_rgb565(&self) -> bool {
        self.fb.var_screen_info.bits_per_pixel == 16
    }

    /// Get mutable access to framebuffer memory
    pub fn frame_mut(&mut self) -> &mut [u8] {
        &mut self.fb.frame
    }

    /// Get framebuffer as RGB565 slice (assumes 16-bit mode)
    pub fn frame_rgb565_mut(&mut self) -> &mut [Rgb565] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.fb.frame.as_mut_ptr() as *mut Rgb565,
                self.fb.frame.len() / 2,
            )
        }
    }

    /// Clear to color (RGB565)
    pub fn clear(&mut self, color: Rgb565) {
        let frame = self.frame_rgb565_mut();
        frame.fill(color);
    }

    /// Set pixel directly in framebuffer
    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        if x < self.width() && y < self.height() {
            let line_length = self.line_length() as usize;
            let offset = y as usize * line_length / 2 + x as usize;
            self.frame_rgb565_mut()[offset] = color;
        }
    }
}

/// Render buffer at internal resolution, then blit to framebuffer
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

    /// Create with default display resolution
    pub fn default_resolution() -> Self {
        Self::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    /// Clear to color
    pub fn clear(&mut self, color: Rgb565) {
        self.pixels.fill(color);
    }

    /// Set pixel
    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }

    /// Blit to framebuffer with scaling and optional rotation
    /// rotation: 0 = none, 1 = 90° CW, 2 = 180°, 3 = 90° CCW
    pub fn blit_scaled(&self, fb: &mut Framebuffer, rotation: u8) {
        let fb_w = fb.width();
        let fb_h = fb.height();
        let fb_buf = fb.frame_rgb565_mut();

        match rotation {
            0 => {
                // No rotation - scale directly
                for fb_y in 0..fb_h {
                    let src_y = (fb_y * self.height / fb_h) as usize;
                    for fb_x in 0..fb_w {
                        let src_x = (fb_x * self.width / fb_w) as usize;
                        let src_idx = src_y * self.width as usize + src_x;
                        let dst_idx = (fb_y * fb_w + fb_x) as usize;
                        fb_buf[dst_idx] = self.pixels[src_idx];
                    }
                }
            }
            3 => {
                // 90° CCW - like the DOOM port
                for fb_y in 0..fb_h {
                    let src_x = ((fb_h - 1 - fb_y) * self.width / fb_h) as usize;
                    for fb_x in 0..fb_w {
                        let src_y = (fb_x * self.height / fb_w) as usize;
                        let src_idx = src_y * self.width as usize + src_x;
                        let dst_idx = (fb_y * fb_w + fb_x) as usize;
                        fb_buf[dst_idx] = self.pixels[src_idx];
                    }
                }
            }
            _ => {}
        }
    }
}
