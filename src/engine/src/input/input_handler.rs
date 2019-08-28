use crate::input;
use crate::input::{KeyCode, MouseInput};
use std::collections::hash_map::HashMap;

pub struct InputHandler {

    keyboard_pressed: Vec<input::KeyCode>,
    mouse_pressed: Vec<input::MouseInput>,
    held_mouse_buttons: HashMap<MouseInput, i32>,
    held_keys: HashMap<KeyCode, i32>
}

impl InputHandler {

    pub fn new() -> InputHandler {
        InputHandler { keyboard_pressed: vec!(), mouse_pressed: vec!(), held_mouse_buttons: HashMap::new(), held_keys: HashMap::new()}
    }

    pub fn update_input_state(&mut self, pump: &mut sdl2::EventPump) {

       self.keyboard_pressed =  pump.keyboard_state().pressed_scancodes()
           .filter(|scancode| {input::is_registered_input(&input::scancode_to_keycode(scancode))})
           .map(|scancode| { input::scancode_to_keycode(&scancode)})
           .collect::<Vec<KeyCode>>();

        self.mouse_pressed =  pump.mouse_state().pressed_mouse_buttons()
            .filter(|mouse| { input::is_registered_mouse_input({&input::sdl_mouse_to_mouse(mouse)})})
            .map(|mouse| { input::sdl_mouse_to_mouse(&mouse) })
            .collect::<Vec<MouseInput>>();

        let size = self.mouse_pressed.len();

        for index in 0..size {

            let input = &self.mouse_pressed[index].clone();

            if let Some(i) = self.held_mouse_buttons.get_mut(input) {
                *i = *i + 1;
            } else {
                self.held_mouse_buttons.insert(*input, 1);
            }
        }

        let size = self.keyboard_pressed.len();

        for index in 0..size {

            let input = &self.keyboard_pressed[index].clone();

            if let Some(i) = self.held_keys.get_mut(input) {
                *i = *i + 1;
            } else {
                self.held_keys.insert(*input, 1);
            }
        }
    }

    pub fn clear_mouse_input(&mut self, button: &MouseInput) {

        if let Some(count) = self.held_mouse_buttons.get_mut(button) {
            *count = 0;
        }
    }

    pub fn clear_keyboard_input(&mut self, key: &KeyCode) {

        if let Some(count) = self.held_keys.get_mut(key) {
            *count = 0;
        }
    }

    pub fn get_keycode_down(&self, code :  &KeyCode) -> bool {

        self.keyboard_pressed.contains(code)
    }

    pub fn get_keycode(&self, key : &KeyCode) -> bool {

        if let Some(count) = self.held_keys.get(key) {
            return count == &1
        } else {
            false
        }
    }

    pub fn get_mouse_button(&self, button : &MouseInput) -> bool {

        if let Some(count) = self.held_mouse_buttons.get(button) {
            return count == &1
        } else {
            false
        }
    }

    pub fn get_mouse_down(&self, button :  &MouseInput) -> bool {
        self.mouse_pressed.contains(button)
    }
}