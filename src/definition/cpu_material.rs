use crate::definition::*;

///
/// A CPU-side version of a material (for example [phong material](crate::PhongMaterial)).
/// Can be constructed manually or loaded via [io](crate::io).
///
pub struct CPUMaterial {
    pub name: String,
    pub color: Option<(f32, f32, f32, f32)>,
    pub texture_image: Option<CPUTexture<u8>>,
    pub diffuse_intensity: Option<f32>,
    pub specular_intensity: Option<f32>,
    pub specular_power: Option<f32>,
}

impl Default for CPUMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color: Some((1.0, 1.0, 1.0, 1.0)),
            texture_image: None,
            diffuse_intensity: Some(0.5),
            specular_intensity: Some(0.2),
            specular_power: Some(6.0),
        }
    }
}
