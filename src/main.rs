pub const DISPLAY_WIDTH: u32 = 480;
pub const DISPLAY_HEIGHT: u32 = 222;

mod platform;
mod ui;

use platform::{Platform, RenderBuffer};
use ui::{Menu, MenuAction, MenuItem, StatusBar, View, ViewResult};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn create_main_menu() -> Menu {
    Menu::new(
        "Main Menu",
        vec![
            MenuItem {
                label: "Messages",
                action: MenuAction::SubMenu(create_messages_menu),
            },
            MenuItem {
                label: "Settings",
                action: MenuAction::SubMenu(create_settings_menu),
            },
            MenuItem {
                label: "About",
                action: MenuAction::SubMenu(create_about_menu),
            },
            MenuItem {
                label: "Exit",
                action: MenuAction::Exit,
            },
        ],
    )
}

fn create_messages_menu() -> Menu {
    Menu::new(
        "Messages",
        vec![
            MenuItem {
                label: "Inbox",
                action: MenuAction::SubMenu(create_inbox),
            },
            MenuItem {
                label: "Sent",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Drafts",
                action: MenuAction::None,
            },
        ],
    )
}

fn create_inbox() -> Menu {
    Menu::new(
        "Inbox",
        vec![
            MenuItem {
                label: "Server alert: CPU 95%",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Backup completed",
                action: MenuAction::None,
            },
            MenuItem {
                label: "New user registered",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Payment received",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Disk space warning",
                action: MenuAction::None,
            },
            MenuItem {
                label: "SSL cert expiring",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Deploy successful",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Error: DB timeout",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Weekly report ready",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Security scan done",
                action: MenuAction::None,
            },
            MenuItem {
                label: "New comment on #42",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Build failed: main",
                action: MenuAction::None,
            },
        ],
    )
}

fn create_settings_menu() -> Menu {
    Menu::new(
        "Settings",
        vec![
            MenuItem {
                label: "Display",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Sound",
                action: MenuAction::None,
            },
            MenuItem {
                label: "Network",
                action: MenuAction::None,
            },
        ],
    )
}

fn create_about_menu() -> Menu {
    Menu::new(
        "About",
        vec![
            MenuItem {
                label: "Version: 0.1.0",
                action: MenuAction::None,
            },
            MenuItem {
                label: "License: MIT",
                action: MenuAction::None,
            },
        ],
    )
}

fn run() -> std::io::Result<()> {
    let mut platform = Platform::new()?;
    let mut render = RenderBuffer::default_resolution();
    let mut views: Vec<Box<dyn View>> = vec![Box::new(create_main_menu())];
    let status_bar = StatusBar::default();

    while platform.is_open() && !views.is_empty() {
        if let Some(action) = platform.poll() {
            let result = views.last_mut().unwrap().handle(action);
            match result {
                ViewResult::None => {}
                ViewResult::Push(view) => views.push(view),
                ViewResult::Pop => {
                    views.pop();
                }
                ViewResult::Exit => break,
            }
        }

        status_bar.render(&mut render);
        if let Some(view) = views.last() {
            view.render(&mut render);
        }
        platform.draw(&render);
        platform.wait();
    }

    Ok(())
}
