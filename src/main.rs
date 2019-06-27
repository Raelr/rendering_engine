extern crate engine;
// Use
use failure::Error;

fn main() -> Result<(),Error>{

    engine::application::run();

    Ok(())
}