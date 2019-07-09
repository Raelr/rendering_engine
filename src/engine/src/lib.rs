#![feature(duration_float)]
extern crate strum;
extern crate failure;

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
pub mod application;
pub mod renderer;















