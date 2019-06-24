use crate::events::event::{Event, EventTrait};
use crate::events::event;
use crate::events::*;
use crate::events::EventType;

use failure::*;
use failure::Error;
use event::EventCategory;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

#[macro_export]
// Macro for creating a mouse pressed event.
macro_rules! m_button_pressed {
    ($x:expr) => {{
        let button_pressed = MouseButtonEvent::new($x, 5);
        button_pressed?
    }};
}

#[macro_export]
// Macro for creating a mouse released event.
macro_rules! m_button_released {
    ($x:expr) => {{
        let button_pressed = MouseButtonEvent::new($x, 6);
        button_pressed?
    }};
}

////////////////////////////////////
//         M E T H O D S          //
////////////////////////////////////

// Struct for the MouseButtonEvent
pub struct MouseButtonEvent {
    button : i32,
    event : Event
}

impl MouseButtonEvent {

    // Constructor for the MouseButtonEvent struct.
    // Should never be used in standard code -> use macros listed above to create MouseButtonPressed and MouseButtonReleased events.
    // Code takes in an integer for the mouse button, and an unsigned int for the type.
    // The types are as follows:
    // 5 = Creates a MouseButtonPressed event.
    // 6 = Creates a MouseButtonReleased event.

    pub fn new(button : i32, e_type : u8) -> Result<MouseButtonEvent, Error> {

        if e_type < 5 || e_type > 6 {
            return Err(format_err!("Invalid type entered: must be either 5 (MouseButtonPressed) or 6 (MouseButtonReleased)."))
        }

        let flags = event::EventCategory::EVENT_CATEGORY_MOUSE | event::EventCategory::EVENT_CATEGORY_INPUT;

        let event = MouseButtonEvent {
            button,
            event: event!(get_type_from_int(e_type), flags)
        };

        Ok(event)
    }

    // Returns the mouse button.
    #[inline] pub fn get_mouse_button(&self) -> &i32 {
        &self.button
    }
}

// Implementation of the base Event trait for this event.
impl EventTrait for MouseButtonEvent {

    // Returns the event type
    fn get_event_type(&self) -> &EventType {
        &self.event.get_event_type()
    }

    // Returns the event name
    fn get_name(&self) -> String {
        self.event.get_name()
    }

    // Returns the event's category flags
    fn get_category_flags(&self) -> &EventCategory {
        &self.event.get_category_flags()
    }

    // Prints the event's information
    fn to_string(&self) -> String {

        let debug = format!("{}: {}", self.get_name(), self.get_mouse_button());

        debug
    }

    // Checks if the event falls into a specific category.
    #[inline] fn is_in_category(&self, category: &EventCategory) -> bool {
        self.event.is_in_category(category)
    }

    #[inline] fn set_is_handled(&mut self, value : bool) {
        self.event.set_is_handled(value)
    }

    #[inline] fn get_is_handled(&self) -> &bool {
        self.event.get_is_handled()
    }
}