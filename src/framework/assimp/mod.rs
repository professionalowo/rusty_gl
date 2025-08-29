use std::{
    ffi::{CStr, CString, c_char, c_uint},
    fmt,
    ops::Deref,
};

use assimp::Color3D;
use assimp_sys::{
    AiColor4D, AiString, AiTextureType, aiGetMaterialColor, aiGetMaterialString,
    aiGetMaterialTexture, aiGetMaterialTextureCount,
};

pub struct AMaterial<'a>(pub assimp::Material<'a>);

impl<'a> Deref for AMaterial<'a> {
    type Target = assimp::Material<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum AiError {
    Failure,
    OutOfMemory,
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure => writeln!(f, "Failure"),
            Self::OutOfMemory => writeln!(f, "Out of memory"),
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
        match unsafe {
            aiGetMaterialColor(self.to_raw(), key.as_ptr(), property_type, index, &mut c)
        } {
            assimp_sys::AiReturn::Success => {
                let AiColor4D { r, g, b, .. } = c;
                Ok(Color3D::new(r, g, b))
            }
            assimp_sys::AiReturn::Failure => Err(AiError::Failure),
            assimp_sys::AiReturn::OutOfMemory => Err(AiError::OutOfMemory),
        }
    }

    pub fn get_texture(&self, texture_type: AiTextureType, index: u32) -> Result<String, AiError> {
        let mut path = AiString::default();
        match unsafe {
            aiGetMaterialTexture(
                self.to_raw(),
                texture_type,
                index,
                &mut path,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        } {
            assimp_sys::AiReturn::Success => Ok(unsafe { AString::from_ai_string(&path) }),
            assimp_sys::AiReturn::Failure => Err(AiError::Failure),
            assimp_sys::AiReturn::OutOfMemory => Err(AiError::OutOfMemory),
        }
    }

    pub fn get_material_string(
        &self,
        key: CString,
        property_type: u32,
        index: u32,
    ) -> Result<String, AiError> {
        let mut str = AiString::default();

        match unsafe {
            aiGetMaterialString(self.to_raw(), key.as_ptr(), property_type, index, &mut str)
        } {
            assimp_sys::AiReturn::Success => Ok(unsafe { AString::from_ai_string(&str) }),
            assimp_sys::AiReturn::Failure => Err(AiError::Failure),
            assimp_sys::AiReturn::OutOfMemory => Err(AiError::OutOfMemory),
        }
    }

    pub fn get_texture_count(&self, texture_type: AiTextureType) -> u32 {
        let Self(material) = self;
        unsafe { aiGetMaterialTextureCount(material.to_raw(), texture_type) }
    }
}

#[repr(C)]
struct AString {
    pub length: c_uint,
    pub data: [c_char; 1024],
}

impl AString {
    unsafe fn from_ai_string(ptr: *const AiString) -> String {
        let s = unsafe { &*(ptr as *const Self) };

        let len = s.length as usize;
        let bytes = &s.data[..len];

        unsafe { CStr::from_ptr(bytes.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    }
}
