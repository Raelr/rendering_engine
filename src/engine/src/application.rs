// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::video::Window;
use sdl2::Sdl;
use crate::platform::windows::windows_window;
use crate::renderer::render_application::RendererInformation;
use crate::renderer::render_application;
use crate::renderer::renderer_tests;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::events::mouse_changed::MouseChangedEvent;
use crate::events::event::EventTrait;
use crate::events::mouse_button_event::MouseButtonEvent;
use crate::events::application_events::{WindowResizeEvent, BaseWindowEvent};
use crate::window::{WindowTrait, WindowProperties};
use crate::platform::windows::windows_window::WindowsWindow;

static mut SDL_INITIALISED : bool = false;

pub struct Application {

    window : Box<dyn WindowTrait>,
    sdl : Sdl,
    running : bool
}

impl Application {

    pub fn new() -> Result<Application, Error> {

        let sdl = unsafe { check_for_sdl() }.unwrap();

        let props : WindowProperties = window_base!();
        let window = WindowsWindow::create(props, &sdl)?;

        let application = Application {
            window,
            sdl,
            running: true
        };

        Ok(application)
    }

    pub fn run(&mut self) {

        while self.running {
            self.window.on_update();
        }
    }
}


unsafe fn check_for_sdl() -> Option<Sdl> {

    if !SDL_INITIALISED {
        let success = sdl2::init().unwrap();
        SDL_INITIALISED = true;
        return Some(success)
    }

    None
}

