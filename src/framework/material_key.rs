use std::ffi::{CString, NulError};

pub trait MaterialKey {
    fn get_key(&self) -> &str;

    fn c_string(&self) -> Result<CString, NulError> {
        CString::new(self.get_key())
    }
}

#[derive(Debug, Clone)]
pub struct MatKey<'a>(pub &'a str);

impl MaterialKey for MatKey<'_> {
    fn get_key(&self) -> &str {
        self.0
    }
}
