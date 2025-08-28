use crate::gl::{self, GLfloat, uniform::UniformLocation};

pub trait Uniform {
    type Options;

    fn set(&self, options: Option<Self::Options>, location: &UniformLocation);
}

impl Uniform for GLfloat {
    type Options = ();

    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        unsafe {
            gl::glUniform1f(*location, *self);
        }
    }
}
