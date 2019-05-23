use crate::renderer::practice::VertexInformation;
use failure::Error;

pub fn create_square() -> Result<VertexInformation, Error> {

    // Specify an array of vertices (positioned as x, y, z coordinates)
    // This array forms a triangle.
    let vertices: Vec<f32> = vec![
        // first square
         0.0,  0.5, 0.0, // 0
         -0.3, -0.5, 0.0, // 1
        -0.4, -0.5, 0.0, // 2
        -0.1,  0.5, 0.0, // 3

        // second square
        0.1, 0.5, 0.0, // 4
        0.3,  -0.5, 0.0, // 5
        0.4, -0.5, 0.0, // 6

        // top triangle
        0.0, 0.84, 0.0, // 7

        // Middle Separator 1st half
        0.12, 0.1, 0.0, // 8
        -0.15, 0.0, 0.0, // 9
        -0.12, 0.1, 0.0, // 10

        // Middle separator 2nd half
        0.15, 0.0, 0.0 // 11
    ];

    // The drawing order of indices within the vertex array.
    let indices : Vec<gl::types::GLuint> = vec![
        0, 1, 3,
        1, 2, 3,
        0, 4, 5,
        5, 6, 4,
        3, 4, 7,
        8, 9, 10,
        11, 8, 9
    ];

    let shape_vertices =  VertexInformation {
        vertices : vec![vertices],
        indices : vec![indices]
    };

    Ok(shape_vertices)
}