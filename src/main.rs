extern crate failure;

// Mods
mod renderer;
mod application;

// Use
use failure::Error;
use application::Application;

fn main() -> Result<(),Error>{

    let application = Application::initialise_with_renderer()?;

    Ok(application.test_render()?)
}