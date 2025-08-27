use std::collections::HashMap;

use crate::math::vec4::Vec4;

use super::texture::Texture2D;

#[derive(Debug)]
pub struct Material {
    pub textures: HashMap<String, Texture2D>,
    pub k_amb: Vec4<f32>,
    pub k_diff: Vec4<f32>,
    pub k_spec: Vec4<f32>,
    pub n_spec: f32,
}
