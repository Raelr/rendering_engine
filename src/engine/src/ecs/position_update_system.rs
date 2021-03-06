use crate::ecs::system::System;
use crate::ecs::{PositionComponent, VelocityComponent, BoxCollider2DComponent, RotationComponent, RotationUpdateComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndex, GenerationalIndexArray};
use crate::game_state::GameState;
use nalgebra::{Vector3, Vector2};

pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {

    type SystemInput = &'a mut GameState;

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size : usize = input.get_map::<VelocityComponent>().entries.len();

        for index in 0..size {

            let mut generation = 0;

            let mut idx : GenerationalIndex = GenerationalIndex {index: 0, generation: 0};

            let mut velocity_change : Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

            {
                if let Some(velocity) = input.get_map_mut::<VelocityComponent>().entries[index].as_mut() {

                    idx = velocity.owned_entity;

                    let current_velocity = velocity.value.velocity;

                        let velocity_length = nalgebra::Vector3::magnitude(&current_velocity);

                    if velocity_length > 0.0 {

                        let x = if f32::is_sign_positive(current_velocity.x) { current_velocity.x } else { -current_velocity.x };
                        let y = if f32::is_sign_positive(current_velocity.y) { current_velocity.y } else { -current_velocity.y };

                        velocity_change = Vector3::normalize(&current_velocity);

                        velocity_change = Vector3::new( velocity_change.x * x, velocity_change.y * y, 0.0);
                    }

                    velocity.value.velocity -= Vector3::new(current_velocity.x * 0.2, current_velocity.y * 0.2, 0.0);
                }
            }

            {
                let positions = &mut input.get_mut::<PositionComponent>(&idx).unwrap();

                positions.position += velocity_change;
                //println!("Player position (x: {} y: {})", positions.position.x, positions.position.y);
            }

            {
                if let Some(collider) = &mut input.get_mut::<BoxCollider2DComponent>(&idx) {

                    collider.position += Vector2::new(velocity_change.x, velocity_change.y);
                }
            }
        }
        Ok(())
    }
}

pub struct RotationUpdateSystem();

impl<'a> System<'a> for RotationUpdateSystem {

    type SystemInput = (&'a mut GameState);

    fn run(input: Self::SystemInput) -> Result<(), Error> {


        Ok(())
    }
}