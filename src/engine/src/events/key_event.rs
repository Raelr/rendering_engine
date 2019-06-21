use crate::events::*;
use crate::events::event::Event;
use event::EventType;
use failure::*;
use crate::events::event::EventType::KeyPressed;
use failure::Error;
use std::process;

#[macro_export]
// Macro for creating a key event. Returns an event regardless of what happens, but quits the process if there is an error.
macro_rules! key_event {
    ($key_code:expr, $repeat_count:expr, $e_type:expr) => {{
        let pressed = key_event::KeyEvent::new($key_code, $repeat_count, $e_type);

        let is_ok = match &pressed {
            Result::Ok(v) => pressed?,
            Result::Err(e) => { eprintln!("Error: {}", e);
                                std::process::exit(1);
                                pressed? }
        };

        is_ok
    }};
}

// A general event type for key inputs.
pub struct KeyEvent {

    key_code : i32,
    repeat_count : i32,
    event : Event,
}

// KeyEvent implementation of base event methods so that they can access basic events
impl event::EventTrait for KeyEvent {

    // Access the methods in the base event struct.
    fn get_event_type(&self) -> &EventType {
        self.event.get_event_type()
    }

    fn get_category_flags(&self) -> &event::EventCategory {
        self.event.get_category_flags()
    }

    // Only print repeat count if a keypressed event is generated
    fn to_string(&self) -> String{

        let debug = match self.get_event_type() {

            EventType::KeyPressed => format!("{}: {} ({} repeats)", self.event.get_name(), self.key_code, self.repeat_count),
            _ => format!("{}: {}", self.event.get_name(), self.key_code),
        };

        debug
    }

    // Calls the is_in_category method in the base event struct.
    fn is_in_category(&self, category : &event::EventCategory) -> bool {
        self.event.is_in_category(category)
    }
}

impl KeyEvent {

    // Get the key code of the input
    fn get_key_code(&self) -> &i32 {
        &self.key_code
    }

    // Get the repeat count variable.
    fn get_repeat_count(&self) -> &i32 {
        &self.repeat_count
    }

    // Creates a new generic instance of the class. Makes sure that you cant generate a KeyPressed event without passing a KeyPressed enum in first.
    pub fn new(key_code : i32, repeat_count : i32, event_type : event::EventType) -> Result<KeyEvent, Error> {

        // Check for KeyPressed
        let is_pressed = match event_type {
            KeyPressed => true,
            _ => false
        };

        // Make sure that input is correct.
        if repeat_count > -1 && !is_pressed || is_pressed && repeat_count == -1{
            return Err(format_err!("Invalid event creation. You either have a KeyPressed event with a \
            repeat_count of -1, or you have a non KeyPressed event with a keycount > -1."));
        }

        // Create flags specific to this event
        let flags = event::EventCategory::EVENT_CATEGORY_KEYBOARD  | event::EventCategory::EVENT_CATEGORY_INPUT;

        let key_event = KeyEvent {
            key_code,
            repeat_count,
            event : event!(event_type, flags)
        };

        // return event.
        Ok(key_event)
    }
}





