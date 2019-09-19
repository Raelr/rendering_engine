use crate::ecs::system::System;
use crate::ecs::{PositionComponent, VelocityComponent, BoxCollider2DComponent, RotationComponent, RotationUpdateComponent, LookAtPositionComponent, SelectedComponent, RenderComponent, ScaleComponent, ColorComponent, TextureMixComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndex, GenerationalIndexArray};
use crate::game_state::GameState;
use crate::utilities::vector_utils;
use nalgebra::{Vector3, Vector2};
use crate::utilities::vector_utils::get_box_corners;

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

                let new_position: Vector2<f32>;

                {
                    let collider = input.get_mut::<BoxCollider2DComponent>(&gen_idx).unwrap();

                    let mut corners = collider.corners.clone();
                    let mut rotated_corners : Vec<Vector2<f32>> = Vec::new();

                    for corner in corners {
                        let coords =  vector_utils::get_point_after_rotation(
                            Vector2::new(corner.x, corner.y), collider.position, angle_change);
                        rotated_corners.push(coords);
                    }

                    use std::f32::MIN;

                    let mut max_x = MIN;

                    let max_corner = {

                        let mut chosen : Vector2<f32> = Vector2::new(0.0,0.0);
                        for corner in &rotated_corners {

                            if corner.x > max_x {
                                chosen = corner.clone_owned();
                                max_x = corner.x;
                            }
                        }

                        chosen
                    };

                    new_position = max_corner;

                    println!("Max: {}:{}", new_position.x, new_position.y);

                    collider.corners = rotated_corners;
                }


                let scale = Vector3::new(10.0, 10.0, 10.0);
                let corners = get_box_corners(new_position, Vector2::new(scale.x, scale.y));

                let entity = GameState::create_entity(input)
                    .with(RenderComponent {shader_program : triangle_render!(), vertex_array_object : quad!()})
                    .with(PositionComponent {position: Vector3::new(new_position.x, new_position.y, 0.0)})
                    .with(RotationComponent { rotation: Vector3::new(0.0, 0.0, 0.0) })
                    .with(ScaleComponent {scale})
                    .with(ColorComponent {color : (1.0, 1.0, 1.0, 0.0) })
                    .with(VelocityComponent {velocity : Vector3::new(0.0, 0.0, 0.0)})
                    .with(BoxCollider2DComponent {position: Vector2::new(new_position.x - scale.x, new_position.y - scale.y), size : Vector2::new(scale.x * 2.0, scale.y * 2.0), corners})
                    .build();
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