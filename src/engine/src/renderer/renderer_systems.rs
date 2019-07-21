use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex, ArrayEntry};
use crate::components::{PositionComponent, ColorComponent, TimerComponent, RenderComponent};
use failure::Error;
use crate::platform::open_gl::*;
use crate::platform::windows::windows_window::WindowsWindow;
use crate::renderer::shapes::shape::{Quad, Shape};

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

    pub fn draw_quad(renderers : &GenerationalIndexArray<RenderComponent>, shape : &Quad) -> Result<(), Error>{

        renderers.entries.iter().try_for_each(|renderer : &Option<ArrayEntry<RenderComponent>> | -> Result<(), Error>{
            if let Some(renderer) = renderer {
                renderer.value.shader_program.set_used();
                shape.set_texture(&renderer.value)?;
                unsafe {
                    renderer.value.shader_program.set_float("opacity", shape.opacity)?;
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                }
            } Ok(())
        })?;
        Ok(())
    }
}