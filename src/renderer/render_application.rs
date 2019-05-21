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

    pub sdl : Sdl,
    pub window : Window,
    pub video : sdl2::VideoSubsystem
}

impl RendererInformation {

    fn from(sdl : Sdl, window : Window, video : sdl2::VideoSubsystem) -> Result<RendererInformation, Error> {

        let renderer = RendererInformation {
            sdl,
            window,
            video
        };

        Ok(renderer)
    }
}

pub fn initialise() -> Result<RendererInformation, Error>{

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
        window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // -------------------------------------------------------------------------------------------
    // End of Window creation

    Ok(RendererInformation::from(sdl, window, video_subsystem)?)
}

pub fn generate_n_buffers(amount : i32, buffers : Vec<&mut u32>) {

    unsafe {
        for mut buffer in buffers {
            gl::GenBuffers(amount, buffer);
        }
    }
}