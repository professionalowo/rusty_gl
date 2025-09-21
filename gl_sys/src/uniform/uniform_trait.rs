use crate::{
    bindings::{GLfloat, GLint, glUniform1f, glUniform1i},
    uniform::UniformLocation,
};

pub trait Uniform {
    type Options;

    fn set(&self, options: Option<Self::Options>, location: &UniformLocation);
}

impl Uniform for GLfloat {
    type Options = ();

    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        unsafe {
            glUniform1f(*location, *self);
        }
    }
}

impl Uniform for GLint {
    type Options = ();
    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        unsafe { glUniform1i(*location, *self) };
    }
}
