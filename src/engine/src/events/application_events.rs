use crate::events::event::{EventTrait, Event};
use crate::events::{EventType, event};
use crate::events;
use failure::Error;
use events::*;
use crate::events::EventType::WindowResize;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

#[macro_export]
// Macro for creating a window resized event.
macro_rules! window_resize {
    ($width:expr, $height:expr) => {{
        let event = WindowResizeEvent::new($width, $height);
        event?
    }};
}

#[macro_export]
// Macro for creating a window close event.
macro_rules! window_close {
    () => {{
        let event = BaseWindowEvent::new(7);
        event?
    }};
}

#[macro_export]
// Macro for creating a window apptick event.
macro_rules! app_tick {
    () => {{
        let event = BaseWindowEvent::new(8);
        event?
    }};
}

#[macro_export]
// Macro for creating a window app update event.
macro_rules! app_update {
    () => {{
        let event = BaseWindowEvent::new(9);
        event?
    }};
}


#[macro_export]
// Macro for creating a window app render event.
macro_rules! app_render {
    () => {{
        let event = BaseWindowEvent::new(10);
        event?
    }};
}

////////////////////////////////////
//         M E T H O D S          //
////////////////////////////////////

// window resize event class. Takes in a width and a height
pub struct WindowResizeEvent {
    event : Event,
    width : u32,
    height : u32
}

impl WindowResizeEvent {

    // Takes in width and height - doesnt need an e_type  since it is hardcoded in.

    pub fn new (width : u32, height : u32) -> Result<WindowResizeEvent, Error>{

        let flags = events::event::EventCategory::EVENT_CATEGORY_APPLICATION;

        let event = WindowResizeEvent {
            event : event!(WindowResize, flags),
            width,
            height
        };

        Ok(event)
    }

    // Return the width value of the window
    #[inline] pub fn get_width(&self) -> &u32 {
        &self.width
    }

    // Return the height value of the window
    #[inline] pub fn get_height(&self) -> &u32 {
        &self.height
    }
}

// Implementation of the Event trait class on the WindowResize event.
impl EventTrait for WindowResizeEvent {

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

        let debug = format!("{}: {}, {}", self.get_name(), self.get_width(), self.get_height());

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

// Base class for any other window classes which we may need to implement. For now, this is empty,
// may require further editing in future iterations.
pub struct BaseWindowEvent {
    event : Event
}

// Implementation, takes in a simple constructor.
impl BaseWindowEvent {

    // Constructor, can create a number of events: WindowClose, AppTick, AppUpdate, and AppRender
    // These are signified in the following numbers:
    // 7 = WindowCloseEvent
    // 8 = AppTick
    // 9 = AppUpdate
    // 10 = AppRender
    // NOTE -> NEVER use this constructor. Use macros to create event instead.

    pub fn new(e_type : u8) -> Result<BaseWindowEvent, Error> {

        let flags = events::event::EventCategory::EVENT_CATEGORY_APPLICATION;

        let event = BaseWindowEvent {
          event: event!(get_type_from_int(e_type), flags)
        };

        Ok(event)
    }
}

// Implementation of Event Trait for the BaseWindowEvent class.
impl EventTrait for BaseWindowEvent {

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

        let debug = format!("{}", self.get_name());

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
