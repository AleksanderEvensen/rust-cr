#[allow(non_snake_case)]
mod Console;

use std::time::{Duration, Instant};

use Console::ConsoleRenderer;

fn main() {

    let mut console = ConsoleRenderer::new();
    console.set_font_size(800, (30,30));
    console.construct(30, 30, "Hello World - Demo");

    let mut last_frame = Instant::now();
    let mut text_x = 0.0;
    let mut text_y = 0.0;

    loop {
        console.clear();

        let now = Instant::now();
        let dt: Duration = now - last_frame; 
        let delta_time = dt.as_secs_f32();       
        last_frame = Instant::now();

        let dt_string = format!("{:?}", dt.as_secs_f64());

        console.draw_string(1, 1, &dt_string, 0x00f0);
        console.draw_string(1, 2, format!("FPS: {}", 1.0/dt.as_secs_f64()).as_str(), 0x00f0);

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_LEFT) {
            text_x -= 20.0*delta_time;
        }

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_RIGHT) {
            text_x += 20.0*delta_time;
        }
        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_UP) {
            text_y -= 20.0*delta_time;
        }

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_DOWN) {
            text_y += 20.0*delta_time;
        }


        console.draw(text_x.round() as i16,text_y.round() as i16, ' ', 0x00f0);



        console.blit();
    }
}