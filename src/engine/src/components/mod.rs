// Test struct. I know bools are bad, however i need to test this somehow.
pub struct PositionComponent {

    position : (f32, f32, f32),
    reversed : bool
}
//  Same as above.
pub struct ColorComponent {

    color : (f32, f32, f32, f32),
    use_vertex_colors : bool,
}