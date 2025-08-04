use crate::{
    gl::{glGetUniformLocation, glUniformMatrix3fv},
    math::mat3::Mat3,
};

pub fn mat3f(location: i32, transpose: bool, matrix: Mat3<f32>) {
    let cols = matrix.cols();
    let value = cols.as_ptr() as *const f32;
    unsafe {
        glUniformMatrix3fv(location, 1, u8::from(transpose), value);
    }
}

//TODO: always seems to return -1, even if the uniform exists
pub fn get_location(program: u32, name: impl AsRef<str>) -> i32 {
    let name = name.as_ref();
    unsafe { glGetUniformLocation(program, name.as_ptr() as *const i8) }
}
