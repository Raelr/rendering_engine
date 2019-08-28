use failure::Error;
use crate::ecs::Component;
use crate::generational_index::generational_index::GenerationalIndex;
use crate::game_state::GameState;
use crate::ecs::*;

pub trait System<'a> {

    type SystemInput;

    fn run(input : Self::SystemInput) -> Result<(), Error>;
}