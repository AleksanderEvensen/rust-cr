#[allow(non_snake_case)]
mod Console;

use Console::ConsoleRenderer;

fn main() {

    let mut console = ConsoleRenderer::new("Hello World - Game", 100,100, 10,10);
    let is_running = true;

    let mut x = 0.0;


    while is_running {
        console.clear();


        x+=0.001;

        let (x,y) = get_point_from_angle(x, 10.0);

        console.draw(50+x.round() as i16, 50+y.round() as i16, '*', 0x00f0);

        console.fill(0,0,5,25,'#', 0x00f0 | 0x0001);

        console.blit();
        
    }
}

fn get_point_from_angle(angle:f32, radius:f32) -> (f32, f32) {
    let x = (angle * radius).sin() * radius;
    let y = (angle * radius).cos() * radius;
    (x,y)
}
