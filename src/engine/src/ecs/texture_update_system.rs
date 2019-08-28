use crate::ecs::{TextureMixComponent, TextureUpdateComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndex};
use crate::game_state::GameState;
use crate::ecs::system::System;

pub struct TextureUpdateSystem;

impl<'a> System<'a> for TextureUpdateSystem {

    type SystemInput = (&'a mut GameState);

    fn run(input: Self::SystemInput) -> Result<(), Error> {

        let size = input.get_map::<TextureUpdateComponent>().entries.len();

        let mut opacity: gl::types::GLfloat;

        for index in 0..size {

            let generation;

            {
                let updates = &mut input.get_map_mut::<TextureUpdateComponent>();

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