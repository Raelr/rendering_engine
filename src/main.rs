extern crate engine;
// Use
use failure::Error;

fn main() -> Result<(),Error>{

    let mut application = engine::application::Application::new()?;

    application.run();

    Ok(())
}