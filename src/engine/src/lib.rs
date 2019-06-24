extern crate strum;
extern crate failure;
#[macro_use]extern crate strum_macros;
#[macro_use]extern crate bitflags;

////////////////////////////////////
//           M A C R O S          //
////////////////////////////////////

#[macro_use]
macro_rules! bit {
    ($value:expr) => {1 << $value};
}

#[macro_use]
pub mod events;
pub mod window;
pub mod application;
pub mod renderer;















