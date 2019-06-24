// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

use crate::window::{WindowProperties, WindowTrait};
use crate::events::event::{EventTrait, Event};
// Use
use failure::Error;
use sdl2::video::Window;
use sdl2::Sdl;
use std::option;
use self::sdl2::video::SwapInterval::{VSync, Immediate};
use crate::renderer::render_application::initialise;
use std::slice::Windows;

pub struct WindowsWindow {

    window : Window,
    events : sdl2::EventPump,
    video : sdl2::VideoSubsystem,
    context : sdl2::video::GLContext,
    data : WindowData
}

impl WindowTrait for  WindowsWindow {

    fn on_update(&mut self) {

        unsafe {
            // Test to see if the color changes.
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for event in self.events.poll_iter() {
            match event {
                _ => ()
            }
        }

        self.window.gl_swap_window();
    }

    fn set_event_callback(&self, call_back : fn(&Box<dyn WindowTrait>)) {}

    fn set_vsync(&mut self, enabled : bool) {

        if enabled {
            self.video.gl_set_swap_interval(VSync);
        } else {
            self.video.gl_set_swap_interval(Immediate);
        }

        self.data.vsync = enabled;
    }

    fn is_vsync(&self) -> &bool {&false}

    fn get_native_window(&self) {}

    fn create(properties : WindowProperties, sdl: &Sdl) -> Result<Box<dyn WindowTrait>, Error> where Self : Sized {

        Ok(Box::new(WindowsWindow::new(properties, &sdl)?))
    }
}

impl WindowsWindow {

    fn new(properties : WindowProperties, sdl : &Sdl) -> Result<WindowsWindow, Error> {

        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window = video_subsystem.window( &properties.title, properties.width, properties.height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        // Create gl context AFTER window is created.
        let gl_context = window.gl_create_context().unwrap();

        // Initialise gl.
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as * const std::os::raw::c_void);

        let data = WindowData {
            title : properties.title.clone(),
            width : properties.width.clone(),
            height : properties. height.clone(),
            vsync : true,
            callback : None
        };

        let event_pump = sdl.event_pump().unwrap();

        let window = WindowsWindow {
            window,
            events : event_pump,
            video : video_subsystem,
            context: gl_context,
            data
        };

        Ok(window)

        // NOW we need to set all the callbacks.
    }
}

pub struct WindowData {

    title : String,
    width : u32,
    height : u32,
    pub vsync : bool,
    pub callback : Option<fn(Box<dyn EventTrait>)>
}
