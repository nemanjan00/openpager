//! Display abstraction for OpenPager
//!
//! Supports:
//! - Linux framebuffer (real hardware)
//! - Mock display (x86_64 development)

#[cfg(target_os = "linux")]
pub mod framebuffer;

#[cfg(target_os = "linux")]
pub use framebuffer::{argb_to_565, rgb_to_565, Framebuffer, RenderBuffer, Rgb565};

/// Common display trait for abstracting over different backends
pub trait Display {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: u16);
    fn set_pixel(&mut self, x: u32, y: u32, color: u16);
    fn flush(&mut self) -> std::io::Result<()>;
}

#[cfg(target_os = "linux")]
impl Display for Framebuffer {
    fn width(&self) -> u32 {
        self.width()
    }

    fn height(&self) -> u32 {
        self.height()
    }

    fn clear(&mut self, color: u16) {
        Framebuffer::clear(self, color);
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: u16) {
        Framebuffer::set_pixel(self, x, y, color);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Framebuffer::flush(self)
    }
}
