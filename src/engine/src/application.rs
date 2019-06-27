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

static mut SDL_INITIALISED : bool = false;

pub struct Application {

    window : Box<WindowTrait>,
    sdl : Sdl,
    running : bool
}

impl Application {

    pub fn new() -> Result<Application, Error> {

        let sdl = unsafe { check_for_sdl() }.unwrap();

        let props : WindowProperties = window_base!();
        let mut window = WindowsWindow::create(props, &sdl)?;

        let mut application = Application {
            window,
            sdl,
            running : true
        };

        Ok(application)
    }
}

pub fn run() -> Result<(),Error>{

    let mut app = Application::new()?;

    let mut running = true;

    let mut window = app.window;

    let mut window_data = window.get_data();

    while running {

        window.on_update();

    }

    Ok(())
}

unsafe fn check_for_sdl() -> Option<Sdl> {

    if !SDL_INITIALISED {
        let success = sdl2::init().unwrap();
        SDL_INITIALISED = true;
        return Some(success)
    }

    None
}





