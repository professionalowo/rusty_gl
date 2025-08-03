use crate::{
    gl::{glGetUniformLocation, glUniformMatrix3fv},
    math::mat3::Mat3,
};

pub fn mat3f(location: i32, transpose: bool, matrix: Mat3<f32>) {
    let cols = matrix.cols();
    let value = cols.as_ptr() as *const f32;
    println!("{:?}", cols);
    unsafe {
        glUniformMatrix3fv(location, 1, u8::from(transpose), value);
    }
}

pub fn get_location(program: u32, name: &str) -> i32 {
    unsafe { glGetUniformLocation(program, name.as_ptr() as *const i8) }
}
