// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Mods
mod renderer;

// Use
use failure::Error;
use std::ffi::{CString};

fn main() -> Result<(),Error>{

    // initialises a struct which in turn stores all sdl and gl information
    let renderer = renderer::render_application::initialise()?;

    // Create gl context AFTER window is created.
    let _gl_context = renderer.window.gl_create_context().unwrap();

    // Initialise gl.
    let _gl = gl::load_with(|s| renderer.video.gl_get_proc_address(s) as * const std::os::raw::c_void);

    // creates a shader project which combines a vertex and fragment shader.
    let shader_program = renderer::renderer_tests::basic_program()?;

    // returns a struct which stores both index and vertex information.
    let shape = renderer::renderer_tests::create_triangle_quad()?;

    // Creates a vertex buffer in the GPU. the uint is an unique id which allows quick access to the
    // buffer.
    let mut vertex_buffer : gl::types::GLuint = 0;

    let mut element_buffer_object : gl::types::GLuint = 0;

    let mut vertex_array_object: gl::types::GLuint = 0;

    // Generates buffers for all buffer objects.
    renderer::render_application::generate_n_buffers(
        1, vec![&mut vertex_buffer, &mut element_buffer_object]);

    unsafe {
        // Generates a vertex array object (VAO) and returns ints ID.
        gl::GenVertexArrays(1, &mut vertex_array_object);

        // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in
        // the VAO
        gl::BindVertexArray(vertex_array_object);

        // Binds the created buffer to a specific type (in this case we specify that this is an
        // array buffer)

        renderer::render_application::generate_buffer_data(gl::ARRAY_BUFFER,
                                                           &vertex_buffer, &shape.vertices);

        renderer::render_application::generate_buffer_data(gl::ELEMENT_ARRAY_BUFFER,
                                                           &element_buffer_object,
                                                           &shape.indices);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

        // Specifies how data stored in the vertex buffer is to be interpreted.
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute. since its a vec3 the size is 3
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // Accesses the vertex attribute stored in location 1.

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // This time the stride is 6
                                                                  // (vec3 position, vec3 color)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Change color of the window. Calls an unsafe function from gl library.
    unsafe {
        gl::Viewport(0, 0, 900, 900); // Set viewport.
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set window color.
    }

    // Event pump which stores all events and allows them to be processed.
    let mut event_pump = renderer.sdl.event_pump().unwrap();

    // The main event loop which keeps the window open.
    'main: loop {

        // Looks for events and acts acording to which ones are recieved.
        for event in event_pump.poll_iter() {
            match event {
                // Quit event
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => ()
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader_program.set_used();

            unsafe {

                // Binds the vertex array
                gl::BindVertexArray(vertex_array_object);

                // Draws count vertices in the vertex buffer or VAO.
                gl::DrawElements(
                    gl::TRIANGLES, // mode
                    6, // starting index in the enabled arrays
                    gl::UNSIGNED_INT,
                    std::ptr::null()
                );
            }
            // Updates the window.
            renderer.window.gl_swap_window();
        }
    }
    Ok(())
}