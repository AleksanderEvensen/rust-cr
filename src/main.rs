#[allow(non_snake_case)]
mod Console;

use std::{convert::TryInto, time::{Duration, Instant}};

use Console::ConsoleRenderer;

fn main() {

    let mut console = ConsoleRenderer::new("Hello World - Game", 100,100, 10,10);
    let is_running = true;

    let mut x = 0.0;
    let mut text_x = 0.0;
    let mut text_y = 0.0;
    let mut last_frame = Instant::now();

    while is_running {
        console.clear();
        
        let now = Instant::now();
        let dt: Duration = now - last_frame; 
        let delta_time = dt.as_secs_f32();       
        last_frame = Instant::now();

        let dt_string = format!("{:?}", dt.as_secs_f64());

        console.draw_string(1, 1, &dt_string, 0x00f0);

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_SPACE) {
            x+=0.1 * delta_time;
        }

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_LEFT) {
            text_x -= 100.0*delta_time;
        }

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_RIGHT) {
            text_x += 100.0*delta_time;
        }
        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_UP) {
            text_y -= 100.0*delta_time;
        }

        if ConsoleRenderer::is_key_down(winapi::um::winuser::VK_DOWN) {
            text_y += 100.0*delta_time;
        }
        
        console.draw_string(text_x.round() as i16, text_y.round() as i16, "Hello World", 0x00f0);
        let (x,y) = get_point_from_angle(x as f32, 10.0);
        console.draw(50+x.round() as i16, 20+y.round() as i16, '*', 0x00f0);

        
        console.blit();
        
    }
}

fn get_point_from_angle(angle:f32, radius:f32) -> (f32, f32) {
    let x = (angle * radius).sin() * radius;
    let y = (angle * radius).cos() * radius;
    (x,y)
}