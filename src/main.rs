extern crate engine;
// Use
use failure::Error;

fn main() -> Result<(),Error>{

    let application = engine::application::Application::initialise_with_renderer()?;

    Ok(application.test_render(engine::renderer::renderer_tests::TestType::RectangleElement)?)
}