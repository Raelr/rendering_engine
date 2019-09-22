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

        CheckBoxColliderSystem::check_sat_collision(input.0, input.1.clone());

        Ok(())
    }
}

impl CheckBoxColliderSystem {

    pub fn check_sat_collision(state: &mut GameState, collision_point : Vector2<f32>) {

        let colliders = state.get_map::<BoxCollider2DComponent>();
        let size = colliders.entries.len();

        for index in 0..size {

            let mut gen_idx = GenerationalIndex {generation: 0, index: 0};

            let mut collided = false;

            {
                let collider = colliders.entries[index].as_ref().unwrap();
                let corners = &collider.value.corners;
                let normals = CheckBoxColliderSystem::get_normals(&corners);
                CheckBoxColliderSystem::get_sat_projections(&vec![collision_point], &corners, &normals);
            }
        }
    }

    pub fn get_normals(corners : &Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {

        let normal_y = get_direction_2d(corners[1], corners[0]);
        let normal_y = Vector2::new(-normal_y.y, normal_y.x);

        let normal_x = get_direction_2d(corners[1], corners[3]);
        let normal_x = Vector2::new(normal_x.y, -normal_x.x);

//        println!("normalY = x: {} y: {}", normal_y.x, normal_y.y);
//        println!("normalX = x: {} y: {}", normal_x.x, normal_x.y);

        vec![normal_y, normal_x]
    }

    pub fn get_sat_projections(shape_one_crnrs : &Vec<Vector2<f32>>, shape_two_crnrs: &Vec<Vector2<f32>>, axes : &Vec<Vector2<f32>>) {

        println!("Mouse");
        let shape_one_projections =
            CheckBoxColliderSystem::get_crnr_projections(shape_one_crnrs, axes);

//        let min_projection = CheckBoxColliderSystem::get_min(&shape_one_projections);
//        let max_projection = CheckBoxColliderSystem::get_max(&shape_one_projections);
//        println!("Minimum projection =  x: {} y: {}", min_projection.x, min_projection.y);
//        println!("Maximum projection =  x: {} y: {}", max_projection.x, max_projection.y);

        println!("Box");
        let shape_two_projections =
            CheckBoxColliderSystem::get_crnr_projections(shape_two_crnrs, axes);



    }

    pub fn get_crnr_projections(corners: &Vec<Vector2<f32>>, axes : &Vec<Vector2<f32>>) -> Vec<Vector2<f32>>{

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

        println!("Minimum projection on first axis =  x: {} y: {}", min_projection_one.x, min_projection_one.y);
        println!("Maximum projection on first axis =  x: {} y: {}", max_projection_one.x, max_projection_one.y);
        println!("Minimum projection on second axis =  x: {} y: {}", min_projection_two.x, min_projection_two.y);
        println!("Maximum projection on second axis =  x: {} y: {}", max_projection_two.x, max_projection_two.y);

        projected_crnrs_x
    }

    pub fn get_min(corners : &Vec<Vector2<f32>>, axis: Vector2<f32>) -> Vector2<f32> {

        let mut minimum_corner = corners[0];
        let mut min_dot = minimum_corner.dot(&axis);

        for corner in corners {

            let dot = corner.dot(&axis);

            if dot < min_dot {
                minimum_corner = *corner;
                min_dot = dot;
            }
        }

        minimum_corner
    }

    pub fn get_max(corners : &Vec<Vector2<f32>>, axis: Vector2<f32>) -> Vector2<f32> {

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

        maximum_corner
    }
}

#[derive(Clone)]
pub struct SatCollisions {

    pub shape_one_min : Vector2<f32>,
    pub shape_two_min : Vector2<f32>,

    pub shape_one_max : Vector2<f32>,
    pub shape_two_max : Vector2<f32>,

    pub shape_one_projections : Vec<Vector2<f32>>,
    pub shape_two_projections : Vec<Vector2<f32>>
}

#[derive(Clone)]
pub struct SatShape {

    pub first_axis_min : Vector2<f32>,
    pub first_axis_max : Vector2<f32>,
    pub second_axis_min : Vector2<f32>,
    pub second_axis_max : Vector2<f32>,
}