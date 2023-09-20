use super::*;

pub struct Drawer<'a> {
    rect: RectangleShape<'a>,
}

impl<'a> Drawer<'a> {
    pub fn new() -> Self {
        Self {
            rect: RectangleShape::new(),
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow, data: &info::Data) {
        for i in 0..NUM_BARS {
            self.rect
                .set_position((BAR_WIDTH * i as f32, WINDOW_HEIGHT as f32));
            self.rect
                .set_size((BAR_WIDTH, data.get(i as _).unwrap()));
            self.rect.set_origin((0.0, self.rect.size().y));
            self.rect.set_fill_color(if i % 2 == 0 {
                Color::WHITE
            } else {
                Color::rgb(100, 100, 100)
            });

            window.draw(&self.rect);
        }
    }
}
