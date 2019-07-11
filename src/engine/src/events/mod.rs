use crate::events::EventType::{KeyPressed, KeyReleased, KeyTyped, MouseMoved, MouseScrolled, MouseButtonPressed, MouseButtonReleased, NONE, WindowClose, Apptick, AppUpdate, AppRender};

// Enums for determining an event type.
pub enum EventType {
    NONE = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

pub mod window_event;







