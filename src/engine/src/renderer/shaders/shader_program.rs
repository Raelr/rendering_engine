use gl;
use std;

use crate::renderer::shaders::*;
use std::ffi::{CString};
use failure::Error;


#[macro_export]
// Macro for creating a key typed event.
macro_rules! triangle_render {
    () => {{
    use crate::renderer::shaders::shader_program::*;
    use crate::renderer::shaders::shader::*;
    use std::ffi::{CString};

        let program = ShaderProgram::from_shaders(&[Shader::from_vert_source(
            &CString::new(include_str!("/Users/aryehzinn/Desktop/Development/scrapyard_engine/src/engine/src/renderer/triangle.vert"))
                    .unwrap()).unwrap(),
                    Shader::from_frag_source(&CString::new(
                    include_str!("/Users/aryehzinn/Desktop/Development/scrapyard_engine/src/engine/src/renderer/triangle.frag"))
                    .unwrap()).unwrap()]).unwrap();
        program
    }};
}

#[macro_export]
// Macro for creating a key typed event.
macro_rules! triangle_fade {
    () => {{
        let program = ShaderProgram::from_shaders(&[Shader::from_vert_source(
        &CString::new(include_str!("renderer/fade_triangle.glsl"))
            .unwrap()).unwrap(),
        Shader::from_frag_source(&CString::new(
            include_str!("renderer/fade_triangle.frag"))
            .unwrap()).unwrap()]).unwrap();

            program
    }};
}

// A Program is a combination of multiple shaders working together to achieve an effect.
pub struct ShaderProgram {

    id : gl::types::GLuint,
}

/// A program which combines multiple shaders to create a single shape.
impl ShaderProgram {

    pub fn from_shaders(shaders: &[shader::Shader]) -> Result<gl::types::GLuint, String> {

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

        Ok(program_id)
    }

    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub unsafe fn set_bool(&self, value : bool, name : &str) -> Result<(), Error> {
        let condition : u32 = match value {
            true => (1),
            false => (0)
        };

        gl::Uniform1ui(gl::GetUniformLocation(self.id(), CString::new(name)?.as_ptr()), condition);

        Ok(())
    }

    pub unsafe fn set_vector4(&self, name : &str, vec : (f32, f32, f32, f32)) -> Result<(), Error>{

        gl::Uniform4f(gl::GetUniformLocation(self.id(), CString::new(name)?.as_ptr()), vec.0, vec.1, vec.2, vec.3);

        Ok(())
    }

    pub unsafe fn set_vector2(&self, name : &str, vec : (f32, f32)) -> Result<(), Error> {

        gl::Uniform2f(gl::GetUniformLocation(self.id(), CString::new(name)?.as_ptr()), vec.0, vec.1);

        Ok(())
    }

    pub unsafe fn set_int(&self, name : &str, number : i32) -> Result<(), Error> {

        gl::Uniform1i(gl::GetUniformLocation(self.id(), CString::new(name)?.as_ptr()), number);

        Ok(())
    }

    pub unsafe fn set_float(&self, name : &str, number : f32) -> Result<(), Error> {

        gl::Uniform1f(gl::GetUniformLocation(self.id(), CString::new(name)?.as_ptr()), number);

        Ok(())
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}