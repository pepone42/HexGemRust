extern crate rand;
extern crate sdl2;

use board::*;
use system::System;

mod color;
mod hexagrid;
mod gem;
mod board;
mod actor;
mod system;


fn mainloop(sys : &mut System) {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;    
    use sdl2::pixels::Color;
    
    let mut events = sys.sdl_context.event_pump().unwrap();

    // loop until we receive a QuitEvent
    'event: loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event,
                Event::KeyDown { keycode, .. } => {
                    print!("KeyDown Event {:?}\n", keycode);
                    match keycode {
                        Some(Keycode::Escape) => break 'event,
                        _ => break, // unmapped
                    }
                }
                _ => continue,
            }
        }
        // Render a fully black window
        sys.renderer.set_draw_color(Color::RGB(0, 0, 0));
        sys.renderer.clear();
        sys.renderer.present();
    }
}


fn main() {
    let b = Board::new(5, 7);
    let a = b.grid.get(0);
    println!("[{}]", a.get_color());
    b.randomize(true);
    println!("[{}]", a.get_color());
    
    let mut sys =&mut System::new("Hexgem",400,240);
    mainloop(sys);
}
