//! Display abstraction for OpenPager

pub mod render;
pub use render::RenderBuffer;

#[cfg(target_arch = "mips")]
pub mod framebuffer;
#[cfg(target_arch = "mips")]
pub use framebuffer::Framebuffer;

#[cfg(not(target_arch = "mips"))]
pub mod window;
#[cfg(not(target_arch = "mips"))]
pub use window::WindowDisplay;
