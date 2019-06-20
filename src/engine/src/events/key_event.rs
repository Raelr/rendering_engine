use crate::events::*;

use failure::*;
use crate::events::event::Event;

pub struct KeyEvent {

    key_code : i32,
    repeat_count : Option<i32>,
    event : Event,
}

impl event::EventTrait for KeyEvent {

    fn get_event_type(&self) -> Result<&event::EventType, Error> {
        Ok(self.event.get_event_type()?)
    }

    fn get_category_flags(&self) -> Result<&event::EventCategory, Error> {
        Ok(self.event.get_category_flags()?)
    }

    fn to_string(&self) -> Result<String, Error> {

        let debug = match self.repeat_count {

            Option::None => format!("{}: {}", self.event.get_name()?, self.key_code),
            _ => format!("{}: {} ({} repeats)", self.event.get_name()?, self.key_code, self.repeat_count.unwrap())
        };

        Ok (debug)
    }

    fn is_in_category(&self, category : &event::EventCategory) -> Result<bool, Error> {
        self.event.is_in_category(category)
    }
}

impl KeyEvent {

    fn get_key_code(&self) -> i32 {
        self.key_code
    }

    fn get_repeat_count(&self) -> i32 {
        self.repeat_count.unwrap()
    }

    fn new(key_code : i32, repeat_count : Option<i32>, event_type : event::EventType) -> Result<KeyEvent, Error> {

        let flags = event::EventCategory::EVENT_CATEGORY_KEYBOARD  | event::EventCategory::EVENT_CATEGORY_INPUT;

        let event = Event::new(event_type, flags)?;

        let key_event = KeyEvent {
            key_code,
            repeat_count,
            event
        };

        Ok(key_event)
    }

    pub fn from_key_pressed(key_code : i32, repeat_count : Option<i32>) -> Result<KeyEvent, Error> {

        Ok(KeyEvent::new(key_code, repeat_count, event::EventType::KeyPressed)?)
    }

    pub fn from_key_released(key_code : i32) -> Result<KeyEvent, Error> {

        Ok(KeyEvent::new(key_code, Option::None, event::EventType::KeyReleased)?)
    }

    pub fn from_key_typed(key_code : i32) -> Result<KeyEvent, Error> {

        Ok(KeyEvent::new(key_code, Option::None, event::EventType::KeyTyped)?)
    }
}





