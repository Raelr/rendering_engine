extern crate sdl2;
extern crate gl;

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

