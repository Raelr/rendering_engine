extern crate gl;

// Use
use failure::Error;
use std::ffi::{CString};

use crate::renderer::shaders::shader::Shader;
use crate::renderer::shaders::shader_program::ShaderProgram;
use crate::renderer::render_application;
use crate::renderer::render_application::RendererInformation;

pub struct VertexInformation {
    pub vertices : Vec<f32>,
    pub indices : Vec<u32>
}

pub fn basic_program() -> Result<ShaderProgram, Error> {

    Ok(ShaderProgram::from_shaders(&[Shader::from_vert_source(
            &CString::new(include_str!("triangle.vert"))
                    .unwrap()).unwrap(),
                    Shader::from_frag_source(&CString::new(
                    include_str!("triangle.frag"))
                    .unwrap()).unwrap()]).unwrap())
}

pub fn create_triangle_quad() -> Result<VertexInformation, Error> {

    // Specify an array of vertices (positioned as x, y, z coordinates)
    // This array forms a triangle.
    let vertices: Vec<f32> = vec![
        // positions        // Colors
        0.5,  0.5, 0.0,    1.0, 0.0, 0.0, // bottom right
        0.5, -0.5, 0.0,    0.0, 1.0, 0.0, // bottom left
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0, // top
        -0.5,  0.5, 0.0,   0.5, 0.2, 0.0
    ];

    // The drawing order of indices within the vertex array.
    let indices : Vec<gl::types::GLuint> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    let shape_vertices =  VertexInformation {
        vertices,
        indices
    };

    Ok(shape_vertices)
}

pub fn create_two_triangles() -> Result<VertexInformation, Error>{

    let vertices: Vec<f32> = vec! [

        //triangle 1
        0.0, 0.0, 0.0,  // middle
        -0.25, 0.5, 0.0, // top left
        -0.5, 0.0, 0.0, // left

        // traingle 2
        0.0, 0.0, 0.0, // middle
        0.25, 0.5, 0.0, // top right
        0.5, 0.0, 0.0  // right
    ];

    let shape = VertexInformation {
        vertices,
        indices : vec![0]
    };

    Ok(shape)
}

pub fn render_basic_square_with_elements(renderer : &RendererInformation) -> Result<u32, Error> {

    // returns a struct which stores both index and vertex information.
    let shape = create_triangle_quad()?;

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
        render_application::generate_buffer_data(gl::ARRAY_BUFFER,
                                                           &vertex_buffer, &shape.vertices);

        render_application::generate_buffer_data(gl::ELEMENT_ARRAY_BUFFER,
                                                           &element_buffer_object,
                                                           &shape.indices);

        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 6, 0);

        render_application::generate_vertex_array(1, 3, 6, 3);

        // Resets the bindings on the GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Set the viewport for the image.
        gl::Viewport(0, 0, 900, 900); // Set viewport.

        // Set the color of the window.
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set window color.
    }

    Ok(vertex_array_object)
}

pub fn render_from_vertex_array(renderer : &RendererInformation)-> Result<u32, Error>{

    // returns a struct which stores both index and vertex information.
    let shape = create_two_triangles()?;

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
        render_application::generate_buffer_data(gl::ARRAY_BUFFER,
                                                 &vertex_buffer, &shape.vertices);

        // Creates a vertex attribute pointer and enables it on the GPU
        render_application::generate_vertex_array(0, 3, 3, 0);

        // Resets the bindings on the GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Set the viewport for the image.
        gl::Viewport(0, 0, 900, 900); // Set viewport.

        // Set the color of the window.
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set window color.
    }

    Ok(vertex_array_object)
}

