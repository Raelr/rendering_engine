// Crates
extern crate gl;
extern crate failure;

// Internal crates:
use crate::ecs::{PositionComponent, ColorComponent, RenderComponent, TextureMixComponent};
use crate::ecs::*;
use crate::ecs::system::System;
use crate::events::window_event::WindowEvent;
use crate::game_state::GameState;
use crate::platform::windows::windows_window;
use crate::window::{WindowProperties, WindowTrait};
use crate::platform::windows::windows_window::{WindowsWindow};
use crate::sdl2::mouse::MouseButton;
use crate::input::{MouseInput, KeyCode};
use crate::nalgebra::{Vector3, Vector2};
use crate::utilities::vector_utils::*;

// Use
use failure::Error;
use std::collections::VecDeque;
use std::time::{Duration};
use crate::input::input_handler::*;
use crate::input;
use crate::utilities::camera_utils;
use crate::ecs::look_at_position_system::{LookAtPositionSystem, UpdateFocusPointSystem};


/// This is the code for the current event loop.
/// So far the event loop contains the base SDL struct, an event pump, a window, and a game state object.
/// So far, it initialises all entities, and has the event loop render three triangles to the screen.

pub fn run() -> Result<(), Error> {

    // Initialise sdl
    let sdl = sdl2::init().unwrap();

    // Create the base window for the application.
    let mut window = windows_window::create_new(window_base!(), &sdl);

    // Initialises the game state.
    let mut game_state = GameState::create_initial_state();

    // Get the event pump from sdl.
    let mut pump = sdl.event_pump().unwrap();

    // Initialise the one time event queue.
    let mut one_time_events: VecDeque<Box<dyn FnMut()>> = VecDeque::new();

    // Initialise event queue for the game window.
    let mut one_time_window_events: VecDeque<Box<dyn FnMut(&mut WindowsWindow)>> = VecDeque::new();

    unsafe { gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32); }

    // Sets up the entities in the ECS.
    let m_camera = GameState::init_test_state(&mut game_state, &window)?;

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

                sdl2::event::Event::MouseButtonUp {timestamp: _, window_id: _, which: _ , mouse_btn: button, .. }
                    => { input_handler.clear_mouse_input(&input::sdl_mouse_to_mouse(&button))},

                sdl2::event::Event::KeyUp { timestamp: _, window_id: _ , keycode: code, scancode: scancode, .. }
                    => { println!("Key Released: {}", code.unwrap()); input_handler.clear_keyboard_input(&input::scancode_to_keycode(&scancode.unwrap()))}

                // TODO
                _ => ()
            }
        }

        // KEYBOARD INPUT MODULE - NEEDS WORK

        input_handler.update_input_state(&mut pump);

        // MOUSE INPUT MODULE - NEEDS WORK

        // LEFT CLICK
        if input_handler.get_mouse_down(&MouseInput::LeftMouse) {

            let mouse_coordinates = input::get_mouse_coordinates(&pump);

            // TODO: UPDATE ORTHOGRAPHIC CAMERA WHEN SCREEN IS RESIZED.
            let screen_coordinates = camera_utils::ortho_screen_to_world_coordinates(
                &game_state.get::<OrthographicCameraComponent>(&m_camera).unwrap(),
                mouse_coordinates);

            // CHECK IF MOUSE IS HELD DOWN

            if input_handler.get_mouse_button(&MouseInput::LeftMouse) {

                check_mouse_collision_system::CheckBoxColliderSystem::run((&mut game_state, &screen_coordinates))?;

            } else {

                selection_system::FollowMouseSystem::run((&mut game_state, &screen_coordinates))?;
            }
        }

        // RIGHT CLICK
        if input_handler.get_mouse_down(&MouseInput::RightMouse) {

            let mouse_coordinates = input::get_mouse_coordinates(&pump);

             let screen_coords = camera_utils::ortho_screen_to_world_coordinates(
                &game_state.get::<OrthographicCameraComponent>(&m_camera).unwrap(),
                mouse_coordinates);

            if input_handler.get_mouse_button(&MouseInput::RightMouse) {

                let scale = Vector3::new(100.0, 100.0, 0.0);
                let position = Vector3::new(screen_coords.x, screen_coords.y, 0.0);

                selection_system::DeselectSystem::run(&mut game_state);

                let entity = GameState::create_entity(&mut game_state)
                    .with(RenderComponent {shader_program : triangle_render!(), vertex_array_object : quad!()})
                    .with(PositionComponent {position})
                    .with(ScaleComponent {scale})
                    .with(ColorComponent {color : (0.0, 0.0, 0.0, 0.0) })
                    .with(VelocityComponent {velocity : Vector3::new(0.0, 0.0, 0.0)})
                    .with(BoxCollider2DComponent {position: Vector2::new(position.x, position.y), size : Vector2::new(scale.x * 2.0, scale.y * 2.0)})
                    .with(RotationComponent { rotation: Vector3::new(0.0, 0.0, 0.0) })
                    .with(RotationUpdateComponent { axis: Vector3::new(0.0, 0.0, 1.0), angle: get_rotation_angle_2(Vector2::new(screen_coords.x, screen_coords.y), screen_coords) })
                    .with(LookAtPositionComponent{ focus_point: screen_coords})
                    .with(SelectedComponent {
                        selected_color: (0.5, 0.5, 0.5, 0.5),
                        origin_color: (0.0, 0.0, 0.0, 0.0),
                        cursor_offset: Vector2::new(0.0, 0.0)
                    })
                    .build();
            }

            UpdateFocusPointSystem::run((&mut game_state, screen_coords));
            LookAtPositionSystem::run((&mut game_state));
        }

        if input_handler.get_keycode(&KeyCode::Space) {

            let position = Vector3::new(0.0, 0.0, 0.0);
            let scale = Vector3::new(50.0, 50.0, 50.0);

            let entity = GameState::create_entity(&mut game_state)
                .with(RenderComponent {shader_program : triangle_render!(), vertex_array_object : quad!()})
                .with(PositionComponent {position})
                .with(RotationComponent { rotation: Vector3::new(0.0, 0.0, 0.0) })
                .with(ScaleComponent {scale})
                .with(ColorComponent {color : (1.0, 1.0, 1.0, 0.0) })
                .with(TextureMixComponent { textures : vec!
                [texture!("src/engine/src/renderer/textures/container.jpg",0, gl::TEXTURE0, String::from("Texture1")),
                 texture!("src/engine/src/renderer/textures/awesomeface.png",1, gl::TEXTURE1, String::from("Texture2"))],
                    opacity: 0.0})
                .with(TextureUpdateComponent {opacity_change : 0.0 })
                .with(VelocityComponent {velocity : Vector3::new(0.0, 0.0, 0.0)})
                .with(BoxCollider2DComponent {position: Vector2::new(position.x, position.y), size : Vector2::new(scale.x * 2.0, scale.y * 2.0)})
                .build();
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

            //println!("Texture");
            texture_update_system::TextureUpdateSystem::run(&mut game_state)?;

            //println!("Selection");
            selection_system::SelectionSystem::run((&mut game_state))?;

            //println!("Position");
            position_update_system::PositionUpdateSystem::run(&mut game_state)?;

            //println!("Render");
            render_system::RenderSystem::run(
                (game_state.get_map::<RenderComponent>(),
                         game_state.get_map::<PositionComponent>(),
                         game_state.get_map::<ColorComponent>(),
                         game_state.get_map::<TextureMixComponent>(),
                         game_state.get_map::<ScaleComponent>(),
                         game_state.get::<OrthographicCameraComponent>(&m_camera).unwrap(),
                         game_state.get_map::<RotationComponent>()))?;
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









