use std::time::Instant;
use crate::renderer::shaders::shader_program::ShaderProgram;

pub enum Components {

    Position (PositionComponent),
    Color (ColorComponent),
    Timer (TimerComponent)
}

// Test struct. I know bools are bad, however i need to test this somehow.
pub struct PositionComponent {

    pub position : (f32, f32, f32),
    pub reversed : bool
}

//  Same as above.
pub struct ColorComponent {

    pub color : (f32, f32, f32, f32),
    pub use_vertex_colors : bool,
    pub use_position : bool
}

pub struct TimerComponent {

    pub now : Instant
}

pub struct RenderComponent {

    pub shader_program : ShaderProgram,
}