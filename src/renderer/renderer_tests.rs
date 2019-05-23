extern crate gl;

// Use
use failure::Error;
use std::ffi::{CString};

use crate::renderer::shaders::shader::Shader;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::renderer::render_application;
use crate::renderer::render_application::RendererInformation;
use crate::renderer::practice::*;

pub enum TestType {
    RectangleElement,
    TwoTrianglesSingleVertex,
    TwoTrianglesTwoVertices,
    UpperCaseA
}

pub fn basic_program() -> Result<ShaderProgram, Error> {

    Ok(ShaderProgram::from_shaders(&[Shader::from_vert_source(
            &CString::new(include_str!("triangle.vert"))
                    .unwrap()).unwrap(),
                    Shader::from_frag_source(&CString::new(
                    include_str!("triangle.frag"))
                    .unwrap()).unwrap()]).unwrap())
}

pub fn render_basic_square_with_elements() -> Result<Vec<u32>, Error> {

    // returns a struct which stores both index and vertex information.
    let shape = square_render::create_triangle_quad()?;

    // Creates a vertex buffer in the GPU. the uint is an unique id which allows quick access to the
    // buffer.
    let mut vertex_buffer : gl::types::GLuint = 0;

    let mut element_buffer_object : gl::types::GLuint = 0;

    let mut vertex_array_object: gl::types::GLuint = 0;

    // Generates buffers for all buffer objects.
    render_application::generate_n_buffers(
        1, vec![&mut vertex_buffer, &mut element_buffer_object]);

    unsafe {

        // Generates a vertex array object (VAO) and returns ints ID.
        gl::GenVertexArrays(1, &mut vertex_array_object);

        // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in
        // the VAO
        gl::BindVertexArray(vertex_array_object);

        // Binds the created buffer to a specific type (in this case we specify that this is an
        // array buffer)

        for vertices in &shape.vertices {
            render_application::generate_buffer_data(gl::ARRAY_BUFFER, &vertex_buffer, vertices);
        }

        for indices in &shape.indices {
            render_application::generate_buffer_data(gl::ELEMENT_ARRAY_BUFFER,
                                                     &element_buffer_object,
                                                     indices);
        }

        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 6, 0);

        render_application::generate_vertex_array(1, 3, 6, 3);

        // Resets the bindings on the GPU
//        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
//        gl::BindVertexArray(0);
    }

    Ok(vec![vertex_array_object])
}

pub fn render_from_vertex_array()-> Result<Vec<u32>, Error>{

    // returns a struct which stores both index and vertex information.
    let shape = two_triangles_single_array::create_two_triangles()?;

    // Creates a vertex buffer in the GPU. the uint is an unique id which allows quick access to the
    // buffer.
    let mut vertex_buffer : gl::types::GLuint = 0;

    let mut vertex_array_object: gl::types::GLuint = 0;

    // Generates buffers for all buffer objects.
    render_application::generate_n_buffers(
        1, vec![&mut vertex_buffer]);

    unsafe {

        // Generates a vertex array object (VAO) and returns ints ID.
        gl::GenVertexArrays(1, &mut vertex_array_object);

        // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in
        // the VAO
        gl::BindVertexArray(vertex_array_object);

        // Binds the created buffer to a specific type (in this case we specify that this is an
        // array buffer)
        for vertices in &shape.vertices {
            render_application::generate_buffer_data(gl::ARRAY_BUFFER,
                                                     &vertex_buffer, vertices);

        }
        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 3, 0);

        // Resets the bindings on the GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    Ok(vec![vertex_array_object])
}

pub fn render_from_separate_arrays() -> Result<Vec<u32>, Error> {


    let shapes = two_triangles_2::create_two_triangles()?;

    let mut vertex_buffer1 : gl::types::GLuint = 0;

    let mut vertex_buffer2 : gl::types::GLuint = 0;

    let mut vertex_array1 : gl::types::GLuint = 0;

    let mut vertex_array2 : gl::types::GLuint = 0;

    render_application::generate_n_buffers(1, vec![&mut vertex_buffer1, &mut vertex_buffer2]);

    unsafe {

        gl::GenVertexArrays(1, &mut vertex_array1);

        // Whenever an object needs to be rendered using separate VAOs and VBOs, these steps MUST be followed:

        gl::GenVertexArrays(1, &mut vertex_array2);  // 1. Generate array
        gl::BindVertexArray(vertex_array1); // 2. Bind the vertex array

        render_application::generate_buffer_data(gl::ARRAY_BUFFER, // 3. Generate buffer data
                                                 &vertex_buffer1, &shapes.vertices[0 as usize]);

        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 3, 0); // 4. Create attribute pointer and enable it.

        gl::BindVertexArray(vertex_array2);

        render_application::generate_buffer_data(gl::ARRAY_BUFFER,
                                                 &vertex_buffer2, &shapes.vertices[1 as usize]);

        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 0, 0);

        // Resets the bindings on the GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    Ok(vec![vertex_array1, vertex_array2])
}

pub fn draw_uppercase_a()-> Result<Vec<u32>, Error>{

    // returns a struct which stores both index and vertex information.
    let shape = square::create_square()?;

    // Creates a vertex buffer in the GPU. the uint is an unique id which allows quick access to the
    // buffer.
    let mut vertex_buffer : gl::types::GLuint = 0;

    let mut element_buffer_object : gl::types::GLuint = 0;

    let mut vertex_array_object: gl::types::GLuint = 0;

    // Generates buffers for all buffer objects.
    render_application::generate_n_buffers(
        1, vec![&mut vertex_buffer, &mut element_buffer_object]);

    unsafe {

        // Generates a vertex array object (VAO) and returns ints ID.
        gl::GenVertexArrays(1, &mut vertex_array_object);

        // Binds a VAO  to the GPU. From now on, and changes to VBO's or vertices will be stored in
        // the VAO
        gl::BindVertexArray(vertex_array_object);

        // Binds the created buffer to a specific type (in this case we specify that this is an
        // array buffer)

        for vertices in &shape.vertices {
            render_application::generate_buffer_data(gl::ARRAY_BUFFER, &vertex_buffer, vertices);
        }

        for indices in &shape.indices {
            render_application::generate_buffer_data(gl::ELEMENT_ARRAY_BUFFER,
                                                     &element_buffer_object,
                                                     indices);
        }
        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 3, 0);

        // Resets the bindings on the GPU
        //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //gl::BindVertexArray(0);
    }
    Ok(vec![vertex_array_object])
}



