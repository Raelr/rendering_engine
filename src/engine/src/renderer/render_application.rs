// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::video::Window;
use sdl2::Sdl;
use crate::renderer::shaders::shader_program::ShaderProgram;

pub struct RendererInformation {
    pub sdl: Sdl,
    pub window: Window,
    pub video: sdl2::VideoSubsystem,
    pub context : sdl2::video::GLContext
}

impl RendererInformation {
    fn from(sdl: Sdl, window: Window, video: sdl2::VideoSubsystem, context : sdl2::video::GLContext ) -> Result<RendererInformation, Error> {
        let renderer = RendererInformation {
            sdl,
            window,
            video,
            context
        };

        Ok(renderer)
    }
}

pub fn initialise() -> Result<RendererInformation, Error> {

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
        window("Game", 900, 900)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create gl context AFTER window is created.
    let gl_context = window.gl_create_context().unwrap();

    // Initialise gl.
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as * const std::os::raw::c_void);

    Ok(RendererInformation::from(sdl, window, video_subsystem, gl_context)?)
}

pub fn generate_n_buffers(amount: i32, buffers: Vec<&mut u32>) {
    unsafe {
        for buffer in buffers {
            gl::GenBuffers(amount, buffer);
        }
    }
}

pub fn generate_buffer_data<T>(buffer_type: gl::types::GLenum, buffer : &u32, vertices : &Vec<T>) {

    unsafe {

        gl::BindBuffer(buffer_type, *buffer);

        // Now that we've bound ARRAY_BUFFER to our vertex_buffer, we need to copy the vertices we
        // specified before INTO the buffer we created:
        gl::BufferData(
            buffer_type, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // Specifies the object does not change. If it did change,
            // the call would be DYNAMIC_DRAW or STREAM_DRAW,
            // which would place the data in an easy to access location
        );
    }
}

pub fn generate_vertex_array(location : u32, components : i32,
                             stride : usize, offset : usize) {
    unsafe {

        let offset = if offset == 0 {
            std::ptr::null()
        } else {
            (offset * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        };

        // Specifies how data stored in the vertex buffer is to be interpreted.
        gl::VertexAttribPointer(
            location, // index of the generic vertex attribute ("layout (location = 0)")
            components, // the number of components per generic vertex attribute. since its a vec3 the size is 3
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            offset // offset of the first component
        );

        gl::EnableVertexAttribArray(location); // this is "layout (location = 0)" in vertex shader
    }
}

pub fn draw(vertex_arrays: &Vec<u32>, stride: i32, is_element: bool) {

    unsafe {

        for vao in vertex_arrays {
            // Binds the vertex array
            gl::BindVertexArray(*vao);

            if is_element {
                gl::DrawElements(gl::TRIANGLES, stride, gl::UNSIGNED_INT, std::ptr::null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, stride);
            }
        }

        // gl::BindVertexArray(vertex_arrays[0 as usize]);
        // Draws count vertices in the vertex buffer or VAO.
        gl::BindVertexArray(0);
    }
}