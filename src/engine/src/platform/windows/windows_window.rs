// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use crate
use crate::window::{WindowProperties, WindowTrait};
use crate::events::window_event;
use crate::application::{ScrapYardApplication};
use crate::events::window_event::WindowEvent;
use crate::platform::open_gl::OpenGLContext;

// Use
use sdl2::video::Window;
use sdl2::Sdl;
use self::sdl2::video::SwapInterval::{VSync, Immediate};
use std::process;
use std::collections::VecDeque;

/// Indended to be a window class for a windows implementation. A bit irrelevant at the moment since the window work on mac, but
/// in future, if the engine is going to be used with the WinAPI, then it'll be more customised.

pub struct WindowsWindow {

    window : Window,
    video : sdl2::VideoSubsystem,
    context : OpenGLContext,
    pub data : WindowData
}

/// static function for creating a base window (doesnt require specific elements to be inputted by user)

pub fn create_new(properties : WindowProperties, sdl: &Sdl) -> WindowsWindow  {

    WindowsWindow::new(properties, &sdl)
}

// TODO: Work out if this trait system is even needed at this point. Might just remove and replace with a base window struct.

impl WindowTrait for  WindowsWindow {

    fn on_update(&mut self) {

        self.context.swap_buffers(&mut self.window)
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

/// Just the implementation for the windows window struct. All it has is a base constructor for now,
/// could be extended to contain added functionality later.

impl WindowsWindow {

    fn new(properties : WindowProperties, sdl : &Sdl) -> WindowsWindow {

        let mut video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let mut window = video_subsystem.window( &properties.title, properties.width, properties.height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let mut context = OpenGLContext::new(&mut window, &mut video_subsystem);

        let data = WindowData {
            title : properties.title.clone(),
            width : properties.width.clone(),
            height : properties. height.clone(),
            vsync : true,
        };

        let mut window = WindowsWindow {
            window,
            video : video_subsystem,
            context,
            data
        };

        window.set_vsync(true);

        window
    }
}

/// A struct which contains base window data. This has the title, width, height, and vsync details
/// within.

pub struct WindowData {

    title : String,
    pub width : u32,
    pub height : u32,
    pub vsync : bool,
}

/// This handles all base window events. In its current iteration, the function merely takes in an
/// sdl2 window event, as well as a window event struct, checks the event for its type and passes in relevant
/// functions.

pub fn process_event(window_event : &sdl2::event::WindowEvent, event : &mut WindowEvent) {

    match window_event {
        // When the window is closed, the application should close down.
        sdl2::event::WindowEvent::Close => { event.events.push_back(Box::new(on_window_close))},

        // When the screen is resized it should update the current window and log the information to the console.
        sdl2::event::WindowEvent::Resized(x, y)
        => {
             // Set the width and height appropriately
             event.window.data.width = *x as u32;
             event.window.data.height = *y as u32;
             // Push the event which logs the information into the appropriate queue.
             event.events.push_back(Box::new(on_window_resized))
           }
        // TODO
        sdl2::event::WindowEvent::Minimized => println!("{}", "minimized"),
        // TODO
        sdl2::event::WindowEvent::Exposed => println!("{}", "exposed"),
        // TODO
        sdl2::event::WindowEvent::FocusGained => println!("{}", "focus gained"),
        // TODO
        sdl2::event::WindowEvent::Enter => println!("{}", "Mouse entered"),
        // TODO
        sdl2::event::WindowEvent::TakeFocus => println!("{}", "Taking focus"),
        // TODO
        _ => ()
    }
}

/// Not sure if this function should be left alone. So far it just closes the entire application.

#[inline] pub fn on_window_close<'a>(event : &mut WindowsWindow) {

    println!("WINDOW: Window closed, Exiting {}.", event.data.title);
    process::exit(1)
}

/// Logs the new height and width of the window after a resize has occurred.
/// TODO: Add more functionality when rendering is actually put into place.

#[inline] pub fn on_window_resized(event : &mut WindowsWindow) {

    println!("{} {} {}", "WINDOW: Resized:", event.data.width, event.data.height);
}