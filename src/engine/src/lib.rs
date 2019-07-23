#![feature(duration_float)]
extern crate strum;
extern crate failure;
extern crate anymap;
extern crate image;
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
















