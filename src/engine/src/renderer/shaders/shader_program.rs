use gl;
use std;

use crate::renderer::shaders::*;

// A Program is a combination of multiple shaders working together to achieve an effect.
pub struct ShaderProgram {

    id : gl::types::GLuint,
}

impl ShaderProgram {

    pub fn from_shaders(shaders: &[shader::Shader]) -> Result<ShaderProgram, String> {

        // Creates a shader program instance in the GPU and returns it's ID
        let program_id = unsafe { gl::CreateProgram() };

        // Attaches the inputted shaders to the program within the GPU.
        for shader in shaders {
            // Attaches the shader to the specific program shader.
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }
        // Links all programs into a single program shader.
        unsafe { gl::LinkProgram(program_id) };


        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        // As with the shader, we need to make sure that the program creation process was successful.
        let mut success: gl::types::GLint = 1;

        unsafe {

            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {

            let mut len: gl::types::GLint = 0;

            unsafe {

                // Get the log length.
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            // Create an empty string.
            let error = shader_utilities::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id, // Object being checked
                    len,        // Error log length
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar // the empty string.
                );
            }
            return Err(error.to_string_lossy().into_owned()); // Return the error.
        }
        Ok(ShaderProgram {id : program_id})
    }

    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}