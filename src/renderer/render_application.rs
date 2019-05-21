// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::video;

pub struct RendererInformation {
    pub sdl: Sdl,
    pub window: Window,
    pub video: sdl2::VideoSubsystem,
}

impl RendererInformation {
    fn from(sdl: Sdl, window: Window, video: sdl2::VideoSubsystem) -> Result<RendererInformation, Error> {
        let renderer = RendererInformation {
            sdl,
            window,
            video,
        };

        Ok(renderer)
    }
}

pub fn initialise() -> Result<RendererInformation, Error> {

    // Section for creating a window:
    //_________________________________________________________________

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

    // -------------------------------------------------------------------------------------------
    // End of Window creation

    Ok(RendererInformation::from(sdl, window, video_subsystem)?)
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