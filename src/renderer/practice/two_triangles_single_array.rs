use crate::renderer::practice::VertexInformation;
use failure::Error;

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
        vertices : vec![vertices],
        indices : vec![vec![0]]
    };

    Ok(shape)
}