use egui_sfml::{
    egui::{self, FontId, RichText},
    SfEgui,
};
use sfml::{graphics::*, system::*, window::*};

mod draw;
mod info;

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 900;
pub const NUM_BARS: u32 = 100;
pub const TIME_BETWEEN_STEPS: Time = Time::milliseconds(100); // time between each sort step

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Sorting algorithm visualizer",
        Style::CLOSE,
        &Default::default(),
    );

    let mut sfegui = SfEgui::new(&window);

    window.set_vertical_sync_enabled(true);

    let mut ascending = true;
    let mut drawer = draw::Drawer::new();
    let mut data = info::Data::with_size(NUM_BARS as _, ascending);
    data.generate_random(1.0, WINDOW_HEIGHT as f32 * 0.7);

    let mut elapsed_time = Time::ZERO;
    let mut last_step_time = Time::ZERO;
    let mut dtc = Clock::start();

    let mut bars_str = NUM_BARS.to_string();
    let mut bars = NUM_BARS;
    let mut time_bw_steps_str = TIME_BETWEEN_STEPS.as_milliseconds().to_string();
    let mut time_bw_steps = TIME_BETWEEN_STEPS.as_milliseconds();

    while window.is_open() {
        let dt = dtc.restart();
        elapsed_time += dt;

        while let Some(event) = window.poll_event() {
            sfegui.add_event(&event);
            if let Event::Closed = event {
                window.close();
            } else if let Event::KeyPressed { code, .. } = event {
                if code == Key::Space {
                    data.sort_step();
                }
            }
        }

        data.change_order(ascending);
        sfegui
            .do_frame(|ctx| {
                let win = egui::Window::new("Settings")
                    .fixed_pos((0.0, 0.0))
                    .auto_sized()
                    .collapsible(false);

                win.show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("No. of bars").font(FontId::proportional(14.0)));
                        ui.text_edit_singleline(&mut bars_str);
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Time b/w steps").font(FontId::proportional(14.0)));
                        ui.text_edit_singleline(&mut time_bw_steps_str);
                        ui.label(RichText::new("ms").font(FontId::proportional(14.0)));
                    });

                    ui.checkbox(
                        &mut ascending,
                        RichText::new("ascending").font(FontId::proportional(14.0)),
                    );

                    if ui.button(RichText::new("update").font(FontId::proportional(14.0))).clicked() {
                        if let Ok(x) = bars_str.parse() {
                            bars = x;
                            data.resize(bars as _);
                        }

                        if let Ok(x) = time_bw_steps_str.parse() {
                            time_bw_steps = x;
                        }

                        data.generate_random(1.0, WINDOW_HEIGHT as f32 * 0.7);
                        data.reset();
                    }
                });
            })
            .unwrap();

        if !data.is_sorted() && (elapsed_time - last_step_time).as_milliseconds() > time_bw_steps {
            last_step_time = elapsed_time;
            data.sort_step();
        }

        window.clear(Color::BLACK);
        drawer.draw(&mut window, &data, WINDOW_WIDTH as f32 / bars as f32);
        sfegui.draw(&mut window, None);
        window.display();
    }
}
