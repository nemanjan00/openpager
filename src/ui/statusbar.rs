use embedded_graphics::{
    mono_font::{ascii::FONT_9X15_BOLD, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use super::colors::*;
use super::icons;
use crate::platform::{read_battery, RenderBuffer};

pub const STATUSBAR_HEIGHT: i32 = 24;

pub struct StatusBar;

impl Default for StatusBar {
    fn default() -> Self {
        Self
    }
}

impl StatusBar {
    fn read_time() -> (u8, u8) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        // Convert to hours:minutes (UTC, add timezone offset if needed)
        let secs_today = secs % 86400;
        let hours = ((secs_today / 3600) as u8) % 24;
        let minutes = ((secs_today % 3600) / 60) as u8;
        (hours, minutes)
    }
}

const ICON_SPACING: i32 = 6;

impl StatusBar {
    pub fn render(&self, display: &mut RenderBuffer) {
        let bounds = display.bounding_box();
        let width = bounds.size.width;

        // Status bar background
        Rectangle::new(Point::zero(), Size::new(width, STATUSBAR_HEIGHT as u32))
            .into_styled(PrimitiveStyle::with_fill(SELECTION))
            .draw(display)
            .unwrap();

        let text_style = MonoTextStyle::new(&FONT_9X15_BOLD, FOREGROUND);

        // Left: Title
        embedded_graphics::text::Text::new("OpenPager", Point::new(6, 18), text_style)
            .draw(display)
            .unwrap();

        // Right side: draw icons from right to left, tracking cursor
        let mut cursor = width as i32 - 6;

        // Battery icon
        let (battery, charging) = read_battery();
        let battery_width = icons::battery::draw(display, cursor, battery, charging);
        cursor -= battery_width + ICON_SPACING;

        // Clock (before icons) - 5 chars * 9px = 45px wide
        let (hours, minutes) = Self::read_time();
        let time_str = format!("{:02}:{:02}", hours, minutes);
        let clock_width = 45;
        embedded_graphics::text::Text::new(
            &time_str,
            Point::new(cursor - clock_width, 18),
            text_style,
        )
        .draw(display)
        .unwrap();
    }
}
