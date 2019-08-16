use crate::ecs::{RenderComponent, PositionComponent, ColorComponent, TextureMixComponent, TextureUpdateComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use std::ffi::CString;
use crate::game_state::GameState;
use std::borrow::BorrowMut;

pub trait System<'a> {

    type SystemInput;

    fn run(input : Self::SystemInput) -> Result<(), Error>;
}