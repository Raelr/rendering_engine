use failure::Error;
use crate::events::event::EventTrait;

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

    title : String,
    width : u32,
    height : u32
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
    fn on_update() {}
    fn get_width(&self) -> u32 {0}
    fn get_height(&self) -> u32 {0}
    fn set_event_callback<E : EventTrait>(call_back : fn(&E)) {}
    fn set_vsync(&self, enabled : bool) {}
    fn is_vsync(&self) -> &bool {&false}
    fn get_native_window(&self) {}
    fn create(properties : &WindowProperties) {}
}

