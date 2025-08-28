use std::{ffi::{c_char, c_uint, CString}, fmt};

use assimp::Color3D;
use assimp_sys::{AiColor4D, aiGetMaterialColor};

pub struct AMaterial<'a>(pub assimp::Material<'a>);

#[derive(Debug)]
pub enum AiError {
    Failure,
    OutOfMemory,
}

impl fmt::Display for AiError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self{
			Self::Failure => writeln!(f,"Failure"),
			Self::OutOfMemory => writeln!(f,"Out of memory")
		}
	}
}

impl<'a> AMaterial<'a> {
    pub fn get_material_color(
        &self,
        key: CString,
        property_type: c_uint,
        index: c_uint,
    ) -> Result<Color3D, AiError> {
        let mut c = AiColor4D {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        };
        let Self(material) = self;
        match unsafe { aiGetMaterialColor(material.to_raw(), key.as_ptr(), property_type, index, &mut c) } {
            assimp_sys::AiReturn::Success => {
                let AiColor4D { r, g, b, .. } = c;
                Ok(Color3D::new(r, g, b))
            }
            assimp_sys::AiReturn::Failure => Err(AiError::Failure),
            assimp_sys::AiReturn::OutOfMemory => Err(AiError::OutOfMemory),
        }
    }
}
