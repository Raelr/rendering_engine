use crate::ecs::OrthographicCameraComponent;
use nalgebra::{Vector3, Vector2, Matrix4, Vector4};

pub fn ortho_screen_to_world_coordinates(camera: &OrthographicCameraComponent, coordinates : Vector2<f32>) -> Vector2<f32>{

    let clicked = Vector4::new((coordinates.x/ camera.dimensions.x) * 2.0 - 1.0,
                               (coordinates.y/ camera.dimensions.y) * 2.0 - 1.0,
                               0.5, 1.0);

    let orthographic_projection = camera.projection;

    let projection_view = orthographic_projection * camera.view;

    let inversed: Matrix4<f32> = nalgebra::Matrix4::qr(projection_view).try_inverse().unwrap();

    let inversed = inversed * clicked;

    println!("Left clicked at position: x: {} y: {} x: {}", inversed.x, inversed.y, inversed.z);

    Vector2::new(inversed.x, -inversed.y)
}