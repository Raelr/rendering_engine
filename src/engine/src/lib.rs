#![feature(duration_float)]
#[macro_use]
extern crate strum;
extern crate failure;
extern crate anymap;
extern crate image;
extern crate cgmath;

use cgmath::*;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

#[macro_use]
macro_rules! bit {
    ($value:expr) => {1 << $value};
}


#[macro_use] pub mod events;
#[macro_use] pub mod window;
pub mod generational_index;
pub mod platform;
#[macro_use]pub mod renderer;
#[macro_use]pub mod ecs;
pub mod game_state;
pub mod application;
















