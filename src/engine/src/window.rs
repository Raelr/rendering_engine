use failure::Error;
use sdl2::Sdl;
use crate::events::event::EventTrait;
use crate::platform::windows::windows_window::WindowData;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

#[macro_export]
// Macro for creating a key typed event.
macro_rules! window_base {
    () => {{
        let window = WindowProperties::new("Scrapyard Engine", 1280, 720);
        window?
    }};
}

////////////////////////////////////
//         M E T H O D S          //
////////////////////////////////////

pub struct WindowProperties {

    pub title : String,
    pub width : u32,
    pub height : u32
}

impl WindowProperties {

    pub fn new(title : &str, width : u32, height : u32) -> Result<WindowProperties, Error> {

        let window = WindowProperties {
            title : String::from(title),
            width,
            height
        };

        Ok(window)
    }
}

pub trait WindowTrait {

    fn drop(&mut self) {}
    fn on_update(&mut self) {}
    fn get_width(&self) -> u32 {0}
    fn get_height(&self) -> u32 {0}
    fn set_vsync(&mut self, enabled : bool) {}
    fn is_vsync(&self) -> &bool {&false}
    fn get_native_window(&self) {}
    fn get_data(&mut self) -> &mut WindowData;
    fn create(properties : WindowProperties, sdl: &Sdl) -> Result<Box<dyn WindowTrait>, Error> where Self : Sized;
}

