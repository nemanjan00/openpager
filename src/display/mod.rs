//! Display abstraction for OpenPager
//!
//! - MIPS (embedded): Linux framebuffer
//! - x86_64 (dev): Window via minifb

#[cfg(target_arch = "mips")]
pub mod framebuffer;
#[cfg(target_arch = "mips")]
pub use framebuffer::{rgb_to_565, Framebuffer as Display, RenderBuffer, Rgb565};

#[cfg(not(target_arch = "mips"))]
pub mod window;
#[cfg(not(target_arch = "mips"))]
pub use window::{rgb_to_565, RenderBuffer, Rgb565, WindowDisplay as Display};
