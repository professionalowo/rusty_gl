use std::ffi::{CString, NulError};

use crate::framework::material::{AMaterial, MaterialConversionError};
use assimp::Color3D;
use assimp_sys::AiTextureType;
pub(super) trait MaterialKey {
    fn get_key(&self) -> &'static str;

    fn c_string(&self) -> Result<CString, NulError> {
        CString::new(self.get_key())
    }
}

impl MaterialKey for &'static str {
    fn get_key(&self) -> &'static str {
        self
    }
}

impl MaterialKey for AiTextureType {
    fn get_key(&self) -> &'static str {
        match self {
            Self::Diffuse => "$clr.diffuse",
            Self::Ambient => "$clr.ambient",
            Self::Specular => "$clr.specular",
            _ => "",
        }
    }
}

pub(super) fn material_color<K>(
    mat: &AMaterial<'_>,
    key: K,
) -> Result<Color3D, MaterialConversionError>
where
    K: MaterialKey,
{
    mat.get_material_color(
        key.c_string().map_err(MaterialConversionError::NulError)?,
        0,
        0,
    )
    .map_err(MaterialConversionError::AiError)
}
