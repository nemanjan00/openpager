// Effective landscape resolution after 90° rotation of 222x480 framebuffer
pub const DISPLAY_WIDTH: u32 = 480;
pub const DISPLAY_HEIGHT: u32 = 222;

mod display;
mod ui;

#[cfg(target_arch = "mips")]
mod input;

use display::RenderBuffer;
use ui::Menu;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

#[cfg(not(target_arch = "mips"))]
fn run() -> std::io::Result<()> {
    use display::WindowDisplay;
    use minifb::Key;

    let mut display = WindowDisplay::default_resolution(2)?;
    let mut render = RenderBuffer::default_resolution();
    let mut menu = Menu::new("OpenPager", &["Messages", "Settings", "About", "Exit"]);

    while display.is_open() {
        if let Some(keys) = display.get_keys() {
            for key in keys {
                match key {
                    Key::Up => menu.up(),
                    Key::Down => menu.down(),
                    Key::Enter => {
                        if menu.selected == 3 {
                            return Ok(());
                        }
                    }
                    Key::Escape => return Ok(()),
                    _ => {}
                }
            }
        }

        menu.draw(&mut render).unwrap();
        display.blit(render.pixels_raw(), render.width, render.height);
        display.update()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}

#[cfg(target_arch = "mips")]
fn run() -> std::io::Result<()> {
    use display::Framebuffer;
    use input::{Button, Input};

    let mut fb = Framebuffer::new(None)?;
    let mut render = RenderBuffer::default_resolution();
    let mut input = Input::new(None)?;
    let mut menu = Menu::new("OpenPager", &["Messages", "Settings", "About", "Exit"]);

    loop {
        menu.draw(&mut render).unwrap();
        fb.blit(render.pixels_raw(), render.width, render.height);

        match input.wait()? {
            Button::Up => menu.up(),
            Button::Down => menu.down(),
            Button::Forward => {
                if menu.selected == 3 {
                    return Ok(());
                }
            }
            Button::Back | Button::Power => return Ok(()),
            _ => {}
        }
    }
}
