use std::ffi::{CString, NulError};

pub trait MaterialKey {
    fn get_key(&self) -> &str;

    fn c_string(&self) -> Result<CString, NulError> {
        CString::new(self.get_key())
    }
}

impl MaterialKey for &str {
    fn get_key(&self) -> &str {
        self
    }
}
