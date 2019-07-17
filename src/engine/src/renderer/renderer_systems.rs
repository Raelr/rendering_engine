use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use crate::components::{PositionComponent, ColorComponent, TimerComponent, RenderComponent};
use failure::Error;
use crate::platform::open_gl::*;
use crate::platform::windows::windows_window::WindowsWindow;

extern crate gl;

pub struct RendererTestSystem;

impl RendererTestSystem {

    pub fn render_positions(renderers : &GenerationalIndexArray<RenderComponent>,
                              positions : &GenerationalIndexArray<PositionComponent>) -> Result<(), Error> {

        for index in 0..renderers.entries.len() {

            if let Some(r) = &renderers.entries[index] {

                let renderer = &r.value.shader_program;

                renderer.set_used();

                let position = positions.get(&GenerationalIndex { index, generation : r.generation});

                if let Some(p) = position {

                    unsafe {

                        renderer.set_vector2("Offset", (p.position.0, p.position.1))?;
                        renderer.set_bool(p.reversed, "ReverseShape", )?;

                    }
                }
            }
        }

        Ok(())
    }

    pub fn render_colors(color : &GenerationalIndexArray<ColorComponent>, renderers : &GenerationalIndexArray<RenderComponent>,
                         timer :  &GenerationalIndexArray<TimerComponent>) -> Result<(), Error> {

        let mut count = 0;

        for index in 0..color.entries.len() {

            if let Some(c) = &color.entries[index] {

                let component = &c.value;

                if let Some(r) = renderers.get(&GenerationalIndex {index, generation : c.generation}) {

                    r.shader_program.set_used();

                    unsafe {
                        r.shader_program.set_bool(component.use_position, "UsePosition")?;
                        r.shader_program.set_bool(component.use_vertex_colors, "UseVertexColors")?;

                        let color = {

                            if let Some(t) = timer.get(&GenerationalIndex {index, generation : c.generation}) {

                                (0.0, (f32::sin( t.now.elapsed().as_secs_f64() as f32)  + 1.0 / 2.0), 0.0, 1.0)

                            } else {

                                component.color
                            }
                        };

                        r.shader_program.set_vector4( "VertexColor", color)?;
                    }
                }
            }
            count += 1;
        }
        Ok(())
    }

    pub fn draw_triangles(renderers : &GenerationalIndexArray<RenderComponent>) {

        for renderer in &renderers.entries {

            if let Some(e) = renderer {

                e.value.shader_program.set_used();

                unsafe {

                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    //println!("{}", count);
                }
            }
        }
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