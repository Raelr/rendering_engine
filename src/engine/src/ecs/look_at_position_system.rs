use crate::ecs::system::System;
use crate::ecs::{PositionComponent, VelocityComponent, BoxCollider2DComponent, RotationComponent, RotationUpdateComponent, LookAtPositionComponent, SelectedComponent, RenderComponent, ScaleComponent, ColorComponent, TextureMixComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndex, GenerationalIndexArray};
use crate::game_state::GameState;
use crate::utilities::vector_utils;
use nalgebra::{Vector3, Vector2};

pub struct LookAtPositionSystem;

impl<'a> System<'a> for LookAtPositionSystem {
    type SystemInput = (&'a mut GameState);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size = input.get_map::<LookAtPositionComponent>().entries.len();

        for index in 0..size {

            let component = input.get_map::<LookAtPositionComponent>().entries[index].as_ref().unwrap();
            let gen_idx = component.owned_entity.clone();

            let selected = match input.get::<SelectedComponent>(&gen_idx) {
                Some(c) => true,
                None => false
            };

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

                let newPosition : Vector3<f32>;

                {
                    let collider = input.get_mut::<BoxCollider2DComponent>(&gen_idx).unwrap();

                    let mut x_max = collider.position.x + (collider.size.x * 0.5);
                    let mut y_max = collider.position.y + (collider.size.y * 0.5);

                    let mut x_min = collider.position.x - (collider.size.x * 0.5);
                    let mut y_min = collider.position.y - (collider.size.y * 0.5);

                    let max = vector_utils::get_point_after_rotation(Vector2::new(x_max, y_max), angle_change);
                    let min = vector_utils::get_point_after_rotation(Vector2::new(x_min, y_min), angle_change);

                    println!("Max: x: {} y: {} Min: x: {} y: {}", max.x, max.y, min.x, min.y);

                    newPosition = Vector3::new(max.x - collider.size.x, max.y - collider.size.y, 0.0);

                }

                let scale = Vector3::new(10.0, 10.0, 10.0);

                let entity = GameState::create_entity(input)
                    .with(RenderComponent {shader_program : triangle_render!(), vertex_array_object : quad!()})
                    .with(PositionComponent {position: newPosition})
                    .with(RotationComponent { rotation: Vector3::new(0.0, 0.0, 0.0) })
                    .with(ScaleComponent {scale})
                    .with(ColorComponent {color : (1.0, 1.0, 1.0, 0.0) })
                    .with(VelocityComponent {velocity : Vector3::new(0.0, 0.0, 0.0)})
                    .with(BoxCollider2DComponent {position: Vector2::new(newPosition.x - scale.x, newPosition.y - scale.y), size : Vector2::new(scale.x * 2.0, scale.y * 2.0)})
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