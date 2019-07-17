use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex, ArrayEntry};
use crate::components::{PositionComponent, ColorComponent, TimerComponent, RenderComponent};
use failure::Error;
use crate::platform::open_gl::*;
use crate::platform::windows::windows_window::WindowsWindow;

extern crate gl;

pub struct RendererTestSystem;

impl RendererTestSystem {

    pub fn render_positions(renderers : &GenerationalIndexArray<RenderComponent>,
                              positions : &GenerationalIndexArray<PositionComponent>) -> Result<(), Error> {
        let mut idx = 0;
        renderers.entries.iter().try_for_each( |renderer| -> Result<(), Error>{
            if let Some(renderer) = renderer {
                renderer.value.shader_program.set_used();
                if let Some(position) = positions.get(&GenerationalIndex { index : idx, generation : renderer.generation}) {
                    unsafe {
                        renderer.value.shader_program.set_vector2("Offset", (position.position.0, position.position.1))?;
                        renderer.value.shader_program.set_bool(position.reversed, "ReverseShape", )?;
                    }
                } idx+= 1;
            } Ok(()) })?;
        Ok(())
    }

    pub fn render_colors(color : &GenerationalIndexArray<ColorComponent>, renderers : &GenerationalIndexArray<RenderComponent>,
                         timer :  &GenerationalIndexArray<TimerComponent>) -> Result<(), Error> {

        let mut idx = 0;

        color.entries.iter().try_for_each(|component : &Option<ArrayEntry<ColorComponent>>| -> Result<(), Error> {
            if let Some(entry) = component.as_ref() {
                let value = &entry.value;
                let generational_idx = &GenerationalIndex {index : idx, generation : entry.generation};
                if let Some(renderer) = renderers.get(generational_idx) {
                    renderer.shader_program.set_used();
                    unsafe {
                        renderer.shader_program.set_bool(value.use_position, "UsePosition")?;
                        renderer.shader_program.set_bool(value.use_vertex_colors, "UseVertexColors")?;
                        let color =  {
                            let mut input_color = value.color;
                            if !timer.entries.is_empty() {
                                if let Some(color_timer) = timer.get(generational_idx) {
                                    input_color = (0.0, (f32::sin( color_timer.now.elapsed().as_secs_f64() as f32)  + 1.0 / 2.0), 0.0, 1.0)
                                }
                            }
                            input_color
                        };
                        renderer.shader_program.set_vector4( "VertexColor", color)?; idx += 1;
                    }
                }
            } Ok(())
        })?;
        Ok(())
    }

    pub fn draw_triangles(renderers : &GenerationalIndexArray<RenderComponent>) {

        renderers.entries.iter().for_each(|renderer| {
            if let Some(renderer) = renderer {
                renderer.value.shader_program.set_used();
                unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }
            }
        });
    }

    pub fn init_shapes(window : &WindowsWindow) {

        let vertices: Vec<f32> = vec![

            // positions     // colors
            0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
            0.0,  0.5, 0.0,  0.0, 0.0, 1.0,
        ];

        let mut vertex_buffer_object: gl::types::GLuint = 0;

        let mut vertex_array_object: gl::types::GLuint = 0;

        generate_n_buffers(1, vec![&mut vertex_buffer_object]);

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array_object);

            // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in,
            // the VAO
            gl::BindVertexArray(vertex_array_object);

            // Binds the created buffer to a specific type (in this case we specify that this is an
            // array buffer)
            generate_buffer_data(gl::ARRAY_BUFFER,
                                                     &vertex_buffer_object, &vertices);

            // Creates a vertex attribute pointer and enables it on the GPU
            generate_vertex_array(0, 3, 6, 0);

            generate_vertex_array(1, 3, 6, 3);

            gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32);

            // Resets the bindings on the GPU
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            gl::BindVertexArray(vertex_array_object);
        }
    }
}