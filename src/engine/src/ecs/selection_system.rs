use crate::ecs::system::System;
use failure::Error;
use crate::game_state::GameState;
use crate::ecs::{SelectedComponent, ColorComponent, PositionComponent, BoxCollider2DComponent};
use nalgebra::{Vector3, Vector2};
use crate::generational_index::generational_index::GenerationalIndex;
use crate::input::input_handler::InputHandler;

pub struct SelectionSystem;

impl<'a> System<'a> for SelectionSystem {

    type SystemInput = (&'a mut GameState, &'a InputHandler);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size  = input.0.get_map::<SelectedComponent>().entries.len();

        //println!("Selected Size: {}", size);

        for index in 0..size {

            let gen_index : GenerationalIndex;

            let select_color : (f32, f32, f32, f32);

            {
                let selected = input.0.get_map::<SelectedComponent>().entries[0].as_ref().unwrap();

                gen_index = selected.owned_entity;
                select_color = selected.value.selected_color;
            }

            {
                let mut color = input.0.get_mut::<ColorComponent>(&gen_index).unwrap();

                color.color = select_color;
            }
        }

        Ok(())
    }
}

pub struct DeselectSystem;

impl<'a> System<'a> for DeselectSystem {

    type SystemInput = &'a mut GameState;

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size = input.get_map::<SelectedComponent>().entries.len();

        //println!("Size of array to be cleared: {}", size);

        for index in 0..size {

            let idx : GenerationalIndex;

            {
                idx = input.get_map::<SelectedComponent>().entries[index].as_ref().unwrap().owned_entity;
            }

            let color = input.get_mut::<ColorComponent>(&idx).unwrap();

            color.color = (1.0, 1.0, 1.0, 1.0);

            input.remove_component::<SelectedComponent>(&idx);
        }

        Ok(())
    }
}

impl DeselectSystem {

    pub fn deselect_single(index : &GenerationalIndex, state : &mut GameState) {

        let color = state.get_mut::<ColorComponent>(index).unwrap();

        color.color = (1.0, 1.0, 1.0, 1.0);

        state.remove_component::<SelectedComponent>(index);
    }
}

pub struct FollowMouseSystem;

impl<'a> System<'a> for FollowMouseSystem {
    type SystemInput = (&'a mut GameState, &'a Vector2<f32>);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size = input.0.get_map::<SelectedComponent>().entries.len();
        //println!("Selected size: {}", size);

        let cursor_pos = Vector3::new(input.1.x, input.1.y, 0.0);

        let mut offset : Vector2<f32>;

        for index in 0..size {

            let idx : GenerationalIndex;

            {
                idx = input.0.get_map::<SelectedComponent>().entries[index].as_ref().unwrap().owned_entity;
            }
            offset = input.0.get::<SelectedComponent>(&idx).as_ref().unwrap().cursor_offset;

            let offset = Vector3::new(offset.x, offset.y, 0.0);

            {
                let position = input.0.get_mut::<PositionComponent>(&idx).unwrap();

                position.position = cursor_pos + offset;
            }

            {
                let collider = input.0.get_mut::<BoxCollider2DComponent>(&idx).unwrap();

                let collider_pos = cursor_pos + offset;

                let coords = Vector2::new(collider_pos.x, collider_pos.y);

                collider.position = coords;

                //println!("Moving entity: {} {} to position: {} {}", idx.index, idx.generation, coords.x, coords.y);
            }
        }

        Ok(())
    }
}

