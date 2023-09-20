use sfml::{graphics::*, system::*, window::*};

mod draw;
mod info;

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 900;
pub const NUM_BARS: u32 = 1000;
pub const TIME_BETWEEN_STEPS: Time = Time::milliseconds(30); // time between each sort step
pub const BAR_WIDTH: f32 = WINDOW_WIDTH as f32 / NUM_BARS as f32;

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Sorting algorithm visualizer",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_vertical_sync_enabled(true);

    let mut drawer = draw::Drawer::new();
    let mut data = info::Data::with_size(NUM_BARS as _, true); // true for ascending, false for descending
    data.generate_random(1.0, WINDOW_HEIGHT as f32 * 0.7);

    let mut elapsed_time = Time::ZERO;
    let mut last_step_time = Time::ZERO;
    let mut dtc = Clock::start();
    while window.is_open() {
        let dt = dtc.restart();
        elapsed_time += dt;

        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            } else if let Event::KeyPressed { code, .. } = event {
                if code == Key::Space {
                    data.sort_step();
                }
            }
        }

        if !data.is_sorted() && elapsed_time - last_step_time > TIME_BETWEEN_STEPS {
            last_step_time = elapsed_time;
            data.sort_step();
        }

        window.clear(Color::BLACK);
        drawer.draw(&mut window, &data);
        window.display();
    }
}
