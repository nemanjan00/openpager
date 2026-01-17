use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

use super::colors::*;
use super::{View, ViewResult, STATUSBAR_HEIGHT};
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

const ITEM_HEIGHT: i32 = 28;
const TITLE_Y: i32 = STATUSBAR_HEIGHT + 20;
const SEPARATOR_Y: i32 = STATUSBAR_HEIGHT + 28;
const ITEM_START_Y: i32 = STATUSBAR_HEIGHT + 55;

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
        ((height as i32 - ITEM_START_Y) / ITEM_HEIGHT).max(1) as usize
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

        for (vi, i) in (self.scroll..(self.scroll + visible).min(self.items.len())).enumerate() {
            let item = &self.items[i];
            let y = ITEM_START_Y + (vi as i32 * ITEM_HEIGHT);

            if i == self.selected {
                Rectangle::new(Point::new(5, y - 18), Size::new(width - 30, 24))
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

        // Draw scroll indicators if needed
        if self.scroll > 0 {
            Text::new("^", Point::new((width - 10) as i32, ITEM_START_Y), item_style)
                .draw(display)
                .unwrap();
        }
        if self.scroll + visible < self.items.len() {
            let y = ITEM_START_Y + ((visible - 1) as i32 * ITEM_HEIGHT);
            Text::new("v", Point::new((width - 10) as i32, y), item_style)
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
