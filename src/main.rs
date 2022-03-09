#[allow(non_snake_case)]
mod Console;

use std::time::{Duration, Instant};

use windows::Win32::UI::Input::KeyboardAndMouse;
use Console::ConsoleRenderer;

fn main() {
    let mut console = ConsoleRenderer::new();
    console.set_font_size(800, (30, 30));
    console.construct(30, 30, "Hello World - Demo");

    let mut last_frame = Instant::now();
    let mut text_x: f32 = 4.0;
    let mut text_y: f32 = 4.0;

    let mut circle_x = 10.0;
    let mut circle_y = 10.0;

    loop {
        console.clear();

        let now = Instant::now();
        let dt: Duration = now - last_frame;
        let delta_time = dt.as_secs_f32();
        last_frame = Instant::now();

        let dt_string = format!("{:?}", dt.as_secs_f64());

        console.draw_string(1, 1, &dt_string, 0x00f0);
        console.draw_string(
            1,
            2,
            format!("FPS: {}", 1.0 / dt.as_secs_f64()).as_str(),
            0x00f0,
        );

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_LEFT) {
            text_x -= 20.0 * delta_time;
        }
        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_LEFT) {
            text_x -= 20.0 * delta_time;
        }

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_RIGHT) {
            text_x += 20.0 * delta_time;
        }
        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_UP) {
            text_y -= 20.0 * delta_time;
        }

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_DOWN) {
            text_y += 20.0 * delta_time;
        }

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_W) {
            circle_y -= 20.0 * delta_time;
        }

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_S) {
            circle_y += 20.0 * delta_time;
        }

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_A) {
            circle_x -= 20.0 * delta_time;
        }

        if ConsoleRenderer::is_key_down(KeyboardAndMouse::VK_D) {
            circle_x += 20.0 * delta_time;
        }

        let dist = get_dist(
            circle_x as f64,
            circle_y as f64,
            text_x as f64,
            text_y as f64,
        );
        // console.draw_circle_experimental(circle_x.round() as i16, circle_y.round() as i16, dist.round() as i16, ' ', 0x00f0);
        console.draw_circle(
            circle_x.round() as i16,
            circle_y.round() as i16,
            dist.round() as i16,
            ' ',
            0x00f0,
            false,
        );
        console.draw(text_x.round() as i16, text_y.round() as i16, '#', 0x00f0);
        console.draw_string(1, 3, format!("Distance: {}", dist).as_str(), 0x00f0);

        console.blit();
    }
}

fn get_dist(x: f64, y: f64, x2: f64, y2: f64) -> f32 {
    let dx = x2 - x;
    let dy = y2 - y;
    return ((dx * dx + dy * dy) as f32).sqrt();
}
