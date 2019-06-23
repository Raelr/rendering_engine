use crate::events::*;
use crate::events::event::Event;
use EventType;
use failure::*;
use failure::Error;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

#[macro_export]
// Macro for creating a mouse moved event.
macro_rules! mouse_moved {
    ($x:expr, $y:expr) => {{
        let mouse_moved = MouseChangedEvent::new($x, $y, 3);
        mouse_moved?
    }};
}

#[macro_export]
// Macro for creating a mouse scrolled event.
macro_rules! mouse_scrolled {
    ($x:expr, $y:expr) => {{
        let mouse_scrolled = MouseChangedEvent::new($x, $y, 4);
        mouse_scrolled?
    }};
}

////////////////////////////////////
//         M E T H O D S          //
////////////////////////////////////

// Basic struct for a generic mouse event. This event handles both mouse movement and scrolling.
pub struct MouseChangedEvent {
    event: Event,
    m_mouse_x : f64,
    m_mouse_y : f64
}

//Basic implementation, has a constructor and allows the x and y values which were passed in to be taken.
impl MouseChangedEvent {

    // Constructor, should never be used outside of macros -> use macros listed above instead.
    // Takes in a number of arguments to construct the struct. Also needs a type (specified by the e_type u8 value).
    // Can only take specific types, these are:
    // 3: MouseMoved event (for when the mouse is moved)
    // 4: MouseScrolled event (for when the scroll wheel is used.)
    // NOTE: Both types take in the same inputs, but both should be used for their specific purposes.

    pub fn new(m_mouse_x : f64, m_mouse_y : f64, e_type : u8) -> Result<MouseChangedEvent, Error> {

        if e_type < 3 || e_type > 4 {
            return Err(format_err!("Invalid type entered: must be either 3 (MouseMoved) or 4 (MouseScrolled)."))
        }

        let flags = event::EventCategory::EVENT_CATEGORY_MOUSE | event::EventCategory::EVENT_CATEGORY_INPUT;

        let event = MouseChangedEvent {
            event : event!(get_type_from_int(e_type), flags),
            m_mouse_x,
            m_mouse_y
        };

        Ok(event)
    }

    // Return x value
    #[inline] fn get_x(&self) -> &f64 {
        &self.m_mouse_x
    }
    // Return y value
    #[inline] fn get_y(&self) -> &f64 {
        &self.m_mouse_y
    }
}

// Implementation of the base EventTrait.
impl event::EventTrait for MouseChangedEvent {

    // Returns the vent type
    fn get_event_type(&self) -> &EventType {
        self.event.get_event_type()
    }

    // Returns the event's name
    fn get_name(&self) -> String {
        self.event.get_name()
    }

    // Returns the bitflag of the event
    fn get_category_flags(&self) -> &event::EventCategory {
        self.event.get_category_flags()
    }

    // Prints out the values in the event.
    fn to_string(&self) -> String {

        let debug = format!("{}: {}, {}", self.get_name(), self.get_x(), self.get_y());

        debug
    }

    // Checks if the event fits in a specific category.
    #[inline] fn is_in_category(&self, category: &event::EventCategory) -> bool {
        self.event.is_in_category(category)
    }

    #[inline] fn set_is_handled(&mut self, value : bool) {
        self.event.set_is_handled(value)
    }
}
