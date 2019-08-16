use crate::ecs::system::System;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use crate::ecs::*;
use failure::Error;
use std::ffi::CString;
use crate::game_state::GameState;
use nalgebra::Vector4;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {

    type SystemInput = (&'a GenerationalIndexArray<RenderComponent>,
                        &'a GenerationalIndexArray<PositionComponent>,
                        &'a GenerationalIndexArray<ColorComponent>,
                        &'a GenerationalIndexArray<TextureMixComponent>,
                        &'a GenerationalIndexArray<ScaleComponent>,
                        &'a OrthographicCameraComponent);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let shaders = &input.0.entries;

        let mut idx = 0;

        shaders.into_iter().try_for_each(|shader| -> Result<(), Error> {

            if let Some(shader_program) = shader {

                let index = GenerationalIndex {index: idx, generation : shader_program.generation};

                unsafe {

                    gl::BindVertexArray(shader_program.value.vertex_array_object);

                    // Set shader program being used.
                    gl::UseProgram(shader_program.value.shader_program);

                    // START POSITION RENDERING VARIABLES ------------------------------------------

                    let position = input.1.get(&index).unwrap();

                    let scale = input.4.get(&index).unwrap();

                    let model = nalgebra::Matrix4::new_translation(&position.position) * nalgebra::Matrix4::new_nonuniform_scaling(&scale.scale);

                    //println!("{}, {}, {}", position.position.x, position.position.y, position.position.z);

                    RenderSystem::set_mat4(shader_program.value.shader_program, "Model", model)?;

                    RenderSystem::set_mat4(shader_program.value.shader_program, "View", input.5.view)?;

                    RenderSystem::set_mat4(shader_program.value.shader_program, "Projection", input.5.projection)?;

                    // END OF POSITION RENDERING VARIABLES -----------------------------------------

                    // COLOR RENDERING VARIABLES
                    let color = input.2.get(&index).unwrap();

                    RenderSystem::set_vector4(shader_program.value.shader_program, "Color", (color.color.0, color.color.1, color.color.2, color.color.3))?;

                    // TEXTURE RENDERING VARIABLES
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

                    // DRAW VERTICES
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

    pub unsafe fn set_mat4(id : gl::types::GLuint, name : &str, mat : nalgebra::Matrix4<f32> ) -> Result<(), Error> {

        gl::UniformMatrix4fv(gl::GetUniformLocation(id, CString::new(name)?.as_ptr()),1, gl::FALSE, mat.as_ptr());

        Ok(())
    }
}