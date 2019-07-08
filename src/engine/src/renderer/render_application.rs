// Crates
extern crate sdl2;
extern crate gl;
extern crate failure;

// Use
use failure::Error;
use sdl2::video::Window;
use sdl2::Sdl;

pub fn generate_n_buffers(amount: i32, buffers: Vec<&mut u32>) {
    unsafe {
        for buffer in buffers {
            gl::GenBuffers(amount, buffer);
        }
    }
}

pub fn generate_buffer_data<T>(buffer_type: gl::types::GLenum, buffer : &u32, vertices : &Vec<T>) {

    unsafe {

        gl::BindBuffer(buffer_type, *buffer);

        // Now that we've bound ARRAY_BUFFER to our vertex_buffer, we need to copy the vertices we
        // specified before INTO the buffer we created:
        gl::BufferData(
            buffer_type, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // Specifies the object does not change. If it did change,
            // the call would be DYNAMIC_DRAW or STREAM_DRAW,
            // which would place the data in an easy to access location
        );
    }
}

pub fn generate_vertex_array(location : u32, components : i32,
                             stride : usize, offset : usize) {
    unsafe {

        let offset = if offset == 0 {
            std::ptr::null()
        } else {
            (offset * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        };

        // Specifies how data stored in the vertex buffer is to be interpreted.
        gl::VertexAttribPointer(
            location, // index of the generic vertex attribute ("layout (location = 0)")
            components, // the number of components per generic vertex attribute. since its a vec3 the size is 3
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            offset // offset of the first component
        );

        gl::EnableVertexAttribArray(location); // this is "layout (location = 0)" in vertex shader
    }
}

pub fn draw(vertex_arrays: &Vec<u32>, stride: i32, is_element: bool) {

    unsafe {

        for vao in vertex_arrays {
            // Binds the vertex array
            gl::BindVertexArray(*vao);

            if is_element {
                gl::DrawElements(gl::TRIANGLES, stride, gl::UNSIGNED_INT, std::ptr::null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, stride);
            }
        }

        // gl::BindVertexArray(vertex_arrays[0 as usize]);
        // Draws count vertices in the vertex buffer or VAO.
        gl::BindVertexArray(0);
    }
}