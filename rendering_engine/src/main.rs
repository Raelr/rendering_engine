extern crate sdl2;
extern crate failure;

use failure::Error;

fn main() -> Result<(),Error>{

    // Initialise sdl to allow for window spawning.
    let sdl = sdl2::init().unwrap();

    // Creates the video subsystem which internally contains a clone of sdl.
    let video_subsystem = sdl.video().unwrap();

    // Initialises a new window and allows the input of arguments and parameters into the window.
    let _window = video_subsystem.
        window("Game", 2880, 1800)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    // The main event loop which keeps the window open.
    'main: loop {

        // Looks for events and acts acording to which ones are recieved.
        for event in event_pump.poll_iter() {
            match event {
                // Quit event
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => ()
            }
        }
    }
    Ok(())
}
