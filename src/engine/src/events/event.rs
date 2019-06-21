use failure::*;

// Macro for creating base event
#[macro_export]
macro_rules! event {
    ($event_type:expr, $event_category:expr) => {{
        let e = event::Event::new($event_type, $event_category);
        e
    }};
}

// Enums for determining an event type.
#[derive(Display, Debug)]
pub enum EventType {
    NONE = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

// bitflags for checking when an event falls into a category.
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

// Set of methods to be used for every event.
#[allow(unused_variables)]
pub trait EventTrait {

    fn get_event_type(&self) -> &EventType { &EventType::NONE}
    fn get_name(&self) -> Result<String, Error> { Ok(String::new()) }
    fn get_category_flags(&self) -> Result<&EventCategory, Error>;
    fn to_string(&self) -> String;
    fn is_in_category(&self, category : &EventCategory) -> bool { false}
}

// Base event struct. To be included in ALL event modules.
pub struct Event {

    // Needs event dispatcher;
    event_type : EventType,
    flags : EventCategory
}

// Base event for all event classes.
impl Event {

    // Instntiates a new event.
    pub fn new(event_type : EventType, flags: EventCategory) -> Event{
        Event {event_type, flags}
    }

    // Get the bits of the category flags.
    pub fn get_category_flags(&self) -> Result<&EventCategory, Error> {
        Ok(&self.flags)
    }

    // Get the name of the struct (taken from the type)
    pub fn get_name(&self) -> String {

        self.event_type.to_string()
    }

    // Returns the event type in its enum form.
    pub fn get_event_type(&self) -> &EventType{
        &self.event_type
    }

    // Performs a bitwise operator to check if an enum falls into the correct category.
    pub fn is_in_category(&self, category : &EventCategory) -> bool {

        (category.to_owned() & self.get_category_flags().unwrap().to_owned()) != EventCategory::NONE
    }
}




