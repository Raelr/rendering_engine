extern crate engine;

use engine::events::event::EventHandler;
// Use
use failure::Error;

fn main() -> Result<(),Error>{

    let application = engine::application::Application::initialise_with_renderer()?;

    let event = engine::events::key_event::KeyEvent::from_key_pressed(1, Some(1))?;

    let in_category = event.is_in_category(engine::events::event::EventCategory::EVENT_CATEGORY_KEYBOARD )?;

    let name = event.to_string()?;

    println!("{} {}", name, in_category);

    let event = engine::events::key_event::KeyEvent::from_key_released(1)?;

    let name = event.to_string()?;

    println!("{}", name);

    let event = engine::events::key_event::KeyEvent::from_key_typed(2)?;

    let name = event.to_string()?;

    println!("{}", name);

    Ok(application.test_render(engine::renderer::renderer_tests::TestType::RectangleElement)?)
}