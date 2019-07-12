use std::time::Instant;

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