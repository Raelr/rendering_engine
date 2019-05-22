use crate::renderer::render_application::RendererInformation;
use crate::renderer::render_application;
use crate::renderer::renderer_tests;
use failure::Error;
use crate::renderer::shaders::shader_program::ShaderProgram;

pub struct Application {

    pub renderer : RendererInformation,
}

impl Application {

    pub fn initialise_with_renderer() -> Result<Application, Error> {

        let application = Application {
            renderer : render_application::initialise()?
        };

        Ok(application)
    }

    pub fn run(&self, vertex_array_object : u32, shader_program : ShaderProgram) {

        // Event pump which stores all events and allows them to be processed.
        let mut event_pump = self.renderer.sdl.event_pump().unwrap();

        // The main event loop which keeps the window open.
        'main: loop {

            // Looks for events and acts according to which ones are recieved.
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

                    //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

                    // Draws count vertices in the vertex buffer or VAO.
                    gl::DrawElements(gl::TRIANGLES, // mode
                                     6,             // starting index in the enabled arrays
                                     gl::UNSIGNED_INT,
                                     std::ptr::null()
                    );

                    gl::DrawArrays(
                        gl::TRIANGLES,
                        0,
                        6
                    )
                }
                // Updates the window.
                self.renderer.window.gl_swap_window();
            }
        }
    }

    pub fn test_render(&self) -> Result<(), Error>{

        // creates a shader project which combines a vertex and fragment shader.
        let shader_program = renderer_tests::basic_program()?;

        let vertex_array_object = renderer_tests::render_from_vertex_array(&self.renderer)?;

        self.run(vertex_array_object, shader_program);

        Ok(())
    }
}

