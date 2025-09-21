use assimp::Color3D;
use assimp_sys::AiTextureType;

use crate::{
    assimp::material_key::MaterialKey,
    framework::material::{AMaterial, MaterialConversionError},
};

impl MaterialKey for AiTextureType {
    fn get_key(&self) -> &str {
        match self {
            Self::Diffuse => "$clr.diffuse",
            Self::Ambient => "$clr.ambient",
            Self::Specular => "$clr.specular",
            x => todo!("MaterialKey for {:?} is not implemented", x),
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
    mat.get_material_color(key, 0, 0)
        .map_err(MaterialConversionError::AiError)
}
