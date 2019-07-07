// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;
// Use
use failure::Error;
use sdl2::Sdl;
use crate::platform::windows::windows_window;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::window::{WindowProperties, WindowTrait};
use crate::platform::windows::windows_window::{WindowsWindow, process_event};
use std::collections::VecDeque;
use crate::generational_index::generational_index::*;
use std::time::Duration;
use crate::events::window_event::WindowEvent;

/// GameState object stores all entities and components within itself. If handles the streaming of
/// components into different systems.

pub struct GameState {

}

///

impl GameState {

    pub fn create_initial_state() -> GameState {
        let state = GameState {

        };

        state
    }
}

///

pub struct ScrapYardApplication {

    pub game_state : GameState,
    pub update_void_events : Vec<Box<FnMut(&mut GameState)>>,
}

///

impl ScrapYardApplication {

    pub fn new() -> ScrapYardApplication {

        let mut app = ScrapYardApplication {
            game_state: GameState::create_initial_state(),
            update_void_events : Vec::new(),
        };

        app
    }

    pub fn register_game_update_event(&mut self, event : Box<dyn FnMut(&mut GameState)>) {

        &self.update_void_events.push(event);
    }
}

/// This is the code for the current event loop.
/// The event loop controls the basic data flow of the engine.
/// Currently, it contains the window, a reference to the main application struct, and all the SDL details.
/// There are a couple of details which i'm not sure about - specifically relating to how the data should be organised.
/// Mainly, I'm unsure whether the window should handle all sdl related events or just events relating to it.
/// Currently I have the event pump in the main loop, the match statement would, in theory, redirect the events toward the
/// correct module.

pub fn run() -> Result<(),Error>{

    // Initialise sdl
    let sdl = sdl2::init().unwrap();

    // Create the base window for the application.
    let mut window = windows_window::create_new(window_base!(), &sdl);

    // Create the base application.
    let mut app = ScrapYardApplication::new();

    // Get the event pump from sdl.
    let mut pump = sdl.event_pump().unwrap();

    // Initialise the one time event queue.
    let mut one_time_events : VecDeque<Box<dyn FnMut()>> = VecDeque::new();

    let mut one_time_window_events : VecDeque<Box<dyn FnMut(&mut WindowsWindow)>> = VecDeque::new();

    loop {

        // Update our window.
        window.on_update();

        // Checks for sdl2 events. These are then filtered to appropriate areas to be processed properly.
        for event in  pump.poll_iter() {

            match event {
                // All window events are rerouted toward the active window.
                sdl2::event::Event::Window{timestamp, window_id, win_event}
                => windows_window::process_event(&win_event, &mut WindowEvent { window : &mut window, events: &mut one_time_window_events}),

                //
                sdl2::event::Event::MouseButtonDown{timestamp, window_id, which, mouse_btn, clicks,x, y}
                => println!("MAIN LOOP: Mouse Clicked: {},{}, {}", x, y, window_id),

                //
                sdl2::event::Event::MouseMotion{timestamp, window_id, which,  mousestate, x, y, xrel, yrel}
                => println!("MAIN LOOP: Mouse Moved: {},{}", x, y),

                //
                sdl2::event::Event::KeyDown { keycode, repeat, .. }
                => println!("MAIN LOOP: Key pressed: {} repeating: {}", keycode.unwrap(), repeat),

                //
                _ => ()
            }
        }
        // Cycles through all events stored in this queue and executes them.
        while let Some(mut e ) = one_time_events.pop_front() {
            e();
        }

        while let Some(mut e ) = one_time_window_events.pop_front() {
            e(&mut window);
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


    Ok(())
}









