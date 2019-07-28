use crate::ecs::{RenderComponentTemp, PositionComponent, ColorComponent, TextureMixComponent, TextureUpdateComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use std::ffi::CString;
use crate::game_state::GameState;
use std::borrow::BorrowMut;

pub trait System<'a> {

    type SystemInput;

    fn run(&self, input : Self::SystemInput) -> Result<(), Error>;
}

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {

    type SystemInput = (&'a GenerationalIndexArray<RenderComponentTemp>,
                        &'a GenerationalIndexArray<PositionComponent>,
                        &'a GenerationalIndexArray<ColorComponent>,
                        &'a GenerationalIndexArray<TextureMixComponent>);

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let shaders = &input.0.entries;

        let mut idx = 0;

        shaders.into_iter().try_for_each(|shader| -> Result<(), Error> {

            if let Some(shader_program) = shader {

                let index = GenerationalIndex {index: idx, generation : shader_program.generation};

                unsafe {

                    gl::BindVertexArray(shader_program.value.vertex_array_object);

                    // Set shader program being used.
                    gl::UseProgram(shader_program.value.shader_program);

                    // Set Position of Shader
                    let position = input.1.get(&index).unwrap();

                        RenderSystem::set_vector2(shader_program.value.shader_program, "Offset", (position.position.0, position.position.1))?;

                        RenderSystem::set_bool(shader_program.value.shader_program, position.reversed, "ReverseShape", )?;

                    // Set Color of Shader
                    let color = input.2.get(&index).unwrap();

                        RenderSystem::set_bool(shader_program.value.shader_program,color.use_position, "UsePosition")?;

                        RenderSystem::set_bool(shader_program.value.shader_program, color.use_vertex_colors, "UseVertexColors")?;

                        RenderSystem::set_vector4( shader_program.value.shader_program,"VertexColor", color.color)?;

                    // Set texture
                    let texture_mix = input.3.get(&index);

                    if let Some(texture_comp) = texture_mix {

                        RenderSystem::set_bool(shader_program.value.shader_program, true, "usingTextures")?;

                        for texture in texture_comp.textures.iter() {

                            RenderSystem::set_int(shader_program.value.shader_program, &texture.uniform_name, texture.number)?;
                            RenderSystem::set_float(shader_program.value.shader_program, "opacity", texture_comp.opacity)?;
                            gl::ActiveTexture(texture.active_texture_enum);
                            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);

                        }
                    }

                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                    RenderSystem::set_bool(shader_program.value.shader_program, false, "usingTextures")?;

                } idx += 1;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl RenderSystem {

    pub unsafe fn set_bool(id : gl::types::GLuint, value : bool, name : &str) -> Result<(), Error> {
        let condition : u32 = match value {
            true => (1),
            false => (0)
        };

        gl::Uniform1ui(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()), condition);

        Ok(())
    }

    pub unsafe fn set_vector4(id : gl::types::GLuint, name : &str, vec : (f32, f32, f32, f32)) -> Result<(), Error>{

        gl::Uniform4f(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()), vec.0, vec.1, vec.2, vec.3);

        Ok(())
    }

    pub unsafe fn set_vector2(id : gl::types::GLuint, name : &str, vec : (f32, f32)) -> Result<(), Error> {

        gl::Uniform2f(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()), vec.0, vec.1);

        Ok(())
    }

    pub unsafe fn set_int(id : gl::types::GLuint, name : &str, number : i32) -> Result<(), Error> {

        gl::Uniform1i(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()), number);

        Ok(())
    }

    pub unsafe fn set_float(id : gl::types::GLuint, name : &str, number : f32) -> Result<(), Error> {

        gl::Uniform1f(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()), number);

        Ok(())
    }
}

pub struct TextureUpdateSystem;

impl<'a> System<'a> for TextureUpdateSystem {

    type SystemInput = (&'a mut GameState);

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let size = input.get_map::<TextureUpdateComponent>().entries.len();

        let mut opacity: gl::types::GLfloat = 0.0;

        for index in 0..size {

            let mut generation = 0;

            {
                let mut updates = &mut input.get_map_mut::<TextureUpdateComponent>();

                if let Some(change) = updates.entries[index].as_mut() {
                    opacity = change.value.opacity_change;
                    generation = change.generation;
                    change.value.opacity_change = 0.0;
                } else {
                    continue
                }
            }

            let gen_index = GenerationalIndex { index, generation};

            let textures = input.get_map_mut::<TextureMixComponent>();

            if let Some(texture) = textures.get_mut(&gen_index) {
                texture.opacity += opacity
            }
        }
        Ok(())
    }
}