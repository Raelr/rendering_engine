use crate::events::*;

struct KeyEvent {
    pub keycode : i32,
}

trait KeyEventTrait {

    fn get_key_code(&self) -> Result<i32, Err> {
        Ok(keycode)
    }

    fn init_base_event(i32 : keycode) -> Result<KeyEvent, Err> {
        Ok(KeyEvent {
            keycode
        })
    }
}

impl events::EventHandler for keyEvent {

}

impl KeyEventTrait for KeyEvent {

}

pub struct KeyPressedEvent {

    key_event : KeyEvent,
    repeat_count : i32
}

impl KeyEventTrait for KeyPressedEvent {

    fn get_key_code(&self) -> Result<i32, _> {

        Ok(key_event.keycode)
    }

    fn new_pressed_event(key_code : i32, repeat_count : i32) -> Result<KeyPressedEvent, Err> {

    }
}