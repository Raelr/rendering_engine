use failure::Error;
use crate::platform::open_gl::*;
use crate::platform::windows::windows_window::WindowsWindow;
use image::GenericImageView;
use std::os::raw::c_void;
use crate::ecs::RenderComponent;

extern crate gl;

#[macro_export]
// Macro for creating a key typed event.
macro_rules! quad { () => {{

        use image::GenericImageView;
        use std::os::raw::c_void;
        use crate::platform::open_gl::*;

        let mut vertex_array_object: gl::types::GLuint = 0;
        let mut vertex_buffer_object: gl::types::GLuint = 0;
        let mut element_buffer_object: gl::types::GLuint = 0;

        unsafe {

            let vertices : Vec<gl::types::GLfloat> = vec![
             // Position      //Texture
             0.5,  0.5, 0.0,  1.0, 1.0, // top right
             0.5, -0.5, 0.0,  1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,  0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,  0.0, 1.0  // top left
            ];

            let indices : Vec<gl::types::GLuint> = vec! [
                0, 1, 3,
                1, 2, 3
            ];

            unsafe {

                generate_n_buffers(1, vec![&mut vertex_buffer_object, &mut element_buffer_object]);

                gl::GenVertexArrays(1, &mut vertex_array_object);

                gl::BindVertexArray(vertex_array_object);

                generate_buffer_data(gl::ARRAY_BUFFER, &vertex_buffer_object, &vertices);

                generate_buffer_data(gl::ELEMENT_ARRAY_BUFFER, &element_buffer_object, &indices);

                generate_vertex_array(0, 3, 5, 0);

                generate_vertex_array(1, 2, 5, 3);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);
            }
        }
        vertex_array_object
    }};
}




