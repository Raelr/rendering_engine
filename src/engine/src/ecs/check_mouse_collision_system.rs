use crate::ecs::system::System;
use crate::ecs::{BoxCollider2DComponent, PositionComponent, ColorComponent, SelectedComponent};
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use nalgebra::{Vector2, Vector3};
use failure::Error;
use crate::game_state::GameState;
use crate::ecs::selection_system;

pub struct CheckBoxColliderSystem;

impl<'a> System<'a> for CheckBoxColliderSystem {

    type SystemInput = (&'a mut GameState,
                        &'a Vector2<f32>);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let idx : usize = 0;

        let size = input.0.get_map::<BoxCollider2DComponent>().entries.len();

        for index in 0..size {

            let mut gen_idx : GenerationalIndex;

            let mut generation = 0;

            let mut collided = false;

            let mut offset : Vector2<f32>;

            {
                let collider_entry = input.0.get_map_mut::<BoxCollider2DComponent>().entries[index].as_mut().unwrap();

                let position = collider_entry.value.position;
                let size = collider_entry.value.size;
                let mouse_coordinates = input.1;

                let leftmost_x = position.x - (size.x * 0.5);
                let leftmost_y = position.y - (size.y * 0.5);

                let collision_x = leftmost_x + size.x >= mouse_coordinates.x && mouse_coordinates.x >= leftmost_x;

                let collision_y = leftmost_y + size.y >= mouse_coordinates.y && mouse_coordinates.y >= leftmost_y;

                collided = collision_x && collision_y;

                generation = collider_entry.generation;

                let heading = Vector2::new(mouse_coordinates.x, mouse_coordinates.y);
                let distance = Vector2::magnitude(&heading);
                let direction = heading / distance;

                offset = position - direction * distance;
                println!("x: {}, y: {}", offset.x, offset.y);
            }

            gen_idx = GenerationalIndex { index, generation};

            let selected = match input.0.get::<SelectedComponent>(&gen_idx) {
                Some(val) => true,
                None => false
            };

            if collided {
                {
                    if !selected {
                        input.0.add_component_to(SelectedComponent { selected_color: (0.7, 0.7, 0.7, 0.7), cursor_offset: offset}, &gen_idx);
                    } else {
                        input.0.get_mut::<SelectedComponent>(&gen_idx).unwrap().cursor_offset = offset;
                    }
                }
            } else {
                selection_system::DeselectSystem::run(input.0);
            }
        }
        Ok(())
    }
}