// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::Sdl;
use crate::platform::windows::windows_window;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::window::{WindowProperties, WindowTrait};
use crate::platform::windows::windows_window::{WindowsWindow, process_event};
use std::collections::VecDeque;
use crate::generational_index::generational_index::*;
use crate::events::window_event::WindowEvent;
use crate::renderer::render_application;
use crate::renderer::renderer_tests::{basic_program, fade_program};
use crate::renderer::renderer_component::TriangleRenderComponent;
use crate::renderer::shaders::shader::Shader;
use std::time::{Duration, Instant};
use std::os::raw::c_float;
use std::ffi::{CStr, CString};

/// GameState object stores all entities and components within itself. If handles the streaming of
/// components into different systems.

pub struct GameState {}

/// should store all components and entity IDs when actual gameobjects and players are added to the game.
/// TODO: populate GameState with relevant variables.

impl GameState {
    pub fn create_initial_state() -> GameState {
        let state = GameState {};

        state
    }
}

/// The base application struct for the engine.

pub struct ScrapYardApplication {
    pub game_state: GameState,
    pub update_void_events: Vec<Box<FnMut(&mut GameState)>>,
}

/// Constructor and registration methods. Might need to remove the update events (since they don't seem to do anything right now)

impl ScrapYardApplication {
    pub fn new() -> ScrapYardApplication {
        let mut app = ScrapYardApplication {
            game_state: GameState::create_initial_state(),
            update_void_events: Vec::new(),
        };

        app
    }

    pub fn register_game_update_event(&mut self, event: Box<dyn FnMut(&mut GameState)>) {
        &self.update_void_events.push(event);
    }
}

/// This is the code for the current event loop.
/// The event loop controls the basic data flow of the engine.
/// Currently, it contains the window, a reference to the main application struct, and all the SDL details.
/// There are a couple of details which i'm not sure about - specifically relating to how the data should be organised.
/// Mainly, I'm unsure whether the window should handle all sdl related events or just events relating to it.
/// Currently I have the event pump in the main loop, the match statement would, in theory, redirect the events toward the
/// correct module.

pub fn run() -> Result<(), Error> {

    // Initialise sdl
    let sdl = sdl2::init().unwrap();

    // Create the base window for the application.
    let mut window = windows_window::create_new(window_base!(), &sdl);

    // Create the base application.
    let mut app = ScrapYardApplication::new();

    // Get the event pump from sdl.
    let mut pump = sdl.event_pump().unwrap();

    // Initialise the one time event queue.
    let mut one_time_events: VecDeque<Box<dyn FnMut()>> = VecDeque::new();

    // Initialise event queue for the game window.
    let mut one_time_window_events: VecDeque<Box<dyn FnMut(&mut WindowsWindow)>> = VecDeque::new();

    // Create a list of triangle render objects.
    let mut triangle_objects: Vec<TriangleRenderComponent> = Vec::new();

    triangle_objects.push(TriangleRenderComponent { shader_program: triangle_render!() });

    triangle_objects.push(TriangleRenderComponent { shader_program: triangle_render!() });

    triangle_objects.push(TriangleRenderComponent { shader_program: triangle_fade!() });

    /// Rendering code. For now this will stay here. Need to find a suitable home for it once i've gotten a hang of rendering.
    /// TODO: Move the rendering code to a different struct (probably a renderer component).

    let vertices: Vec<f32> = vec![

         // positions     // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
         -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0,
    ];

    let mut vertex_buffer_object: gl::types::GLuint = 0;

    let mut vertex_array_object: gl::types::GLuint = 0;

    render_application::generate_n_buffers(1, vec![&mut vertex_buffer_object]);

    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);

        // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in,
        // the VAO
        gl::BindVertexArray(vertex_array_object);

        // Binds the created buffer to a specific type (in this case we specify that this is an
        // array buffer)
        render_application::generate_buffer_data(gl::ARRAY_BUFFER,
                                                 &vertex_buffer_object, &vertices);

        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 6, 0);

        render_application::generate_vertex_array(1, 3, 6, 3);

        gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32);

        // Resets the bindings on the GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);
    }

    let now = Instant::now();

    // Main loop of the game engine.
    loop {

        // Checks for sdl2 events. These are then filtered to appropriate areas to be processed properly.
        for event in pump.poll_iter() {
            match event {
                // All window events are rerouted toward the active window.
                sdl2::event::Event::Window { timestamp, window_id, win_event }
                => windows_window::process_event(&win_event, &mut WindowEvent { window: &mut window, events: &mut one_time_window_events }),

                // TODO
                sdl2::event::Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y }
                => println!("MAIN LOOP: Mouse Clicked: {},{}, {}", x, y, window_id),

                // TODO
                sdl2::event::Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel }
                => println!("MAIN LOOP: Mouse Moved: {},{}", x, y),

                // TODO
                sdl2::event::Event::KeyDown { keycode, repeat, .. }
                => println!("MAIN LOOP: Key pressed: {} repeating: {}", keycode.unwrap(), repeat),

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

        /// Continuation of rendering code.

        unsafe {

            gl::BindVertexArray(vertex_array_object);

            gl::Clear(gl::COLOR_BUFFER_BIT);

            /// This is the code needed to render something AT THE VERY LEAST.

            // FIRST TRIANGLE

            triangle_objects[0].shader_program.set_used();

            gl::Uniform2f(gl::GetUniformLocation(triangle_objects[0].shader_program.id(), CString::new("Offset")?.as_ptr()), 0.5, 0.0);

            gl::Uniform1ui(gl::GetUniformLocation(triangle_objects[0].shader_program.id(), CString::new("UsePosition")?.as_ptr()), 1);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // SECOND TRIANGLE

            triangle_objects[1].shader_program.set_used();

            gl::Uniform2f(gl::GetUniformLocation(triangle_objects[1].shader_program.id(), CString::new("Offset")?.as_ptr()), -0.5, 0.0);

            gl::Uniform1ui(gl::GetUniformLocation(triangle_objects[1].shader_program.id(), CString::new("UsePosition")?.as_ptr()), 0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // THIRD TRIANGLE

            triangle_objects[2].shader_program.set_used();

            let color_location = gl::GetUniformLocation(triangle_objects[2].shader_program.id(), CString::new("ourColor")?.as_ptr());

            gl::Uniform4f(color_location, 0.0, (f32::sin( now.elapsed().as_secs_f64() as f32) / 2.0 + 0.5), 0.0, 1.0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        /// End of rendering code.

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









