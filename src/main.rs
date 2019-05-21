// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Mods
pub mod render_gl;

// Use
use failure::Error;
use std::ffi::{CString};

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
    let _gl_context = window.gl_create_context().unwrap();

    // Initialise gl.
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as * const std::os::raw::c_void);

    let shader_program = render_gl::Program::from_shaders(
        &[render_gl::Shader::from_vert_source( // Vert Shader source
            &CString::new(
                include_str!("triangle.vert")).unwrap()).unwrap(),
            render_gl::Shader::from_frag_source(&CString::new( // Frag shader source
                include_str!("triangle.frag")).unwrap()).unwrap()]).unwrap();

    shader_program.set_used();

    // Specify an array of vertices (positioned as x, y, z coordinates)
    // This array forms a triangle.
    let vertices: Vec<f32> = vec![
        // positions        // Colors
         0.5,  0.5, 0.0,    1.0, 0.0, 0.0, // bottom right
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0, // bottom left
        -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,  // top
        -0.5,  0.5, 0.0,    0.5, 0.2, 0.0
    ];

    // The drawing order of indices within the vertex array.
    let indices : Vec<gl::types::GLuint> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    // Creates a vertex buffer in the GPU. the uint is an unique id which allows quick access to the
    // buffer.
    let mut vertex_buffer : gl::types::GLuint = 0;

    let mut element_buffer_object : gl::types::GLuint = 0;

    unsafe {
        // Creates a n buffers and binds them to the vertex buffer's id (set above).
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::GenBuffers(1, &mut element_buffer_object);
    }

    let mut vertex_array_object: gl::types::GLuint = 0;

    unsafe {
        // Generates a vertex array object (VAO) and returns ints ID.
        gl::GenVertexArrays(1, &mut vertex_array_object);

        // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in
        // the VAO
        gl::BindVertexArray(vertex_array_object);

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

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);
        gl::BufferData (
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

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
            window.gl_swap_window();
        }
    }
    Ok(())
}