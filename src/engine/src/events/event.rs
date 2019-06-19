use failure::*;
use strum::AsStaticRef;

#[derive(Display, Debug)]
pub enum EventType {
    None = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}
#[derive(Display, Debug)]
pub enum EventCategory {
    None = 0,
    EventCategoryApplication    = 1,
    EventCategoryInput          = 2,
    EventCategoryKeyboard       = 3,
    EventCategoryMouse          = 4,
    EventCategoryMouseButton    = 5
}

#[allow(unused_variables)]
pub trait EventHandler {

    // Needs event dispatcher;
    fn get_event_type(&self) -> Result<EventType, Error> { Ok(EventType::None)}
    fn get_name(&self) -> Result<String, Error> { Ok(String::new()) }
    fn get_category_flags(&self) -> Result<i32, Error> { Ok(0)}
    fn to_string(&self) -> Result<String, Error>;
    fn is_in_category(&self, category : EventCategory) -> Result<bool, Error> { Ok(false)}
}
