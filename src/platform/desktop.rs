use minifb::{Key, Window, WindowOptions};
use std::io;

use super::{Action, RenderBuffer};
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub fn read_battery() -> (u8, bool) {
    (100, false)
}

pub struct DesktopPlatform {
    window: Window,
}

impl DesktopPlatform {
    pub fn new() -> io::Result<Self> {
        let scale = 2;
        let window = Window::new(
            "OpenPager",
            DISPLAY_WIDTH as usize * scale,
            DISPLAY_HEIGHT as usize * scale,
            WindowOptions {
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                ..Default::default()
            },
        )
        .map_err(io::Error::other)?;

        Ok(Self { window })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn poll(&mut self) -> Option<Action> {
        for key in self.window.get_keys() {
            match key {
                Key::Up => return Some(Action::Up),
                Key::Down => return Some(Action::Down),
                Key::Enter => return Some(Action::Select),
                Key::Escape => return Some(Action::Back),
                _ => {}
            }
        }
        None
    }

    pub fn draw(&mut self, render: &RenderBuffer) {
        let _ = self.window.update_with_buffer(
            render.pixels_raw(),
            DISPLAY_WIDTH as usize,
            DISPLAY_HEIGHT as usize,
        );
    }

    pub fn wait(&self) {
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
