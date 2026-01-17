use minifb::{Key, Window, WindowOptions};
use std::io;

use super::{Action, RenderBuffer};
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

fn rgb565_to_u32(pixel: u16) -> u32 {
    let r = ((pixel >> 11) & 0x1F) as u32;
    let g = ((pixel >> 5) & 0x3F) as u32;
    let b = (pixel & 0x1F) as u32;
    let r = (r << 3) | (r >> 2);
    let g = (g << 2) | (g >> 4);
    let b = (b << 3) | (b >> 2);
    (r << 16) | (g << 8) | b
}

pub struct DesktopPlatform {
    window: Window,
    buffer: Vec<u32>,
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
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let buffer = vec![0u32; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];

        Ok(Self { window, buffer })
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
        for (i, &pixel) in render.pixels_raw().iter().enumerate() {
            self.buffer[i] = rgb565_to_u32(pixel);
        }
        let _ = self.window.update_with_buffer(
            &self.buffer,
            DISPLAY_WIDTH as usize,
            DISPLAY_HEIGHT as usize,
        );
    }

    pub fn wait(&self) {
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
