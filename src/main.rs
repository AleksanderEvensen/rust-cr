mod Console;
use Console::ConsoleRenderer;

use std::time:: { Duration, Instant };
use std::thread;

fn main() {

    let mut console = ConsoleRenderer::new("Hello World - Game", 80,25, 1,1);
    let is_running = true;

    let mut x = 0;


    while is_running {
        console.clear();

        console.draw(x, 9, 'X', 0x00f0);

        x+=1;

        console.fill(1,1,5,5,'#', 0x00f0);

        console.blit();
        
        thread::sleep(Duration::from_millis(200));
    }
    
    
    // console.draw(10,10,'#', 0x00F0);

    // console.blit();

    // thread::sleep(std::time::Duration::from_secs(5));
}
