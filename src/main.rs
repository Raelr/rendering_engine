// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Mods
pub mod render_gl;

// Use
use failure::Error;
use gl::*;
use std::ffi::{CString, CStr};

fn main() -> Result<(),Error>{

    // Initialise sdl to allow for window spawning.
    let sdl = sdl2::init().unwrap();

    // Creates the video subsystem which internally contains a clone of sdl.
    let video_subsystem = sdl.video().unwrap();

    // Specify which version of OpenGL we'll be using.
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    // Initialises a new window and allows the input of arguments and parameters into the window.
    let window = video_subsystem.
        window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create gl context AFTER window is created.
    let gl_context = window.gl_create_context().unwrap();

    // Initialise gl.
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as * const std::os::raw::c_void);

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    // Specify an array of vertices (positioned as x, y, z coordinates)
    // This array forms a triangle.
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0];

    // Creates a vertex buffer in the GPU. the uint is an unique id which allows quick access to the
    // buffer.
    let mut vertex_buffer : gl::types::GLuint = 0;

    unsafe {
        // Creates a n buffers and binds them to the vertex buffer's id (set above).
        gl::GenBuffers(1, &mut vertex_buffer);
    }

    unsafe {
        // Binds the created buffer to a specific type (in this case we specify that this is an
        // array buffer)
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

        // Now that we've bound ARRAY_BUFFER to our vertex_buffer, we need to copy the vertices we
        // specified before INTO the buffer we created:
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // Specifies the object does not change. If it did change,
                             // the call would be DYNAMIC_DRAW or STREAM_DRAW, which would
                             // place the data in an easy to access location
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer for reuse. The 0 indicates that
                                             // the binding is being reset.
    }

    let mut vertex_array_object: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_object);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Change color of the window. Calls an unsafe function from gl library.
    unsafe {
        gl::Viewport(0,0, 900, 700); // Set viewport.
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set window color.
    }

    // Event pump which stores all events and allows them to be processed.
    let mut event_pump = sdl.event_pump().unwrap();

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

                gl::BindVertexArray(vertex_array_object);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // starting index in the enabled arrays
                    3 // number of indices to be rendered
                );
            }

            // Updates the window.
            window.gl_swap_window();
        }
    }
    Ok(())
}