//! UI components

pub mod colors;
mod menu;
mod statusbar;

pub use menu::{Menu, MenuItem, MenuAction};
pub use statusbar::{StatusBar, STATUSBAR_HEIGHT};

use crate::platform::{Action, RenderBuffer};

pub enum ViewResult {
    None,
    Push(Box<dyn View>),
    Pop,
    Exit,
}

pub trait View {
    fn render(&self, buffer: &mut RenderBuffer);
    fn handle(&mut self, action: Action) -> ViewResult;
}
