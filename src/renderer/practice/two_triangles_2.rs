use crate::renderer::practice::VertexInformation;
use failure::Error;

pub fn create_two_triangles() -> Result<VertexInformation, Error> {


    let vertices1: Vec<f32> = vec! [

        //triangle 1
        0.0, 0.0, 0.0,  // middle
        -0.25, 0.5, 0.0, // top left
        -0.5, 0.0, 0.0, // left

    ];

    let vertices2: Vec<f32> = vec![

        // triangle 2
        0.0, 0.0, 0.0, // middle
        0.25, 0.5, 0.0, // top right
        0.5, 0.0, 0.0  // right
    ];

    let shape_vertices =  VertexInformation {
        vertices : vec![vertices1, vertices2],
        indices : vec![vec![0]]
    };

    Ok(shape_vertices)
}