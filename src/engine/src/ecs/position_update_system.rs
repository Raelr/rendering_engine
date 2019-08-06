use crate::ecs::system::System;
use crate::ecs::{PositionComponent, VelocityComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use crate::game_state::GameState;
use cgmath::vec3;

pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {

    type SystemInput = &'a mut GameState;

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let size : usize = input.get_map::<VelocityComponent>().entries.len();

        for index in 0..size {

            let mut idx : GenerationalIndex;

            let mut generation = 0;

            let mut velocity_change : (f32, f32, f32) = (0.0, 0.0, 0.0);

            {
                if let Some(velocity) = input.get_map_mut::<VelocityComponent>().entries[index].as_mut() {
                    generation = velocity.generation;
                    velocity_change = velocity.value.velocity;

                    
                }
            }

            let idx = GenerationalIndex {index, generation};

            {
                let positions = &mut input.get_map_mut::<PositionComponent>().get_mut(&idx).unwrap();

                positions.position.0 += velocity_change.0;
                positions.position.1 += velocity_change.1;
                positions.position.2 += velocity_change.2;
            }
        }
        Ok(())
    }
}