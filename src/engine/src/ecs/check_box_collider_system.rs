use crate::ecs::system::System;
use crate::ecs::{BoxCollider2DComponent, PositionComponent};
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use nalgebra::{Vector2, Vector3};
use failure::Error;
use crate::game_state::GameState;

pub struct CheckBoxColliderSystem;

impl<'a> System<'a> for CheckBoxColliderSystem {

    type SystemInput = (&'a mut GameState,
                        &'a Vector2<f32>);

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let idx : usize = 0;

        let size = input.0.get_map::<BoxCollider2DComponent>().entries.len();

        for index in 0..size {

            let mut gen_idx : GenerationalIndex;

            let mut generation = 0;

            let mut collided = false;

            {
                let collider_entry = input.0.get_map_mut::<BoxCollider2DComponent>().entries[index].as_mut().unwrap();

                let test_position = collider_entry.value.position;
                let test_size = collider_entry.value.size;
                let mouse_coordinates = input.1;

                let leftmost_x = test_position.x - (test_size.x * 0.5);
                let leftmost_y = test_position.y - (test_size.y * 0.5);

                let collision_x = leftmost_x + test_size.x >= mouse_coordinates.x && mouse_coordinates.x >= leftmost_x;

                let collision_y = leftmost_y + test_size.y >= mouse_coordinates.y && mouse_coordinates.y >= leftmost_y;

                collided = collision_x && collision_y;

                println!("{}", collided);

                if collided {
                    collider_entry.value.position = mouse_coordinates.clone();
                }

                generation = collider_entry.generation;
            }

            gen_idx = GenerationalIndex { index, generation};

            if collided {

                let position = input.0.get_mut::<PositionComponent>(&gen_idx).unwrap();

                position.position = Vector3::new(input.1.x, input.1.y, 0.0);
            }

        }
        Ok(())
    }
}