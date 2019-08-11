use crate::ecs::system::System;
use crate::ecs::{PositionComponent, VelocityComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use crate::game_state::GameState;
use cgmath::{vec3, Vector3, InnerSpace};

pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {

    type SystemInput = &'a mut GameState;

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let size : usize = input.get_map::<VelocityComponent>().entries.len();

        for index in 0..size {

            let mut idx : GenerationalIndex;

            let mut generation = 0;

            let mut velocity_change : Vector3<f32> = vec3(0.0, 0.0, 0.0);

            {
                if let Some(velocity) = input.get_map_mut::<VelocityComponent>().entries[index].as_mut() {

                    generation = velocity.generation;

                        let velocity_length = Vector3::magnitude(velocity.value.velocity);

                    if velocity_length > 0.0 {

                        let x = if f32::is_sign_positive(velocity.value.velocity.x) { velocity.value.velocity.x } else { -velocity.value.velocity.x };
                        let y = if f32::is_sign_positive(velocity.value.velocity.y) { velocity.value.velocity.y } else { -velocity.value.velocity.y };
                        let z = if f32::is_sign_positive(velocity.value.velocity.z) { velocity.value.velocity.z } else { -velocity.value.velocity.z };

                        velocity_change = Vector3::normalize(velocity.value.velocity);

                        velocity_change = vec3( velocity_change.x * x, velocity_change.y * y, velocity_change.z * z);
                    }

                    velocity.value.velocity -= vec3(velocity.value.velocity.x * 0.2, velocity.value.velocity.y * 0.2, velocity.value.velocity.z * 0.2)
                }
            }

            let idx = GenerationalIndex {index, generation};

            {
                let positions = &mut input.get_map_mut::<PositionComponent>().get_mut(&idx).unwrap();

                positions.position.x += velocity_change.x;
                positions.position.y += velocity_change.y;
                positions.position.z += velocity_change.z;

                //println!("x: {}, y: {}, z: {}", positions.position.0, positions.position.1, positions.position.2);
            }
        }
        Ok(())
    }
}