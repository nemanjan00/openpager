use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, Triangle},
    text::Text,
};

use super::colors::*;
use super::{STATUSBAR_HEIGHT, View, ViewResult};
use crate::platform::{Action, RenderBuffer};

pub struct MenuItem {
    pub label: &'static str,
    pub action: MenuAction,
}

pub enum MenuAction {
    SubMenu(fn() -> Menu),
    Exit,
    None,
}

const ITEM_HEIGHT: i32 = 22;
const TITLE_Y: i32 = STATUSBAR_HEIGHT + 16;
const SEPARATOR_Y: i32 = STATUSBAR_HEIGHT + 20;
const ITEM_START_Y: i32 = STATUSBAR_HEIGHT + 38;

pub struct Menu {
    pub title: &'static str,
    pub items: Vec<MenuItem>,
    pub selected: usize,
    pub scroll: usize,
}

impl Menu {
    pub fn new(title: &'static str, items: Vec<MenuItem>) -> Self {
        Self {
            title,
            items,
            selected: 0,
            scroll: 0,
        }
    }

    fn visible_items(&self, height: u32) -> usize {
        // Round up to use available space at bottom
        let available = height as i32 - ITEM_START_Y;
        ((available + ITEM_HEIGHT - 1) / ITEM_HEIGHT).max(1) as usize
    }
}

impl View for Menu {
    fn render(&self, display: &mut RenderBuffer) {
        let bounds = display.bounding_box();
        let width = bounds.size.width;

        // Clear background (below status bar)
        Rectangle::new(
            Point::new(0, STATUSBAR_HEIGHT),
            Size::new(width, bounds.size.height - STATUSBAR_HEIGHT as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(BACKGROUND))
        .draw(display)
        .unwrap();

        // Draw title
        let title_style = MonoTextStyle::new(&FONT_10X20, PURPLE);
        Text::new(self.title, Point::new(10, TITLE_Y), title_style)
            .draw(display)
            .unwrap();

        // Draw separator line
        Rectangle::new(Point::new(0, SEPARATOR_Y), Size::new(width, 2))
            .into_styled(PrimitiveStyle::with_fill(COMMENT))
            .draw(display)
            .unwrap();

        // Draw menu items (with scrolling)
        let item_style = MonoTextStyle::new(&FONT_10X20, FOREGROUND);
        let selected_style = MonoTextStyle::new(&FONT_10X20, BACKGROUND);
        let visible = self.visible_items(bounds.size.height);

        let has_scroll_up = self.scroll > 0;
        let has_scroll_down = self.scroll + visible < self.items.len();
        let has_arrows = has_scroll_up || has_scroll_down;
        let selection_width = if has_arrows { width - 35 } else { width - 10 };

        for (vi, i) in (self.scroll..(self.scroll + visible).min(self.items.len())).enumerate() {
            let item = &self.items[i];
            let y = ITEM_START_Y + (vi as i32 * ITEM_HEIGHT);

            if i == self.selected {
                Rectangle::new(
                    Point::new(5, y - 15),
                    Size::new(selection_width, ITEM_HEIGHT as u32 - 2),
                )
                .into_styled(PrimitiveStyle::with_fill(CYAN))
                .draw(display)
                .unwrap();
                Text::new(item.label, Point::new(10, y), selected_style)
                    .draw(display)
                    .unwrap();
            } else {
                Text::new(item.label, Point::new(10, y), item_style)
                    .draw(display)
                    .unwrap();
            }
        }

        // Draw scroll indicators if needed (triangles)
        let arrow_x = (width - 18) as i32;
        let arrow_size = 8i32;
        if has_scroll_up {
            let y = ITEM_START_Y - 10;
            Triangle::new(
                Point::new(arrow_x, y + arrow_size),
                Point::new(arrow_x + arrow_size, y + arrow_size),
                Point::new(arrow_x + arrow_size / 2, y),
            )
            .into_styled(PrimitiveStyle::with_fill(FOREGROUND))
            .draw(display)
            .unwrap();
        }
        if has_scroll_down {
            let y = (bounds.size.height as i32) - arrow_size - 4;
            Triangle::new(
                Point::new(arrow_x, y),
                Point::new(arrow_x + arrow_size, y),
                Point::new(arrow_x + arrow_size / 2, y + arrow_size),
            )
            .into_styled(PrimitiveStyle::with_fill(FOREGROUND))
            .draw(display)
            .unwrap();
        }
    }

    fn handle(&mut self, action: Action) -> ViewResult {
        match action {
            Action::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                    if self.selected < self.scroll {
                        self.scroll = self.selected;
                    }
                }
                ViewResult::None
            }
            Action::Down => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                    // We don't know screen height here, use a reasonable default
                    let visible = 5;
                    if self.selected >= self.scroll + visible {
                        self.scroll = self.selected - visible + 1;
                    }
                }
                ViewResult::None
            }
            Action::Select => match &self.items[self.selected].action {
                MenuAction::SubMenu(create_menu) => ViewResult::Push(Box::new(create_menu())),
                MenuAction::Exit => ViewResult::Exit,
                MenuAction::None => ViewResult::None,
            },
            Action::Back => ViewResult::Pop,
        }
    }
}
