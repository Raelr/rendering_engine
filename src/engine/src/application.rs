// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use crate::platform::windows::windows_window;
use crate::window::{WindowProperties, WindowTrait};
use crate::platform::windows::windows_window::{WindowsWindow};
use std::collections::VecDeque;
use crate::events::window_event::WindowEvent;
use crate::game_state::GameState;
use std::time::{Duration, Instant};
use crate::ecs::{PositionComponent, ColorComponent, TimerComponent, RenderComponent, Texture, RenderComponentTemp, TextureMixComponent, TextureUpdateComponent};
use crate::renderer::shapes::shape::{Triangle, Shape, Quad};
use crate::ecs::systems::{RenderSystem, System, TextureUpdateSystem};
use crate::generational_index::generational_index::GenerationalIndex;


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

    // Initialise the one time event queue.
    let mut one_time_events: VecDeque<Box<dyn FnMut()>> = VecDeque::new();

    // Initialise event queue for the game window.
    let mut one_time_window_events: VecDeque<Box<dyn FnMut(&mut WindowsWindow)>> = VecDeque::new();

    // Sets up the entities in the ECS.
    GameState::init_test_state(&mut game_state)?;

    let render_system = RenderSystem;
    let texture_change = TextureUpdateSystem;

    unsafe { gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32); }

    let now = Instant::now();

    // MAIN LOOP
    'running: loop {

        // Checks for sdl2 events. These are then filtered to appropriate areas to be processed properly.
        for event in pump.poll_iter() {

            match event {

                // All window events are rerouted toward the active window.
                sdl2::event::Event::Window { timestamp : _ , window_id : _, win_event }
                => windows_window::process_event(&win_event, &mut WindowEvent { window: &mut window, events: &mut one_time_window_events }),

                // Breaks the loop.
                sdl2::event::Event::Quit { .. }=> { break 'running },

                // TODO
                sdl2::event::Event::MouseButtonDown { timestamp : _, window_id, which : _, mouse_btn : _, clicks : _, x, y }
                => println!("MAIN LOOP: Mouse Clicked: {},{}, {}", x, y, window_id),

                // TODO
                sdl2::event::Event::MouseMotion { timestamp : _, window_id : _, which : _, mousestate : _, x, y, xrel: _, yrel: _ }
                => println!("MAIN LOOP: Mouse Moved: {},{}", x, y),

                // TODO
                sdl2::event::Event::KeyDown { keycode, repeat, .. }
                => { let key_code = keycode.unwrap();
                    match key_code {

                      sdl2::keyboard::Keycode::Up => {
                          if let Some(update) = game_state.get_map_mut::<TextureUpdateComponent>().get_mut(&GenerationalIndex {index : 0, generation : 0}) {

                              update.opacity_change = 0.1;
                          }
                      }
                      sdl2::keyboard::Keycode::Down => {if let Some(update) = game_state.get_map_mut::<TextureUpdateComponent>().get_mut(&GenerationalIndex {index : 0, generation : 0}) {

                          update.opacity_change = -0.1;
                      }}
                        _ => ()
                    }
                    println!("MAIN LOOP: Key pressed: {} repeating: {}", keycode.unwrap(), repeat);},

                // TODO
                _ => ()
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

        // DRAW CODE
        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT);

            texture_change.run(&mut game_state)?;

            render_system.run(
                ( game_state.get_map::<RenderComponentTemp>(),
                        game_state.get_map::<PositionComponent>(),
                        game_state.get_map::<ColorComponent>(),
                        game_state.get_map::<TextureMixComponent>(),
                        &now.elapsed().as_secs_f32()))?;
        }

        // End of rendering code.
        window.on_update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    unsafe {
        // Unbind vertex array.
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Ok(())
}









