use crate::events::*;

use event::*;
use event::EventHandler;
use failure::*;
use crate::events::event::EventType::*;

pub struct KeyEvent {

    key_code : i32,
    repeat_count : Option<i32>,
    event_type : event::EventType,
    flags : Vec<event::EventCategory>
}

trait KeyEventTrait : event::EventHandler {

    fn get_key_code(&self) ->  i32;
    fn get_repeat_count(&self) -> i32;

}

impl event::EventHandler for KeyEvent{

     fn get_name(&self) -> Result<String, Error> {

        Ok((self.event_type.to_string()))
    }

    fn to_string(&self) -> Result<String, Error> {

        let debug = match self.repeat_count {

            Option::None => format!("{}: {}", self.get_name()?, self.key_code),
            _ => format!("{}: {} ({} repeats)", self.get_name()?, self.key_code, self.repeat_count.unwrap())
        };

        Ok (debug)
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

        let flags = vec![event::EventCategory::EventCategoryKeyboard, event::EventCategory::EventCategoryInput];

        let event = KeyEvent {
            key_code,
            repeat_count,
            event_type,
            flags
        };

        Ok(event)
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





