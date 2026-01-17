use embedded_graphics::{
    mono_font::{ascii::FONT_9X15_BOLD, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, Line},
};

use super::colors::*;
use crate::platform::RenderBuffer;

pub const STATUSBAR_HEIGHT: i32 = 24;

pub struct StatusBar {
    pub signal: u8,       // 0-4 bars
    pub connected: bool,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            signal: 4,
            connected: true,
        }
    }
}

impl StatusBar {
    fn read_battery() -> (u8, bool) {
        #[cfg(target_arch = "mips")]
        {
            let mut capacity = 100u8;
            let mut charging = false;
            if let Ok(content) = std::fs::read_to_string("/sys/class/power_supply/bq27546-0/uevent") {
                for line in content.lines() {
                    if let Some(value) = line.strip_prefix("POWER_SUPPLY_CAPACITY=") {
                        capacity = value.parse().unwrap_or(100);
                    } else if let Some(value) = line.strip_prefix("POWER_SUPPLY_STATUS=") {
                        charging = value == "Charging";
                    }
                }
            }
            (capacity, charging)
        }
        #[cfg(not(target_arch = "mips"))]
        {
            (100, false)
        }
    }
}

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

        // Right side icons
        let right_x = width as i32 - 10;

        // Battery icon (rightmost)
        let (battery, charging) = Self::read_battery();
        self.draw_battery(display, right_x - 20, battery, charging);

        // Signal icon
        self.draw_signal(display, right_x - 45);

        // Connection indicator
        if self.connected {
            self.draw_connected(display, right_x - 60);
        }
    }

    fn draw_battery(&self, display: &mut RenderBuffer, x: i32, battery: u8, charging: bool) {
        let color = if battery > 20 { GREEN } else { RED };
        let y_offset = 8;

        // Battery outline
        Rectangle::new(Point::new(x, y_offset), Size::new(18, 10))
            .into_styled(PrimitiveStyle::with_stroke(FOREGROUND, 1))
            .draw(display)
            .unwrap();

        // Battery tip
        Rectangle::new(Point::new(x + 18, y_offset + 3), Size::new(3, 4))
            .into_styled(PrimitiveStyle::with_fill(FOREGROUND))
            .draw(display)
            .unwrap();

        // Battery fill
        let fill_width = (16 * battery as u32 / 100).max(1);
        Rectangle::new(Point::new(x + 1, y_offset + 1), Size::new(fill_width, 8))
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(display)
            .unwrap();

        // Charging bolt (with dark outline for contrast)
        if charging {
            // Dark outline
            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    if dx != 0 || dy != 0 {
                        Line::new(Point::new(x + 10 + dx, y_offset + dy), Point::new(x + 7 + dx, y_offset + 5 + dy))
                            .into_styled(PrimitiveStyle::with_stroke(BACKGROUND, 1))
                            .draw(display)
                            .unwrap();
                        Line::new(Point::new(x + 7 + dx, y_offset + 5 + dy), Point::new(x + 11 + dx, y_offset + 5 + dy))
                            .into_styled(PrimitiveStyle::with_stroke(BACKGROUND, 1))
                            .draw(display)
                            .unwrap();
                        Line::new(Point::new(x + 11 + dx, y_offset + 5 + dy), Point::new(x + 8 + dx, y_offset + 10 + dy))
                            .into_styled(PrimitiveStyle::with_stroke(BACKGROUND, 1))
                            .draw(display)
                            .unwrap();
                    }
                }
            }
            // Yellow bolt
            Line::new(Point::new(x + 10, y_offset), Point::new(x + 7, y_offset + 5))
                .into_styled(PrimitiveStyle::with_stroke(YELLOW, 1))
                .draw(display)
                .unwrap();
            Line::new(Point::new(x + 7, y_offset + 5), Point::new(x + 11, y_offset + 5))
                .into_styled(PrimitiveStyle::with_stroke(YELLOW, 1))
                .draw(display)
                .unwrap();
            Line::new(Point::new(x + 11, y_offset + 5), Point::new(x + 8, y_offset + 10))
                .into_styled(PrimitiveStyle::with_stroke(YELLOW, 1))
                .draw(display)
                .unwrap();
        }
    }

    fn draw_signal(&self, display: &mut RenderBuffer, x: i32) {
        for i in 0..4 {
            let bar_height = 4 + i * 3;
            let y = 20 - bar_height;
            let color = if i < self.signal as i32 { CYAN } else { COMMENT };

            Rectangle::new(
                Point::new(x + i * 4, y),
                Size::new(3, bar_height as u32),
            )
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(display)
            .unwrap();
        }
    }

    fn draw_connected(&self, display: &mut RenderBuffer, x: i32) {
        // Simple wifi-like icon using lines
        for i in 0..3 {
            let y = 16 - i * 4;
            let half_width = 3 + i * 3;
            Line::new(
                Point::new(x + 6 - half_width, y),
                Point::new(x + 6 + half_width, y),
            )
            .into_styled(PrimitiveStyle::with_stroke(PURPLE, 1))
            .draw(display)
            .unwrap();
        }
    }
}
