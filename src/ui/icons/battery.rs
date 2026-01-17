use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
};

use crate::platform::RenderBuffer;
use crate::ui::colors::*;

const WIDTH: i32 = 21; // 18 body + 3 tip
const SEGMENT_COUNT: u8 = 3;

/// Draws battery icon ending at cursor position, returns width
pub fn draw(display: &mut RenderBuffer, cursor: i32, battery: u8, charging: bool) -> i32 {
    let x = cursor - WIDTH;
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

    // Calculate filled segments (0-3)
    let filled = if battery == 100 {
        SEGMENT_COUNT
    } else {
        (battery * SEGMENT_COUNT / 100).min(SEGMENT_COUNT)
    };

    // Draw 3 segments
    let segment_width = 4u32;
    let segment_gap = 1;
    for i in 0..SEGMENT_COUNT {
        let seg_x = x + 2 + (i as i32 * (segment_width as i32 + segment_gap));
        let color = if i < filled {
            if battery <= 20 {
                RED
            } else if battery == 100 {
                CYAN
            } else {
                GREEN
            }
        } else {
            BACKGROUND
        };

        Rectangle::new(Point::new(seg_x, y_offset + 2), Size::new(segment_width, 6))
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(display)
            .unwrap();
    }

    // Charging bolt (with dark outline for contrast)
    if charging {
        draw_bolt(display, x, y_offset);
    }

    WIDTH
}

fn draw_bolt(display: &mut RenderBuffer, x: i32, y_offset: i32) {
    // Dark outline
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx != 0 || dy != 0 {
                Line::new(
                    Point::new(x + 10 + dx, y_offset + dy),
                    Point::new(x + 7 + dx, y_offset + 5 + dy),
                )
                .into_styled(PrimitiveStyle::with_stroke(BACKGROUND, 1))
                .draw(display)
                .unwrap();
                Line::new(
                    Point::new(x + 7 + dx, y_offset + 5 + dy),
                    Point::new(x + 11 + dx, y_offset + 5 + dy),
                )
                .into_styled(PrimitiveStyle::with_stroke(BACKGROUND, 1))
                .draw(display)
                .unwrap();
                Line::new(
                    Point::new(x + 11 + dx, y_offset + 5 + dy),
                    Point::new(x + 8 + dx, y_offset + 10 + dy),
                )
                .into_styled(PrimitiveStyle::with_stroke(BACKGROUND, 1))
                .draw(display)
                .unwrap();
            }
        }
    }
    // Yellow bolt
    Line::new(
        Point::new(x + 10, y_offset),
        Point::new(x + 7, y_offset + 5),
    )
    .into_styled(PrimitiveStyle::with_stroke(YELLOW, 1))
    .draw(display)
    .unwrap();
    Line::new(
        Point::new(x + 7, y_offset + 5),
        Point::new(x + 11, y_offset + 5),
    )
    .into_styled(PrimitiveStyle::with_stroke(YELLOW, 1))
    .draw(display)
    .unwrap();
    Line::new(
        Point::new(x + 11, y_offset + 5),
        Point::new(x + 8, y_offset + 10),
    )
    .into_styled(PrimitiveStyle::with_stroke(YELLOW, 1))
    .draw(display)
    .unwrap();
}
