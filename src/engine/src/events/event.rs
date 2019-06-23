use crate::events;
use events::EventType;
use failure::Error;
use crate::events::EventType::WindowClose;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

// Macro for creating base event
#[macro_export]
macro_rules! event {
    ($event_type:expr, $event_category:expr) => {{
        let e = event::Event::new($event_type, $event_category);
        e
    }};
}

////////////////////////////////////
//         M E T H O D S          //
////////////////////////////////////

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
    fn get_name(&self) -> String { String::new() }
    fn get_category_flags(&self) -> &EventCategory;
    fn to_string(&self) -> String {String::new()}
    #[inline] fn is_in_category(&self, category : &EventCategory) -> bool { false}
    #[inline] fn set_is_handled(&mut self, value: bool) { }
}

// Base event struct. To be included in ALL event modules.
pub struct Event {

    // Needs event dispatcher;
    event_type : EventType,
    flags : EventCategory,
    is_handled : bool
}

impl Event {
    // Instntiates a new event.
    pub fn new(event_type : EventType, flags: EventCategory) -> Event{
        Event {event_type, flags, is_handled : false}
    }
}

// Base event for all event classes.
impl EventTrait for Event {
    // Returns the event type in its enum form.
    fn get_event_type(&self) -> &EventType{
        &self.event_type
    }

    // Get the name of the struct (taken from the type)
    fn get_name(&self) -> String {

        self.event_type.to_string()
    }

    // Get the bits of the category flags.
    fn get_category_flags(&self) -> &EventCategory {
        &self.flags
    }

    // Performs a bitwise operator to check if an enum falls into the correct category.
    #[inline] fn is_in_category(&self, category : &EventCategory) -> bool {

        (category.to_owned() & self.get_category_flags().to_owned()) != EventCategory::NONE
    }

    #[inline] fn set_is_handled(&mut self, value : bool) {
        self.is_handled = value
    }
}

// Event dispatcher class.
pub struct EventDispatcher {

    // This acts as a means to compare whether incoming events suit this specific type.
    event : Box<dyn EventTrait>
}

// struct for the event dispatcher. Mainly handles the dispatching of appropriate functions as callbacks.
impl EventDispatcher {

    // Creates a new instance of the event dispatcher.
    pub fn new<E: EventTrait + 'static >(event : E) -> Result<EventDispatcher, Error> {

        let dispatcher = EventDispatcher { event : Box::new(event)};

        Ok(dispatcher)
    }

    // Takes in an event, as well as a function to use that event.
    pub fn dispatch<E: EventTrait + 'static>(&mut self, event : &E, func : fn(&E) -> bool)  -> bool {

        if event.get_event_type() == self.event.get_event_type() {
            self.event.set_is_handled(func(&(&event as &E)));
            return true
        }
        false
    }
}




