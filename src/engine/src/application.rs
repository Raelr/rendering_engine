use crate::renderer::render_application::RendererInformation;
use crate::renderer::render_application;
use crate::renderer::renderer_tests;
use failure::Error;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::events::mouse_changed::MouseChangedEvent;
use crate::events::event::EventTrait;
use crate::events::mouse_button_event::MouseButtonEvent;
use crate::events::application_events::{WindowResizeEvent, BaseWindowEvent};

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

    pub fn run(&self, vertex_array_objects : Vec<u32>, shader_program : ShaderProgram, stride: i32, is_element : bool) {

        // Event pump which stores all events and allows them to be processed.
        let mut event_pump = self.renderer.sdl.event_pump().unwrap();

//        unsafe {
//            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
//        }

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

                render_application::draw(&vertex_array_objects, stride, is_element);

                // Updates the window.
                self.renderer.window.gl_swap_window();
            }
        }
    }

    pub fn test_render(&self, test_type : renderer_tests::TestType) -> Result<(), Error>{

        // creates a shader project which combines a vertex and fragment shader.
        let shader_program = renderer_tests::basic_program()?;

        let vertex_array_objects = match test_type {
            renderer_tests::TestType::RectangleElement => renderer_tests::render_basic_square_with_elements()?,
            renderer_tests::TestType::TwoTrianglesSingleVertex => renderer_tests::render_from_vertex_array()?,
            renderer_tests::TestType::TwoTrianglesTwoVertices => renderer_tests::render_from_separate_arrays()?,
            renderer_tests::TestType::UpperCaseA => renderer_tests::draw_uppercase_a()?,
        };

        let stride = match test_type {
            renderer_tests::TestType::RectangleElement => 6,
            renderer_tests::TestType::TwoTrianglesSingleVertex => 6,
            renderer_tests::TestType::TwoTrianglesTwoVertices => 3,
            renderer_tests::TestType::UpperCaseA => 21,
        };

        let is_element : bool = match test_type {
            renderer_tests::TestType::RectangleElement | renderer_tests::TestType::UpperCaseA => true,
            _ => false
        };

        unsafe {
            // Set the viewport for the image.
            gl::Viewport(0, 0, 900, 900); // Set viewport.
            // Set the color of the window.
            gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set window color.
        }

//        let event : BaseWindowEvent = app_render!();
//
//        println!("{}", event.to_string());

        self.run(vertex_array_objects, shader_program, stride, is_element);

        Ok(())
    }
}

