extern crate sdl2;
extern crate gl;
extern crate failure;

pub struct OpenGLContext {
    gl_context : sdl2::video::GLContext,
}

impl OpenGLContext {

    pub fn new(window : &mut sdl2::video::Window, video : &mut sdl2::VideoSubsystem) -> OpenGLContext{

        // Create gl context AFTER window is created.
        let gl_context = window.gl_create_context().unwrap();

        // Initialise gl.
        let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as * const std::os::raw::c_void);

        OpenGLContext { gl_context }
    }

    pub fn swap_buffers(&mut self, window : &mut sdl2::video::Window) {
        window.gl_swap_window();
    }
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


