//! Platform abstraction for display and input

mod render;
pub use render::RenderBuffer;

#[cfg(target_arch = "mips")]
mod device;
#[cfg(target_arch = "mips")]
pub use device::DevicePlatform as Platform;

#[cfg(not(target_arch = "mips"))]
mod desktop;
#[cfg(not(target_arch = "mips"))]
pub use desktop::DesktopPlatform as Platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Select,
    Back,
}
