use crate::nalgebra::Vector2;

pub fn get_rotation_angle_2(vec1 : Vector2<f32>, vec2: Vector2<f32>) -> f32 {
    f32::atan2(vec2.y - vec1.y, vec2.x - vec1.x)
}

pub fn get_point_after_rotation(vec: Vector2<f32>, center: Vector2<f32>, angle: f32 ) -> Vector2<f32> {

    let x = (((vec.x - center.x) * f32::cos(angle)) - ((vec.y - center.y) * f32::sin(angle)) + center.x);
    let y = (((vec.x - center.x) * f32::sin(angle)) + ((vec.y - center.y) * f32::cos(angle)) + center.y);

    Vector2::new(x, y)
}

pub fn get_box_corners(center : Vector2<f32>, scale: Vector2<f32>) -> Vec<Vector2<f32>> {

    let max_x = center.x + (scale.x * 0.5);
    let min_x = center.x - (scale.x * 0.5);

    let max_y = center.y + (scale.y * 0.5);
    let min_y = center.y - (scale.y * 0.5);

    vec![Vector2::new(max_x, max_y), Vector2::new(max_x, min_y), Vector2::new(min_x, max_y), Vector2::new(min_x, min_y)]
}