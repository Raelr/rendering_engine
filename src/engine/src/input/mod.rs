use sdl2::Error;
use sdl2::keyboard::Keycode;
use sdl2::event::EventType::MouseButtonDown;
use nalgebra::Vector2;

pub mod input_handler;

#[derive(PartialEq)]
pub enum KeyCode {

    W, A, S, D,
    Up, Down, Left, Right,
    NA
}
#[derive(PartialEq)]
pub enum MouseInput {

    Left, Right, Middle,
    NA
}

pub fn scancode_to_keycode(scancode: &sdl2::keyboard::Scancode) -> KeyCode {

    match scancode {

        sdl2::keyboard::Scancode::A => KeyCode::A,
        sdl2::keyboard::Scancode::W => KeyCode::W,
        sdl2::keyboard::Scancode::S => KeyCode::S,
        sdl2::keyboard::Scancode::D => KeyCode::D,

        sdl2::keyboard::Scancode::Up => KeyCode::Up,
        sdl2::keyboard::Scancode::Down => KeyCode::Down,
        sdl2::keyboard::Scancode::Left => KeyCode::Left,
        sdl2::keyboard::Scancode::Right => KeyCode::Right,
        _ => KeyCode::NA
    }
}

pub fn sdl_mouse_to_mouse(mouse: &sdl2::mouse::MouseButton) -> MouseInput {

    match mouse {

        sdl2::mouse::MouseButton::Left => MouseInput::Left,
        sdl2::mouse::MouseButton::Left => MouseInput::Right,
        sdl2::mouse::MouseButton::Left => MouseInput::Middle,
        _ => MouseInput::NA
    }
}

pub fn is_registered_input(code : &KeyCode) -> bool {

    code != &KeyCode::NA
}

pub fn is_registered_mouse_input(code : &MouseInput) -> bool {

    code != &MouseInput::NA
}

pub fn get_mouse_coordinates(pump: &sdl2::EventPump) -> Vector2<f32>{
    let state = pump.mouse_state();
    Vector2::new(state.x() as f32, state.y() as f32)
}
