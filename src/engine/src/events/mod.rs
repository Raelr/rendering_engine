// Enums for determining an event type.
#[derive(Display, Debug)]
pub enum EventType {
    NONE = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[macro_use] pub mod event;
#[macro_use] pub mod key_event;
#[macro_use] pub mod mouse_event;







