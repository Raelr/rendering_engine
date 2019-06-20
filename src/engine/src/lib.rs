extern crate strum;
#[macro_use] extern crate strum_macros;
#[macro_use] extern crate bitflags;

pub mod application;
pub mod renderer;
pub mod events;

#[macro_use]
macro_rules! bit {
    ($value:expr) => {1 << $value};
}