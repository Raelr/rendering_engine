use sdl2::Error;

pub enum EventType {
    None = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

pub enum EventCategory {
    None = 0,
    EventCategoryApplication    = 1,
    EventCategoryInput          = 2,
    EventCategoryKeyboard       = 3,
    EventCategoryMouse          = 4,
    EventCategoryMouseButton    = 5
}

pub struct Event {

}

pub trait EventHandler {

    // Needs event dispatcher;
    fn get_event_type(&self) -> Result<EventType, Err>;
    fn get_name(&self) -> Result<String, Err>;
    fn get_category_flags(&self) -> Result<i32, Err>;
    fn to_string(&self) -> Result<String, Err>;
    fn is_in_category(&self, category : EventCategory) -> bool {
        get_category_flags() & category
    }
}
