mod Console;
use Console::ConsoleRenderer;

let mut render;

pub fn run() {
    render = ConsoleRenderer::new(100, 100); 
}