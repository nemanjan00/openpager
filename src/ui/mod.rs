//! UI components

pub mod colors;
mod icons;
mod menu;
mod statusbar;

pub use menu::{Menu, MenuAction, MenuItem};
pub use statusbar::{STATUSBAR_HEIGHT, StatusBar};

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
