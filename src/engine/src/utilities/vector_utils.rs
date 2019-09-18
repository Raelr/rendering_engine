use crate::nalgebra::Vector2;

pub fn get_rotation_angle_2(vec1 : Vector2<f32>, vec2: Vector2<f32>) -> f32 {
    f32::atan2(vec2.y - vec1.y, vec2.x - vec1.x)
}

pub fn get_point_after_rotation(vec: Vector2<f32>, angle: f32 ) -> Vector2<f32> {

    let y = (vec.y * f32::cos(angle)) + (vec.x * f32::sin(angle));
    let x = (-vec.y * f32::sin(angle)) + (vec.x * f32::cos(angle));

    Vector2::new(x, y)
}