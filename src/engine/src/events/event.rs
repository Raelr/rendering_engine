use failure::*;
use strum::AsStaticRef;
use sdl2::event::Event;

#[derive(Display, Debug)]
pub enum EventType {
    NONE = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

bitflags! {

    pub struct EventCategory : i32 {
        const NONE = 0;
        const EVENT_CATEGORY_APPLICATION    = bit!(1);
        const EVENT_CATEGORY_INPUT          = bit!(2);
        const EVENT_CATEGORY_KEYBOARD       = bit!(3);
        const EVENT_CATEGORY_MOUSE          = bit!(4);
        const EVENT_CATEGORY_MOUSEBUTTON    = bit!(5);
    }
}

//pub enum EventCategory {
//    None = 0,
//    EventCategoryApplication    = bit!(1),
//    EventCategoryInput          = bit!(2),
//    EventCategoryKeyboard       = bit!(3),
//    EventCategoryMouse          = bit!(4),
//    EventCategoryMouseButton    = bit!(5)
//}

#[allow(unused_variables)]
pub trait EventHandler {

    // Needs event dispatcher;
    fn get_event_type(&self) -> Result<&EventType, Error> { Ok(&EventType::NONE)}
    fn get_name(&self) -> Result<String, Error> { Ok(String::new()) }
    fn get_category_flags(&self) -> Result<EventCategory, Error>;
    fn to_string(&self) -> Result<String, Error>;
    fn is_in_category(&self, category : EventCategory) -> Result<bool, Error> { Ok(false)}
}
