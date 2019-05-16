use gl;
use std;
use std::ffi::{CString, CStr};

// A Program is a combination of multiple shaders working together to achieve an effect.
pub struct Program {

    id : gl::types::GLuint,
}

impl Program {

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {

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
            let error = create_whitespace_cstring_with_len(len as usize);

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
        Ok(Program {id : program_id})
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

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {

    // Creates a generic shader which takes in a type
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;

        Ok(Shader { id })
    }

    // Creates a shader vertex shader
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    // Creates a fragment shader
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

// Creates a shader of a certain type:
// source = a Cstr containing all the code to be loaded into the shader
// kind = shader kind, passed in as an enum (i.e: VERTEX_SHADER)

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {

    // Creates an empty shader object behind the scenes and assigns it an access id (as id)
    let id = unsafe { gl::CreateShader(kind) };

    // CreateShader returns a 0 if it was unsuccessful, so we need a way to handle a failure.
    let mut success = 1;

    unsafe {
        // Takes in and stores specific elements which will be used to compile the shader.
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        // Actually compiles the shader once all elements were passed in.
        gl::CompileShader(id);
        // Allows the shader to be queried for information. In this case we check if it was compiled
        // successfully and bind that value to the success variable.
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // If the shader failed to compile:
    if success == 0 {

        // define a variable which holds an array size
        let mut len: gl::types::GLint = 0;

        unsafe {
            // Get the length of the info log error.
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // Create a CString which is the same length as the error message.
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            // Gets the shader log info to understand what the error was
            gl::GetShaderInfoLog(
                id, // shader being checked
                len, // info log length
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

// Creates a CString for our error
fn create_whitespace_cstring_with_len(len: usize) -> CString {

    // Allocate buffer to correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

    // Fill buffer with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));

    // Convert buffer to Cstring
    unsafe { CString::from_vec_unchecked(buffer) }
}