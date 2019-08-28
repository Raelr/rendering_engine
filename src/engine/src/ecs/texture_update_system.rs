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


            let gen_index : GenerationalIndex;

            {
                let updates = &mut input.get_map_mut::<TextureUpdateComponent>();

                if let Some(change) = updates.entries[index].as_mut() {
                    opacity = change.value.opacity_change;
                    gen_index = change.owned_entity;
                    change.value.opacity_change = 0.0;
                } else {
                    continue
                }
            }



            let textures = input.get_map_mut::<TextureMixComponent>();

            if let Some(texture) = textures.get_mut(&gen_index) {
                texture.opacity += opacity
            }
        }
        Ok(())
    }
}