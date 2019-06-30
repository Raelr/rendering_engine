// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

use crate::window::{WindowProperties, WindowTrait};
// Use
use sdl2::video::Window;
use sdl2::Sdl;
use self::sdl2::video::SwapInterval::{VSync, Immediate};
use crate::application::{ScrapYardApplication};
use std::process;
use std::collections::VecDeque;

pub struct WindowsWindow {

    window : Window,
    video : sdl2::VideoSubsystem,
    context : sdl2::video::GLContext,
    pub data : WindowData
}

pub fn create_new(properties : WindowProperties, sdl: &Sdl) -> WindowsWindow  {

    WindowsWindow::new(properties, &sdl)
}

impl WindowTrait for  WindowsWindow {

    fn on_update(&mut self) {

        unsafe {
            // Test to see if the color changes.
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.window.gl_swap_window();
    }

    fn set_vsync(&mut self, enabled : bool) {

        if enabled {
            self.video.gl_set_swap_interval(VSync).unwrap();
        } else {
            self.video.gl_set_swap_interval(Immediate).unwrap();
        }

        self.data.vsync = enabled;
    }

    fn is_vsync(&self) -> &bool {
        &self.data.vsync
    }

    fn get_native_window(&self) {}

    fn get_data(&mut self) -> &mut WindowData {
        &mut self.data
    }
}

impl WindowsWindow {

    fn new(properties : WindowProperties, sdl : &Sdl) -> WindowsWindow {

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
        };

        let mut window = WindowsWindow {
            window,
            video : video_subsystem,
            context: gl_context,
            data
        };

        window.set_vsync(true);

        window

        // NOW we need to set all the callbacks.
    }
}

pub struct WindowData {

    title : String,
    width : u32,
    height : u32,
    pub vsync : bool,
}

pub fn process_event(window_event : &sdl2::event::WindowEvent, event_queue : &mut VecDeque<Box<FnMut()>>) {

    match window_event {
        // When the window is closed, the application should close down.
        sdl2::event::WindowEvent::Close => { event_queue.push_back(Box::new(on_window_close))},
        sdl2::event::WindowEvent::Resized(x, y) => println!("{} {} {}", "Window resized:", x, y),
        sdl2::event::WindowEvent::Minimized => println!("{}", "minimized"),
        sdl2::event::WindowEvent::Exposed => println!("{}", "exposed"),
        sdl2::event::WindowEvent::FocusGained => println!("{}", "focus gained"),
        sdl2::event::WindowEvent::Enter => println!("{}", "Mouse entered"),
        sdl2::event::WindowEvent::TakeFocus => println!("{}", "Taking focus"),
        _ => ()
    }
}

#[inline] pub fn on_window_close () {

    println!("Window: closed. Exiting Scrapyard.");
    process::exit(1)
}