use std::time::Instant;
use crate::renderer::shaders::shader_program::ShaderProgram;
use std::any::Any;

pub mod system;
pub mod render_system;
pub mod texture_update_system;

#[macro_export]
// Macro for creating a key typed event.
macro_rules! texture { ($path:expr, $number:expr, $enum:expr, $name:expr) => {{

        use image::GenericImageView;
        use std::os::raw::c_void;

        let mut texture_id : gl::types::GLuint = 0;

        unsafe {

            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            let image  = image::open($path)?;

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width() as i32, image.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, image.to_rgba().into_raw().as_ptr() as *const c_void);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture { uniform_name: $name, texture_id, number: $number, active_texture_enum: $enum }
    }};
}
// Test struct. I know bools are bad, however i need to test this somehow.
pub struct PositionComponent {

    pub position : (f32, f32, f32),
    pub velocity : (f32, f32, f32)
}

impl Component for PositionComponent {}

//  Same as above.
pub struct ColorComponent {

    pub color : (f32, f32, f32, f32)
}

impl Component for ColorComponent {}

pub struct TimerComponent {

    pub now : Instant
}

impl Component for TimerComponent {}

pub struct RenderComponent {

    pub shader_program : ShaderProgram,
}

impl Component for RenderComponent {}

pub struct RenderComponentTemp {

    pub shader_program : gl::types::GLuint,
    pub vertex_array_object : gl::types::GLuint
}

impl Component for RenderComponentTemp {}

pub struct TextureMixComponent {

    pub textures : Vec<Texture>,
    pub opacity : gl::types::GLfloat
}

impl Component for TextureMixComponent {}

pub struct Texture {

    pub uniform_name : String,
    pub texture_id : gl::types::GLuint,
    pub number : i32,
    pub active_texture_enum : gl::types::GLenum
}

pub struct TextureUpdateComponent {

    pub opacity_change : gl::types::GLfloat
}

impl Component for TextureUpdateComponent {}

pub struct RotationComponent {

    rotation : (f32, f32, f32),
    rotation_update : (f32, f32, f32)
}

impl Component for RotationComponent {}

pub trait Component: Any + Sized {}