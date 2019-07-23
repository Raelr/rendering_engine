use std::time::Instant;
use crate::renderer::shaders::shader_program::ShaderProgram;
use std::any::Any;

// Test struct. I know bools are bad, however i need to test this somehow.
pub struct PositionComponent {

    pub position : (f32, f32, f32),
    pub reversed : bool
}

impl Component for PositionComponent {}

//  Same as above.
pub struct ColorComponent {

    pub color : (f32, f32, f32, f32),
    pub use_vertex_colors : bool,
    pub use_position : bool
}

impl Component for ColorComponent {}

pub struct TimerComponent {

    pub now : Instant
}

impl Component for TimerComponent {}

pub struct RenderComponent {

    pub shader_program : ShaderProgram,
}

pub struct ShaderComponent {


}

impl Component for RenderComponent {}

pub trait Component: Any + Sized {}