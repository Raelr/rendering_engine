use crate::ecs::{RenderComponentTemp, PositionComponent, ColorComponent, TextureMixComponent, TextureUpdateComponent};
use failure::Error;
use crate::generational_index::generational_index::{GenerationalIndexArray, GenerationalIndex};
use std::ffi::CString;
use crate::game_state::GameState;
use std::borrow::BorrowMut;
use cgmath::{vec3, Matrix4, Matrix};

pub trait System<'a> {

    type SystemInput;

    fn run(&self, input : Self::SystemInput) -> Result<(), Error>;
}