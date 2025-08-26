use std::collections::HashMap;

use crate::math::vec4::Vec4;

use super::texture::Texture2D;

pub struct Material {
    textures: HashMap<String, Texture2D>,
    k_amb: Vec4<f32>,
    k_diff: Vec4<f32>,
    k_spec: Vec4<f32>,
    n_spec: f32,
}
