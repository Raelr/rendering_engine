use crate::ecs::{RenderComponentTemp, PositionComponent, ColorComponent, TextureMixComponent, TextureUpdateComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use std::ffi::CString;
use crate::game_state::GameState;
use std::borrow::BorrowMut;
use cgmath::{vec3, Matrix4, Matrix};
use crate::ecs::system::System;

pub struct TextureUpdateSystem;

impl<'a> System<'a> for TextureUpdateSystem {

    type SystemInput = (&'a mut GameState);

    fn run(&self, input: Self::SystemInput) -> Result<(), Error> {

        let size = input.get_map::<TextureUpdateComponent>().entries.len();

        let mut opacity: gl::types::GLfloat = 0.0;

        for index in 0..size {

            let mut generation = 0;

            {
                let mut updates = &mut input.get_map_mut::<TextureUpdateComponent>();

                if let Some(change) = updates.entries[index].as_mut() {
                    opacity = change.value.opacity_change;
                    generation = change.generation;
                    change.value.opacity_change = 0.0;
                } else {
                    continue
                }
            }

            let gen_index = GenerationalIndex { index, generation};

            let textures = input.get_map_mut::<TextureMixComponent>();

            if let Some(texture) = textures.get_mut(&gen_index) {
                texture.opacity += opacity
            }
        }
        Ok(())
    }
}