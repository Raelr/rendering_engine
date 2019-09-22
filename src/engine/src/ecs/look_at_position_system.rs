use crate::ecs::system::System;
use crate::ecs::{PositionComponent, VelocityComponent, BoxCollider2DComponent, RotationComponent, RotationUpdateComponent, LookAtPositionComponent, SelectedComponent, RenderComponent, ScaleComponent, ColorComponent, TextureMixComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndex, GenerationalIndexArray};
use crate::game_state::GameState;
use crate::utilities::vector_utils;
use nalgebra::{Vector3, Vector2};
use crate::utilities::vector_utils::{get_box_corners, get_direction_2d};

pub struct LookAtPositionSystem;

impl<'a> System<'a> for LookAtPositionSystem {
    type SystemInput = (&'a mut GameState);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size = input.get_map::<LookAtPositionComponent>().entries.len();

        for index in 0..size {

            let component = input.get_map::<LookAtPositionComponent>().entries[index].as_ref().unwrap();
            let gen_idx = component.owned_entity.clone();

            let selected : bool = input.get::<SelectedComponent>(&gen_idx).is_some();

            if selected {

                let focus_position = component.value.focus_point.clone();
                let component_position: Vector2<f32>;

                {
                    let position = input.get::<PositionComponent>(&gen_idx).unwrap();
                    component_position = Vector2::new(position.position.x, position.position.y);
                }

                let angle_change = vector_utils::get_rotation_angle_2(component_position, focus_position);

                {
                    let rotation = input.get_mut::<RotationComponent>(&gen_idx).unwrap();
                    rotation.rotation = Vector3::new(0.0, 0.0, angle_change);
                }

                {
                    let collider = input.get_mut::<BoxCollider2DComponent>(&gen_idx).unwrap();

                    let corners = vector_utils::get_box_corners(collider.position, collider.size);

                    let corners = vector_utils::get_rotated_corners(corners, collider.position, angle_change);

                    collider.corners = corners;
                }
            }
        }
        Ok(())
    }
}

pub struct UpdateFocusPointSystem;

impl<'a> System<'a> for UpdateFocusPointSystem {

    type SystemInput = (&'a mut GameState, Vector2<f32>);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size  = input.0.get_map::<LookAtPositionComponent>().entries.len();

        for index in 0..size {

            let mut look_at = input.0.get_map_mut::<LookAtPositionComponent>().entries[index].as_mut().unwrap();

            look_at.value.focus_point = input.1;
        }

        Ok(())
    }
}