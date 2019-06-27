// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

use crate::window::{WindowProperties, WindowTrait};
// Use
use failure::Error;
use sdl2::video::Window;
use sdl2::Sdl;
use self::sdl2::video::SwapInterval::{VSync, Immediate};
use crate::renderer::render_application::initialise;
use crate::events::event::{EventTrait, Event};
use std::convert::TryInto;

pub struct WindowsWindow {

    window : Window,
    events : sdl2::EventPump,
    video : sdl2::VideoSubsystem,
    context : sdl2::video::GLContext,
    pub data : WindowData
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
                sdl2::event::Event::Quit{timestamp} => (println!("{} {}", "Quit event detected at time: ", timestamp)),
                sdl2::event::Event::Window{timestamp, window_id, win_event} => { match win_event {
                                                                                                            sdl2::event::WindowEvent::Close => println!("{}", "Window Closed"),
                                                                                                            sdl2::event::WindowEvent::Resized(1270, 720) => println!("{}", "Window resized"),
                                                                                                            sdl2::event::WindowEvent::HitTest => println!("{}", "Hit test"),
                                                                                                            sdl2::event::WindowEvent::Minimized => println!("{}", "minimized"),
                                                                                                            sdl2::event::WindowEvent::Exposed => println!("{}", "exposed"),
                                                                                                            sdl2::event::WindowEvent::FocusGained => println!("{}", "focus gained"),
                                                                                                            sdl2::event::WindowEvent::Enter => println!("{}", "Mouse entered"),
                                                                                                            sdl2::event::WindowEvent::TakeFocus => println!("{}", "Taking focus"),
                                                                                                            _ => ()
                                                                                                            }},
                sdl2::event::Event::MouseButtonDown{timestamp, window_id, which, mouse_btn, clicks,x, y}
                            => println!("Mouse Clicked at position: {},{}", x, y),
                _ => ()
            }
        }

        self.window.gl_swap_window();
    }

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

    fn get_data(&mut self) -> &mut WindowData {
        &mut self.data
    }

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
    pub callback : Option<fn(Box<FnMut(Box<EventTrait>)>)>
}

impl WindowData {

    pub fn set_event_callback<CB : 'static + FnMut(Box<EventTrait>)> (&self, call_back : CB) {

    }
}
