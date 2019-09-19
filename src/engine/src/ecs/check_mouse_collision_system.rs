use crate::ecs::system::System;
use crate::ecs::{BoxCollider2DComponent, SelectedComponent, ColorComponent};
use crate::generational_index::generational_index::{GenerationalIndex, GenerationalIndexArray};
use nalgebra::{Vector2};
use failure::Error;
use crate::game_state::GameState;
use crate::ecs::selection_system;
use crate::utilities::vector_utils::{get_direction_2d, get_projection_2d};

pub struct CheckBoxColliderSystem;

impl<'a> System<'a> for CheckBoxColliderSystem {

    type SystemInput = (&'a mut GameState,
                        &'a Vector2<f32>);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size = input.0.get_map::<BoxCollider2DComponent>().entries.len();

        //println!("size: {}", size);

        for index in 0..size {

            let mut gen_idx : GenerationalIndex = GenerationalIndex{index: 0, generation: 0};

            let collided;

            let offset : Vector2<f32>;

            {
                let collider_entry = input.0.get_map_mut::<BoxCollider2DComponent>().entries[index].as_mut().unwrap();

                let position = collider_entry.value.position;
                let size = collider_entry.value.size;
                let mouse_coordinates = input.1;
                let corners = &collider_entry.value.corners;

                let leftmost_corner = corners[3];

                let collision_x = leftmost_corner.x + size.x >= mouse_coordinates.x && mouse_coordinates.x >= leftmost_corner.x;

                let collision_y = leftmost_corner.y + size.y >= mouse_coordinates.y && mouse_coordinates.y >= leftmost_corner.y;

                collided = collision_x && collision_y;

                gen_idx = collider_entry.owned_entity;

                let heading = Vector2::new(mouse_coordinates.x, mouse_coordinates.y);
                let distance = Vector2::magnitude(&heading);
                let direction = heading / distance;

                offset = position - direction * distance;
            }

            let selected = match input.0.get::<SelectedComponent>(&gen_idx) {
                Some(_val) => true,
                None => false
            };

            // TODO: FIX ISSUES WITH DESELECTION SYSTEM - CAUSING INDEXING ERRORS WHEN TRYING TO REMOVE INDIVIDUALLY

            selection_system::DeselectSystem::run(input.0);
            if collided {
                {
                    let origin_color = input.0.get::<ColorComponent>(&gen_idx).unwrap().color.clone();
                    input.0.add_component_to(SelectedComponent { selected_color: (0.7, 0.7, 0.7, 0.5), origin_color, cursor_offset: offset}, &gen_idx);
                    break;
                }
            }
        }

        CheckBoxColliderSystem::check_sat_collision(input.0, input.1);

        Ok(())
    }
}

impl CheckBoxColliderSystem {

    pub fn check_sat_collision(state: &mut GameState, collision_point : &Vector2<f32>) {

        let colliders = state.get_map::<BoxCollider2DComponent>();
        let size = colliders.entries.len();

        for index in 0..size {

            let mut gen_idx = GenerationalIndex {generation: 0, index: 0};

            let mut collided = false;

            {
                let collider = colliders.entries[index].as_ref().unwrap();
                let corners = &collider.value.corners;
                let normals = CheckBoxColliderSystem::get_normals(&corners);
                let projectionx_topright = get_projection_2d(corners[0], normals[0]);
                let projectiony_topright = get_projection_2d(corners[0], normals[1]);
            }
        }
    }

    pub fn get_normals(corners : &Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {

        let normal_y = get_direction_2d(corners[1], corners[0]);
        let normal_y = Vector2::new(-normal_y.y, normal_y.x);

        let normal_x = get_direction_2d(corners[1], corners[3]);
        let normal_x = Vector2::new(-normal_x.y, normal_x.x);

        let negative_x_normal = -normal_x;

        let negative_y_normal = -normal_y;

        println!("normalY = x: {} y: {}", normal_y.x, normal_y.y);
        println!("normalX = x: {} y: {}", normal_x.x, normal_x.y);
        println!("normalNegativeY = x: {} y: {}", -normal_y.x, -normal_y.y);
        println!("normalNegativeX = x: {} y: {}", -normal_x.x, -normal_x.y);

        vec![normal_y, normal_x, negative_y_normal, negative_x_normal]
    }
}