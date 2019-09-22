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

            CheckBoxColliderSystem::check_sat_collision(input.0, input.1.clone());
//                let collider_entry = input.0.get_map_mut::<BoxCollider2DComponent>().entries[index].as_mut().unwrap();
//
//                let position = collider_entry.value.position;
//                let size = collider_entry.value.size;
//                let mouse_coordinates = input.1;
//                let corners = &collider_entry.value.corners;
//
//                let leftmost_corner = corners[3];
//
//                let collision_x = leftmost_corner.x + size.x >= mouse_coordinates.x && mouse_coordinates.x >= leftmost_corner.x;
//
//                let collision_y = leftmost_corner.y + size.y >= mouse_coordinates.y && mouse_coordinates.y >= leftmost_corner.y;
//
//                collided = collision_x && collision_y;
        }

        Ok(())
    }
}

impl CheckBoxColliderSystem {

    pub fn check_sat_collision(state: &mut GameState, collision_point : Vector2<f32>) {

        let size = state.get_map_mut::<BoxCollider2DComponent>().entries.len();

        for index in 0..size {

            let colliders = state.get_map_mut::<BoxCollider2DComponent>();

            let mut gen_idx = GenerationalIndex {generation: 0, index: 0};

            let mut collided = false;

            let offset: Vector2<f32>;

            let position : Vector2<f32>;

            {
                let collider = colliders.entries[index].as_ref().unwrap();
                let position = collider.value.position.clone();
                let corners = &collider.value.corners;
                let normals = CheckBoxColliderSystem::get_normals(&corners);

                let collisions = CheckBoxColliderSystem::get_sat_projections(&vec![collision_point], &corners, &normals);

                let collision_one =
                    collisions.shape_one.first_axis_max > collisions.shape_two.first_axis_min
                        && collisions.shape_one.first_axis_min < collisions.shape_two.first_axis_max;
                let collision_two = collisions.shape_one.second_axis_max > collisions.shape_two.second_axis_min
                    && collisions.shape_one.second_axis_min < collisions.shape_two.second_axis_max;

                collided = collision_one && collision_two;

                gen_idx = collider.owned_entity;

                let heading = Vector2::new(collision_point.x, collision_point.y);
                let distance = Vector2::magnitude(&heading);
                let direction = heading / distance;

                offset = position - direction * distance;
            }

            let selected = match state.get_mut::<SelectedComponent>(&gen_idx) {
                Some(_val) => true,
                None => false
            };

            selection_system::DeselectSystem::run(state);

            if collided {
                {
                    let origin_color = state.get::<ColorComponent>(&gen_idx).unwrap().color.clone();
                    state.add_component_to(SelectedComponent { selected_color: (0.7, 0.7, 0.7, 0.5), origin_color, cursor_offset: offset}, &gen_idx);
                    break;
                }
            }
        }
    }

    pub fn get_normals(corners : &Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {

        let normal_y = get_direction_2d(corners[1], corners[0]);
        let normal_y = Vector2::new(-normal_y.y, normal_y.x);

        let normal_x = get_direction_2d(corners[1], corners[3]);
        let normal_x = Vector2::new(normal_x.y, -normal_x.x);

        vec![normal_y, normal_x]
    }

    pub fn get_sat_projections(shape_one_crnrs : &Vec<Vector2<f32>>, shape_two_crnrs: &Vec<Vector2<f32>>, axes : &Vec<Vector2<f32>>) -> SatCollisions{

        println!("Mouse");
        let shape_one =
            CheckBoxColliderSystem::get_crnr_projections(shape_one_crnrs, axes);

        println!("Box");
        let shape_two =
            CheckBoxColliderSystem::get_crnr_projections(shape_two_crnrs, axes);

        let collisions = SatCollisions {
            shape_one,
            shape_two
        };

        collisions
    }

    pub fn get_crnr_projections(corners: &Vec<Vector2<f32>>, axes : &Vec<Vector2<f32>>) -> SatShape{

        let mut projected_crnrs_x = Vec::new();
        let mut projected_crnrs_y = Vec::new();

        for corner in corners {
            projected_crnrs_x.push(get_projection_2d(corner, &axes[0]));
            projected_crnrs_y.push(get_projection_2d(corner, &axes[1]));
        }

        let min_projection_one = CheckBoxColliderSystem::get_min(&projected_crnrs_x, axes[0]);
        let max_projection_one = CheckBoxColliderSystem::get_max(&projected_crnrs_x, axes[0]);
        let min_projection_two = CheckBoxColliderSystem::get_min(&projected_crnrs_y, axes[1]);
        let max_projection_two = CheckBoxColliderSystem::get_max(&projected_crnrs_y, axes[1]);

        println!("Minimum projection on first axis = {}", min_projection_one);
        println!("Maximum projection on first axis = {}", max_projection_one);
        println!("Minimum projection on second axis = {}", min_projection_two);
        println!("Maximum projection on second axis = {}", max_projection_two);

        let shape = SatShape {
            first_axis_min: min_projection_one,
            first_axis_max: max_projection_one,
            second_axis_min: min_projection_two,
            second_axis_max: max_projection_two
        };

        shape
    }

    pub fn get_min(corners : &Vec<Vector2<f32>>, axis: Vector2<f32>) -> f32 {

        let mut minimum_corner = corners[0];
        let mut min_dot = minimum_corner.dot(&axis);

        for corner in corners {

            let dot = corner.dot(&axis);

//            println!("min dot: {}", min_dot);
//            println!("dot: {}", dot);

            if dot < min_dot {
                minimum_corner = *corner;
                min_dot = dot;
            }
        }

        min_dot
    }

    pub fn get_max(corners : &Vec<Vector2<f32>>, axis: Vector2<f32>) -> f32 {

        use std::f32::MIN;

        let mut maximum_corner = corners[0];
        let mut max_dot = maximum_corner.dot(&axis);

        for corner in corners {

            let dot  = corner.dot(&axis);
            if dot > max_dot {
                maximum_corner = *corner;
                max_dot = dot;
            }
        }

        max_dot
    }
}

#[derive(Clone)]
pub struct SatCollisions {

    pub shape_one : SatShape,
    pub shape_two : SatShape,
}

#[derive(Clone)]
pub struct SatShape {

    pub first_axis_min : f32,
    pub first_axis_max : f32,
    pub second_axis_min : f32,
    pub second_axis_max : f32,
}