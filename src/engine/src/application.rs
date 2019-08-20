// Crates
extern crate gl;
extern crate failure;

// Internal crates:
use crate::ecs::{PositionComponent, ColorComponent, Texture, RenderComponent, TextureMixComponent, TextureUpdateComponent};
use crate::ecs::*;
use crate::generational_index::generational_index::GenerationalIndex;
use crate::ecs::render_system::RenderSystem;
use crate::ecs::texture_update_system::TextureUpdateSystem;
use crate::ecs::system::System;
use crate::ecs::position_update_system::PositionUpdateSystem;
use crate::ecs::check_mouse_collision_system::CheckBoxColliderSystem;
use crate::events::window_event::WindowEvent;
use crate::game_state::GameState;
use crate::platform::windows::windows_window;
use crate::window::{WindowProperties, WindowTrait};
use crate::platform::windows::windows_window::{WindowsWindow};
use crate::sdl2::keyboard::Scancode;
use crate::sdl2::mouse::MouseButton;
use crate::input::MouseInput;

// Use
use failure::Error;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use nalgebra::*;
use sdl2::controller::Button::A;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use std::borrow::Borrow;
use crate::input::*;
use crate::input::input_handler::*;
use crate::input;
use crate::utilities::camera_utils;


/// This is the code for the current event loop.
/// So far the event loop contains the base SDL struct, an event pump, a window, and a game state object.
/// So far, it initialises all entities, and has the event loop render three triangles to the screen.

pub fn run() -> Result<(), Error> {

    // Initialise sdl
    let sdl = sdl2::init().unwrap();

    // Create the base window for the application.
    let mut window = windows_window::create_new(window_base!(), &sdl);

    let mut game_state = GameState::create_initial_state();

    // Get the event pump from sdl.
    let mut pump = sdl.event_pump().unwrap();
//    let mut pump = sdl.event_pump().unwrap();

    // Initialise the one time event queue.
    let mut one_time_events: VecDeque<Box<dyn FnMut()>> = VecDeque::new();

    // Initialise event queue for the game window.
    let mut one_time_window_events: VecDeque<Box<dyn FnMut(&mut WindowsWindow)>> = VecDeque::new();

    unsafe { gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32); }

    // Sets up the entities in the ECS.
    let m_camera = GameState::init_test_state(&mut game_state, &window)?;

    let now = Instant::now();

    let mut input_handler = InputHandler::new();

    // MAIN LOOP
    'running: loop {

        // Checks for sdl2 events. These are then filtered to appropriate areas to be processed properly.
        for event in pump.poll_iter(){
            // WINDOW EVENTS

            match event {

                // All window events are rerouted toward the active window.
                sdl2::event::Event::Window { timestamp : _ , window_id : _, win_event }
                => windows_window::process_event(&win_event, &mut WindowEvent { window: &mut window, events: &mut one_time_window_events }),

                // Breaks the loop.
                sdl2::event::Event::Quit { .. }=> { break 'running },

                sdl2::event::Event::MouseButtonUp {timestamp: _, window_id: _, which: _ , mouse_btn: MouseButton::Left, .. }
                    => { println!("left click released") },

                // TODO
                _ => ()
            }
        }

        // KEYBOARD INPUT MODULE - NEEDS WORK

        input_handler.update_input_state(&mut pump);

        // MOUSE INPUT MODULE - NEEDS WORK

        if input_handler.get_mouse_down(&MouseInput::Left) {

            let mouse_coordinates = input::get_mouse_coordinates(&pump);

            let screen_coordinates = camera_utils::ortho_screen_to_world_coordinates(
                &game_state.get::<OrthographicCameraComponent>(&m_camera).unwrap(),
                mouse_coordinates);

            if input_handler.get_mouse_button(&MouseInput::Left) {

                check_mouse_collision_system::CheckBoxColliderSystem::run((&mut game_state, &screen_coordinates));

            } else {

                selection_system::FollowMouseSystem::run((&mut game_state, &screen_coordinates));
            }
        }
        
        // Cycles through all events stored in this queue and executes them.
        while let Some(mut e) = one_time_events.pop_front() {
            e();
        }

        // Same as above, but processes window events specifically.
        while let Some(mut e) = one_time_window_events.pop_front() {
            e(&mut window);
        }

        // SYSTEMS
        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT);

            texture_update_system::TextureUpdateSystem::run(&mut game_state)?;

            selection_system::SelectionSystem::run((&mut game_state, &input_handler));

            position_update_system::PositionUpdateSystem::run(&mut game_state)?;

            render_system::RenderSystem::run(
                (game_state.get_map::<RenderComponent>(),
                         game_state.get_map::<PositionComponent>(),
                         game_state.get_map::<ColorComponent>(),
                         game_state.get_map::<TextureMixComponent>(),
                         game_state.get_map::<ScaleComponent>(),
                         game_state.get::<OrthographicCameraComponent>(&m_camera).unwrap()))?;
        }
        // End of rendering code.
        window.on_update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }

    unsafe {
        // Unbind vertex array.
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Ok(())
}









