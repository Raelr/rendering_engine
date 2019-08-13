use std::time::Instant;
use crate::renderer::shaders::shader_program::ShaderProgram;
use std::any::Any;
use nalgebra::Vector3;

pub mod system;
pub mod render_system;
pub mod texture_update_system;
pub mod position_update_system;

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

/// START OF TRANSFORM COMPONENTS ----------------------------------------------------------------->

/// POSITION
/// PositionComponent - Used to store the Entity's position. Currently represented as a
/// Vector3
pub struct PositionComponent {

    pub position : Vector3<f32>
}

impl Component for PositionComponent {}

/// VELOCITY
/// Used to store the velocity of the entity. Currently adds velocity to position every frame
/// SEE: position_update_system
pub struct VelocityComponent {

    pub velocity : Vector3<f32>
}

impl Component for VelocityComponent {}

/// ROTATION
/// Stores current object rotation.

pub struct RotationComponent {

    rotation : Vector3<f32>
}

impl Component for RotationComponent {}

/// END OF TRANFORM COMPONENTS -------------------------------------------------------------------->

/// COLOR
/// Need to abstract color object.
pub struct ColorComponent {

    pub color : (f32, f32, f32, f32)
}

impl Component for ColorComponent {}

/// RENDERER
/// Stores basic shader and renderer information.
/// Uses position and velocity to update itself.

pub struct RenderComponent {

    pub shader_program : gl::types::GLuint,
    pub vertex_array_object : gl::types::GLuint
}

impl Component for RenderComponent {}

/// TEXTURES
/// Stores a list of textures which can be overlaid on top of each other.

pub struct TextureMixComponent {

    pub textures : Vec<Texture>,
    pub opacity : gl::types::GLfloat
}

impl Component for TextureMixComponent {}

/// TEXTURE
/// A single texture object. Stores all texture data.

pub struct Texture {

    pub uniform_name : String,
    pub texture_id : gl::types::GLuint,
    pub number : i32,
    pub active_texture_enum : gl::types::GLenum
}

/// Stores details of textures which may or may not change each frame.

pub struct TextureUpdateComponent {

    pub opacity_change : gl::types::GLfloat
}

impl Component for TextureUpdateComponent {}

pub trait Component: Any + Sized {}