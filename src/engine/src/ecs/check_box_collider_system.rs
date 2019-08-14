use crate::ecs::system::System;
use crate::ecs::{BoxCollider2DComponent};
use crate::generational_index::generational_index::GenerationalIndexArray;
use nalgebra::Vector2;
use failure::Error;

pub struct CheckBoxColliderSystem;

impl<'a> System<'a> for CheckBoxColliderSystem {

    type SystemInput = (&'a GenerationalIndexArray<BoxCollider2DComponent>,
                        &'a Vector2<f32>);

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        input.0.entries.iter().try_for_each(|collider| -> Result<(), Error> {

            let collider_entry = collider.as_ref().unwrap();

            let test_position = collider_entry.value.position;
            let test_size = collider_entry.value.size;
            let mouse_coordinates = input.1;

            let diameter_width = test_size.x * 0.5;
            let diameter_height = test_size.y * 0.5;

            //println!("Diameter: x: {} y: {}", diameter_width, diameter_height);

            let leftmost_x = test_position.x - diameter_width;
            let leftmost_y = test_position.y - diameter_height;

            let collision_x = leftmost_x + test_size.x >= mouse_coordinates.x && mouse_coordinates.x >= leftmost_x;

            let collision_y = leftmost_y + test_size.y >= mouse_coordinates.y && mouse_coordinates.y >= leftmost_y;

            let collision = collision_x && collision_y;

            println!("{}", collision);

            Ok(())
        });
        Ok(())
    }
}