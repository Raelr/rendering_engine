use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use crate::renderer::renderer_component::RenderComponent;
use crate::components::{PositionComponent, ColorComponent, TimerComponent};
use crate::game_state::GameState;
use failure::Error;
extern crate gl;

pub struct RendererTestSystem;

impl RendererTestSystem {

    pub fn render_positions(&mut self, renderers : &mut GenerationalIndexArray<RenderComponent>,
                        positions : &mut GenerationalIndexArray<PositionComponent>, entities : &Vec<GenerationalIndex>) {

        for entity in entities {

            let renderer = renderers.get_mut(entity);

            if let Some(e) = renderer {

                e.shader_program.set_used();

                let position = positions.get(entity);

                if let Some(p) = position {

                    unsafe {
                        e.shader_program.set_vector2("Offset", (p.position.0, p.position.1));
                        e.shader_program.set_bool(p.reversed, "ReverseShape", );

                    }
                }
            }
        }
    }

    pub fn render_colors(&mut self, renderers : &mut GenerationalIndexArray<RenderComponent>,
                     color : &mut GenerationalIndexArray<ColorComponent>, timer :  &mut GenerationalIndexArray<TimerComponent>,
                     entities : &Vec<GenerationalIndex>) -> Result<(), Error> {

        for entity in entities {

            let renderer = renderers.get_mut(entity);

            if let Some(e) = renderer {

                e.shader_program.set_used();

                let color = color.get(entity);

                if let Some(c) = color {

                    unsafe {

                        e.shader_program.set_bool(c.use_position, "UsePosition")?;
                        e.shader_program.set_bool(c.use_vertex_colors, "UseVertexColors")?;

                        let color = {

                            if let Some(t) = timer.get_mut(entity) {
                                (0.0, (f32::sin( t.now.elapsed().as_secs_f64() as f32)  + 1.0 / 2.0), 0.0, 1.0)

                            } else {
                                c.color
                            }

                        };

                        e.shader_program.set_vector4( "VertexColor", color)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn draw_triangles(&mut self, renderers : &mut GenerationalIndexArray<RenderComponent>) {

        for renderer in &renderers.entries {

            if let Some(e) = renderer {

                e.value.shader_program.set_used();

                unsafe {

                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
            } else {
                continue
            }
        }
    }
}