use crate::renderer::practice::VertexInformation;
use failure::Error;

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
        vertices : vec![vertices],
        indices : vec![indices]
    };

    Ok(shape_vertices)
}

