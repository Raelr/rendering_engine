use failure::Error;
use crate::platform::open_gl::*;
use crate::platform::windows::windows_window::WindowsWindow;
use image::GenericImageView;
use std::os::raw::c_void;

extern crate gl;

pub trait Shape {
    type ArrayObject;

    fn get_vertex_array_object(&self) -> Self::ArrayObject;
    fn init(&mut self, window : &WindowsWindow) -> Result<(), Error>;
    fn set_used(&self);
    fn set_texture(&self) {}

}

pub struct Triangle {

    vertex_array_object : gl::types::GLuint,
}

impl Triangle {

    pub fn new() -> Triangle {

        Triangle { vertex_array_object : 0 }
    }
}

impl Shape for Triangle {

    type ArrayObject = gl::types::GLuint;

    fn get_vertex_array_object(&self) -> Self::ArrayObject {

        self.vertex_array_object
    }

    fn init(&mut self, window : &WindowsWindow) -> Result<(), Error> {

        let vertices: Vec<gl::types::GLfloat> = vec![

            // positions     // colors
            0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
            0.0,  0.5, 0.0,  0.0, 0.0, 1.0,
        ];

        let mut vertex_buffer_object: gl::types::GLuint = 0;

        generate_n_buffers(1, vec![&mut vertex_buffer_object]);

        unsafe {

            gl::GenVertexArrays(1, &mut self.vertex_array_object);

            // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in,
            // the VAO
            gl::BindVertexArray(self.vertex_array_object);

            // Binds the created buffer to a specific type (in this case we specify that this is an
            // array buffer)
            generate_buffer_data(gl::ARRAY_BUFFER, &vertex_buffer_object, &vertices);

            // Creates a vertex attribute pointer and enables it on the GPU
            generate_vertex_array(0, 3, 6, 0);

            generate_vertex_array(1, 3, 6, 3);

            gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32);

            // Resets the bindings on the GPU
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            Ok(())
        }
    }

    fn set_used(&self) {

        unsafe { gl::BindVertexArray(self.vertex_array_object); }
    }
}

pub struct Quad {

    vertex_array_object : gl::types::GLuint,
    pub element_buffer_object : gl::types::GLuint,
    pub texture : gl::types::GLuint
}

impl Quad {

    pub fn new() -> Quad {
        Quad { vertex_array_object : 0, element_buffer_object : 0, texture : 0}
    }
}

impl Shape for Quad {

    type ArrayObject = gl::types::GLuint;

    fn get_vertex_array_object(&self) -> Self::ArrayObject {
        self.vertex_array_object
    }

    fn init(&mut self, window: &WindowsWindow) -> Result<(), Error> {

        let vertices : Vec<gl::types::GLfloat> = vec![
             // Position      // Color        //Texture
             0.5,  0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0, // top right
             0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,  1.0, 1.0, 0.0,  0.0, 1.0  // top left
        ];

        let indices : Vec<gl::types::GLuint> = vec! [
            0, 1, 3,
            1, 2, 3
        ];

        unsafe {

            let mut vertex_buffer_object: gl::types::GLuint = 0;

            gl::GenTextures(1, &mut self.texture);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            let image  = image::open("src/engine/src/renderer/textures/container.jpg")?;

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width() as i32, image.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, image.to_rgba().into_raw().as_ptr() as *const c_void);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::GenerateMipmap(gl::TEXTURE_2D);

            // TEXTURES

            generate_n_buffers(1, vec![&mut vertex_buffer_object, &mut self.element_buffer_object]);

            gl::GenVertexArrays(1, &mut self.vertex_array_object);

            gl::BindVertexArray(self.vertex_array_object);

            generate_buffer_data(gl::ARRAY_BUFFER, &vertex_buffer_object, &vertices);

            generate_buffer_data(gl::ELEMENT_ARRAY_BUFFER, &self.element_buffer_object, &indices);

            generate_vertex_array(0, 3, 8, 0);

            generate_vertex_array(1, 3, 8, 3);

            generate_vertex_array(2, 3, 8, 6);

            gl::Viewport(0, 0, window.data.width as i32, window.data.height as i32);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        
        Ok(())
    }

    fn set_used(&self) {
        unsafe { gl::BindVertexArray(self.vertex_array_object) };
    }

    fn set_texture(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.texture); }
    }
}


