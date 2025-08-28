use crate::framework::{material::Material, mesh::Mesh};

#[derive(Debug)]
pub struct Drawelement {
    pub material: Option<Material>,
    pub mesh: Option<Mesh>,
}
