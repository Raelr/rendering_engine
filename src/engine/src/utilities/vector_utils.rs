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

pub fn get_rotated_corners(corners: Vec<Vector2<f32>>, center: Vector2<f32>, angle: f32) -> Vec<Vector2<f32>>{
    let mut rotated_crnrs = Vec::new();
    for corner in corners {
        let rot = get_point_after_rotation(corner, center, angle);
        rotated_crnrs.push(rot);
    }

    rotated_crnrs
}

pub fn get_direction_2d(origin: Vector2<f32>, destination: Vector2<f32>) -> Vector2<f32>{

    let heading = destination - origin;
    let distance = Vector2::magnitude(&heading);
    let direction = heading / distance;

    direction
}

pub fn get_projection_2d(vec: Vector2<f32>, axis: Vector2<f32>) {

    //println!("Initial vec: x: {} y: {}", vec.x, vec.y);
    //println!("Axis: x: {} y: {}", axis.x, axis.y);
    let left = vec.dot(&axis);
    //println!("left: {}", left);
    let right = axis.dot(&axis);
    //println!("right: {}", right);
    let scalar = left / right;

    let projection = Vector2::new(scalar * axis.x, scalar * axis.y);
    println!("Projection = x: {} y: {}", projection.x, projection.y);
}