use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

pub struct Menu<'a> {
    pub title: &'a str,
    pub items: &'a [&'a str],
    pub selected: usize,
}

impl<'a> Menu<'a> {
    pub fn new(title: &'a str, items: &'a [&'a str]) -> Self {
        Self {
            title,
            items,
            selected: 0,
        }
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let white = Rgb565::WHITE;
        let black = Rgb565::BLACK;
        let highlight = Rgb565::new(0, 0, 31); // Blue

        let bounds = display.bounding_box();
        let width = bounds.size.width;

        // Clear background
        Rectangle::new(Point::zero(), bounds.size)
            .into_styled(PrimitiveStyle::with_fill(black))
            .draw(display)?;

        // Draw title
        let title_style = MonoTextStyle::new(&FONT_6X10, white);
        Text::new(self.title, Point::new(10, 15), title_style).draw(display)?;

        // Draw separator line
        Rectangle::new(Point::new(0, 22), Size::new(width, 1))
            .into_styled(PrimitiveStyle::with_fill(white))
            .draw(display)?;

        // Draw menu items
        let item_style = MonoTextStyle::new(&FONT_6X10, white);
        let selected_style = MonoTextStyle::new(&FONT_6X10, black);

        for (i, item) in self.items.iter().enumerate() {
            let y = 35 + (i as i32 * 16);

            if i == self.selected {
                // Highlight selected item
                Rectangle::new(Point::new(5, y - 10), Size::new(width - 10, 14))
                    .into_styled(PrimitiveStyle::with_fill(highlight))
                    .draw(display)?;
                Text::new(item, Point::new(10, y), selected_style).draw(display)?;
            } else {
                Text::new(item, Point::new(10, y), item_style).draw(display)?;
            }
        }

        Ok(())
    }
}
