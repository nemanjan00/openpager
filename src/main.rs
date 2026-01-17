pub const DISPLAY_WIDTH: u32 = 480;
pub const DISPLAY_HEIGHT: u32 = 222;

mod platform;
mod ui;

use platform::{Action, Platform, RenderBuffer};
use ui::Menu;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> std::io::Result<()> {
    let mut platform = Platform::new()?;
    let mut render = RenderBuffer::default_resolution();
    let mut menu = Menu::new("OpenPager", &["Messages", "Settings", "About", "Exit"]);

    while platform.is_open() {
        if let Some(action) = platform.poll() {
            match action {
                Action::Up => menu.up(),
                Action::Down => menu.down(),
                Action::Select => {
                    if menu.selected == 3 {
                        return Ok(());
                    }
                }
                Action::Back => return Ok(()),
            }
        }

        menu.draw(&mut render).unwrap();
        platform.draw(&render);
        platform.wait();
    }

    Ok(())
}
