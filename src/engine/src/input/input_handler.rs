use crate::input;
use sdl2::keyboard::Keycode;
use crate::input::{KeyCode, MouseInput};

pub struct InputHandler {

    keyboard_pressed: Vec<input::KeyCode>,
    mouse_pressed: Vec<input::MouseInput>
}

impl InputHandler {

    pub fn new() -> InputHandler {
        InputHandler { keyboard_pressed: vec!(), mouse_pressed: vec!() }
    }

    pub fn update_input_state(&mut self, pump: &mut sdl2::EventPump) {

       self.keyboard_pressed =  pump.keyboard_state().pressed_scancodes()
           .filter(|scancode| { println!("Adding KeyEvent: {}", scancode); input::is_registered_input(&input::scancode_to_keycode(scancode))})
           .map(|scancode| { input::scancode_to_keycode(&scancode)})
           .collect::<Vec<KeyCode>>();

        self.mouse_pressed =  pump.mouse_state().pressed_mouse_buttons()
            .filter(|mouse| { input::is_registered_mouse_input({println!("Adding mouseevent"); &input::sdl_mouse_to_mouse(mouse)})})
            .map(|mouse| { input::sdl_mouse_to_mouse(&mouse) })
            .collect::<Vec<MouseInput>>();
    }

    pub fn get_keycode_down(&self, code :  &KeyCode) -> bool {

        self.keyboard_pressed.contains(code)
    }

    pub fn get_mouse_down(&self, button :  &MouseInput) -> bool {

        self.mouse_pressed.contains(button)
    }
}