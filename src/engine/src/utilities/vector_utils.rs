use crate::nalgebra::Vector2;

pub fn get_rotation_angle_2(vec1 : Vector2<f32>, vec2: Vector2<f32>) -> f32{
    f32::atan2(vec2.y - vec1.y, vec2.x - vec1.x)
}