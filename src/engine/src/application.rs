// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::Sdl;
use crate::platform::windows::windows_window;
use crate::renderer::render_application;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::events::event::{EventTrait, EventDispatcher, Event};
use crate::events::application_events::{WindowResizeEvent, BaseWindowEvent};
use crate::window::{WindowTrait, WindowProperties};
use crate::platform::windows::windows_window::{WindowsWindow, update};
use crate::events::EventType::WindowClose;
use std::collections::VecDeque;
use self::sdl2::EventPump;
use crate::generational_index::generational_index::*;
use self::sdl2::event::WindowEvent;
use self::sdl2::event::EventType::Window;

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
    pub one_time_events : VecDeque<Box<FnMut(&mut GameState)>>,
    pub event_pump : sdl2::EventPump,
    // Not sure if this will be working with multiple windows as of now. Will start with a single window.
    pub main_window : WindowsWindow
}

impl ScrapYardApplication {

    pub fn new() -> ScrapYardApplication {

        let sdl = sdl2::init().unwrap();

        let pump = sdl.event_pump().unwrap();

        let window = windows_window::create_new(window_base!(), &sdl);

        let mut app = ScrapYardApplication {
            sdl,
            game_state: GameState::create_initial_state(),
            update_void_events : Vec::new(),
            update_window_events : Vec::new(),
            one_time_events : VecDeque::new(),
            event_pump : pump,
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
}

pub fn run() -> Result<(),Error>{

    let allocator = GenerationalIndexAllocator::new();

    let mut app = ScrapYardApplication::new();

    let mut running = true;

    let mut window_events = VecDeque::new();

    while running {

        // Trigger all events which need to be constantly updated.
        for event in app.update_void_events.iter_mut() {
            (event)(&mut app.game_state);
        }

        for event in app.update_window_events.iter_mut() {
            (event)(&mut app.main_window);
        }

        for event in  app.event_pump.poll_iter() {
            match event {

                sdl2::event::Event::Window{timestamp, window_id, win_event} => window_events.push_back(win_event),
                sdl2::event::Event::MouseButtonDown{timestamp, window_id, which, mouse_btn, clicks,x, y}
                => println!("Mouse Clicked at position: {},{}, {}", x, y, window_id),
                sdl2::event::Event::MouseMotion{timestamp, window_id, which,  mousestate, x, y, xrel, yrel}
                => println!("Mouse Moved at position: {},{}", x, y),
                sdl2::event::Event::KeyDown {timestamp, window_id, keycode, scancode, keymod, repeat}
                => println!("Key pressed: {} repeating: {}", keycode.unwrap(), repeat),
                _ => ()
            }
        }

        while !window_events.is_empty() {

            let event = window_events.pop_front();

            match event {
                Some(T) => windows_window::process_event(&T, &mut app),
                None => continue
            }
        }
    }
    Ok(())
}







