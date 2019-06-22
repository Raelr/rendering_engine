use crate::events::EventType::{KeyPressed, KeyReleased, KeyTyped, MouseMoved, MouseScrolled, MouseButtonPressed, MouseButtonReleased, NONE};

// Enums for determining an event type.
#[derive(Display, Debug)]
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
        _ => NONE
    };

    e_type
}

#[macro_use] pub mod event;
#[macro_use] pub mod key_event;
#[macro_use] pub mod mouse_changed;
#[macro_use] pub mod mouse_button_event;






