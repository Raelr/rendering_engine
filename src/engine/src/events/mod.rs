use crate::events::EventType::{KeyPressed, KeyReleased, KeyTyped, MouseMoved, MouseScrolled, MouseButtonPressed, MouseButtonReleased, NONE, WindowClose, Apptick, AppUpdate, AppRender};

// Enums for determining an event type.
pub enum EventType {
    NONE = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

// Gets event types by comparing an unsigned int.
fn get_type_from_int(code : u8) -> EventType {

    let e_type = match code {
        0 => KeyPressed,
        1 => KeyReleased,
        2 => KeyTyped,
        3 => MouseMoved,
        4 => MouseScrolled,
        5 => MouseButtonPressed,
        6 => MouseButtonReleased,
        7 => WindowClose,
        8 => Apptick,
        9 => AppUpdate,
        10 => AppRender,
        _ => NONE
    };

    e_type
}

pub mod window_event;







