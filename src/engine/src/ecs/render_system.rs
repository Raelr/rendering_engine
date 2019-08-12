use crate::ecs::system::System;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use crate::ecs::*;
use failure::Error;
use cgmath::{vec3, Matrix, Rad, Deg};
use cgmath::Vector3;
use std::ffi::CString;
use crate::cgmath::InnerSpace;
use crate::game_state::GameState;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {

    type SystemInput = (&'a GenerationalIndexArray<RenderComponent>,
                        &'a GenerationalIndexArray<PositionComponent>,
                        &'a GenerationalIndexArray<ColorComponent>,
                        &'a GenerationalIndexArray<TextureMixComponent>);

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let shaders = &input.0.entries;

        let mut idx = 0;

        shaders.into_iter().try_for_each(|shader| -> Result<(), Error> {

            println!("{}", idx);

            if let Some(shader_program) = shader {

                let index = GenerationalIndex {index: idx, generation : shader_program.generation};

                unsafe {

                    gl::BindVertexArray(shader_program.value.vertex_array_object);

                    // Set shader program being used.
                    gl::UseProgram(shader_program.value.shader_program);

                    // Set Position of Shader
                    let position = input.1.get(&index).unwrap();

                    let trans = cgmath::Matrix4::from_translation(position.position)
                    * cgmath::Matrix4::from_nonuniform_scale(0.5, 0.75, 0.5);

                    RenderSystem::set_mat4(shader_program.value.shader_program, "Transform", trans)?;

                    // Set Color of Shader
                    let color = input.2.get(&index).unwrap();

                    RenderSystem::set_vector4(shader_program.value.shader_program, "Color", (color.color.0, color.color.1, color.color.2, color.color.3))?;

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

    pub unsafe fn set_mat4(id : gl::types::GLuint, name : &str, mat : cgmath::Matrix4<f32> ) -> Result<(), Error> {

        gl::UniformMatrix4fv(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()),1, gl::FALSE, mat.as_ptr());

        Ok(())
    }
}