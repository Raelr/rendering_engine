extern crate sdl2;
extern crate gl;
extern crate failure;

use failure::Error;
use gl::*;

fn main() -> Result<(),Error>{

    // Initialise sdl to allow for window spawning.
    let sdl = sdl2::init().unwrap();

    // Creates the video subsystem which internally contains a clone of sdl.
    let video_subsystem = sdl.video().unwrap();

    // Initialises a new window and allows the input of arguments and parameters into the window.
    let window = video_subsystem.
        window("Game", 2880, 1800)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create gl context AFTER window is created.
    let gl_context = window.gl_create_context().unwrap();

    // Initialise gl.
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as * const std::os::raw::c_void);

    // Change color of the window. Calls an unsafe function from gl library.
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // Event pump which stores all events and allows them to be processed.
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

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Updates the window.
            window.gl_swap_window();
        }
    }
    Ok(())
}
