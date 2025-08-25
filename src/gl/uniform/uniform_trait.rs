use crate::gl::{GLfloat, glUniform1f, uniform::UniformLocation};

pub trait Uniform {
    type Options;

    fn set(&self, options: Option<Self::Options>, location: &UniformLocation);
}

impl Uniform for GLfloat {
    type Options = ();

    fn set(&self, _options: Option<Self::Options>, location: &UniformLocation) {
        let UniformLocation(location) = *location;
        unsafe {
            glUniform1f(location, *self);
        }
    }
}
