// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use crate::platform::windows::windows_window;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::window::{WindowProperties, WindowTrait};
use crate::platform::windows::windows_window::{WindowsWindow};
use std::collections::VecDeque;
use crate::events::window_event::WindowEvent;
use crate::renderer::render_application;
use crate::renderer::renderer_component::{RenderComponent};
use crate::renderer::shaders::shader::Shader;
use std::time::{Duration, Instant};
use std::ffi::{CString};
use crate::game_state::GameState;
use crate::components::{PositionComponent, ColorComponent, TimerComponent};
use crate::renderer::renderer_systems::RendererTestSystem;


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

    let mut game_state = GameState::create_initial_state();

    // Get the event pump from sdl.
    let mut pump = sdl.event_pump().unwrap();

    // Initialise the one time event queue.
    let mut one_time_events: VecDeque<Box<dyn FnMut()>> = VecDeque::new();

    // Initialise event queue for the game window.
    let mut one_time_window_events: VecDeque<Box<dyn FnMut(&mut WindowsWindow)>> = VecDeque::new();

    let first_comp = game_state.create_entity();

    let second_comp = game_state.create_entity();

    let third_comp = game_state.create_entity();

    // RIGHT

    game_state.register_renderer(&first_comp, RenderComponent { shader_program: triangle_render!() });

    game_state.register_position(&first_comp, PositionComponent { position : (0.5, 0.0, 0.0), reversed : false });

    game_state.register_color(&first_comp, ColorComponent { color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : false, use_position : true});

    game_state.register_entity(first_comp);

    // LEFT

    game_state.register_renderer(&second_comp, RenderComponent { shader_program: triangle_render!() });

    game_state.register_position(&second_comp, PositionComponent { position : (-0.5, 0.0, 0.0), reversed : false });

    game_state.register_color(&second_comp, ColorComponent { color : (0.0, 0.0, 0.0, 0.0), use_vertex_colors : true, use_position : false});

    game_state.register_entity(second_comp);

    // CENTER

    game_state.register_renderer(&third_comp, RenderComponent { shader_program: triangle_render!() });

    game_state.register_position(&third_comp, PositionComponent { position : (0.0, 0.0, 0.0), reversed : true });

    game_state.register_color(&third_comp, ColorComponent { color : (0.0, 1.0, 0.0, 0.0), use_vertex_colors : false, use_position : false});

    game_state.register_timer(&third_comp, TimerComponent {now : Instant::now()});

    game_state.register_entity(third_comp);

    // Rendering code. For now this will stay here. Need to find a suitable home for it once i've gotten a hang of rendering.
    // TODO: Move the rendering code to a different struct (probably a renderer component).

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

    let mut render_system = RendererTestSystem;

    let now = Instant::now();

    // Main loop of the game engine.
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

        // DRAW CODE
        unsafe {

            gl::BindVertexArray(vertex_array_object);

            gl::Clear(gl::COLOR_BUFFER_BIT);

            render_system.render_positions(&mut game_state.render_components, &mut game_state.position_components, &game_state.entities);

            render_system.render_colors(&mut game_state.render_components, &mut game_state.color_components,
                                        &mut game_state.timer_components,  &game_state.entities);

            render_system.draw_triangles(&mut game_state.render_components);
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









