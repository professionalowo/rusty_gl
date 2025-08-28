use std::rc::Rc;

use crate::framework::{material::Material, mesh::Mesh};

#[derive(Debug)]
pub struct Drawelement {
    pub material: Option<Rc<Material>>,
    pub mesh: Option<Mesh>,
}
