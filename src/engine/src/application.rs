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
use crate::platform::windows::windows_window::WindowsWindow;
use crate::events::EventType::WindowClose;

pub struct GameState {

    pub windows_windows : Vec<Option<WindowsWindow>>,
}

impl GameState {

    pub fn create_initial_state() -> GameState {
        let state = GameState {
            windows_windows : Vec::new()
        };

        state
    }
}



pub fn run() -> Result<(),Error>{

    let sdl = sdl2::init().unwrap();

    let mut game_state = GameState::create_initial_state();

    let mut window = windows_window::create_new(window_base!(), &sdl);

    game_state.windows_windows.push(Some(window));

    let mut running = true;

    while running {

        windows_window::update(&mut game_state);
    }


    Ok(())
}





