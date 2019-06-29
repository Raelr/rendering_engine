// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::Sdl;
use crate::platform::windows::windows_window;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::window::{WindowProperties};
use crate::platform::windows::windows_window::{WindowsWindow, update, process_event};
use std::collections::VecDeque;
use crate::generational_index::generational_index::*;

pub struct GameState {
}

impl GameState {

    pub fn create_initial_state() -> GameState {
        let state = GameState {

        };

        state
    }
}

pub struct ScrapYardApplication {

    pub sdl : Sdl,
    pub game_state : GameState,
    pub update_void_events : Vec<Box<FnMut(&mut GameState)>>,
    pub update_window_events : Vec<Box<FnMut(&mut WindowsWindow)>>,
    pub one_time_events : VecDeque<Box<FnMut()>>,
    // Not sure if this will be working with multiple windows as of now. Will start with a single window.
    pub main_window : WindowsWindow
}

impl ScrapYardApplication {

    pub fn new() -> ScrapYardApplication {

        let sdl = sdl2::init().unwrap();

        let window = windows_window::create_new(window_base!(), &sdl);

        let mut app = ScrapYardApplication {
            sdl,
            game_state: GameState::create_initial_state(),
            update_void_events : Vec::new(),
            update_window_events : Vec::new(),
            one_time_events : VecDeque::new(),
            main_window : window
        };

        app.register_window_update_event(Box::new(windows_window::update));

        app
    }

    pub fn register_game_update_event(&mut self, event : Box<dyn FnMut(&mut GameState)>) {

        &self.update_void_events.push(event);
    }

    pub fn register_window_update_event(&mut self, event : Box<dyn FnMut(&mut WindowsWindow)>) {

        &self.update_window_events.push(event);
    }

    pub fn register_one_time_event(&mut self, event : Box<dyn FnMut()>) {

        &self.one_time_events.push_back(event);
    }
}

pub fn run() -> Result<(),Error>{

    let allocator = GenerationalIndexAllocator::new();

    let mut app = ScrapYardApplication::new();

    let mut pump = app.sdl.event_pump().unwrap();

    loop {

        // Trigger all window events (as passed into )
        for event in app.update_window_events.iter_mut() {
            (event)(&mut app.main_window);
        }

        // Checks for sdl2 events. These are then filtered to appropriate areas to be processed properly.
        for event in  pump.poll_iter() {

            match event {
                // All window events are rerouted toward the active window.
                sdl2::event::Event::Window{timestamp, window_id, win_event}
                => windows_window::process_event(&win_event, &mut app),

                sdl2::event::Event::MouseButtonDown{timestamp, window_id, which, mouse_btn, clicks,x, y}
                => println!("Mouse Clicked at position: {},{}, {}", x, y, window_id),

                sdl2::event::Event::MouseMotion{timestamp, window_id, which,  mousestate, x, y, xrel, yrel}
                => println!("Mouse Moved at position: {},{}", x, y),

                sdl2::event::Event::KeyDown {keycode, repeat, ..}
                => println!("Key pressed: {} repeating: {}", keycode.unwrap(), repeat),

                _ => ()
            }
        }

        while let Some(mut e ) = app.one_time_events.pop_front() {
            e();
        }
    }
    Ok(())
}







