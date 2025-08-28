use std::{fmt, mem::take, path::PathBuf};

use crate::framework::{drawelement::Drawelement, material::Material};

#[derive(Debug)]
pub struct Mesh {}

impl Mesh {
    pub fn bind(&self) {
        todo!()
    }
    pub fn unbind(&self) {
        todo!()
    }
    pub fn draw(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub enum MeshLoadError {
    LoadFailed(String),
    InvalidPath(PathBuf),
    MaterialNotFound(usize),
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(s) => write!(f, "Failed to load mesh: {}", s),
            Self::InvalidPath(p) => write!(f, "Invalid path: {:?}", p),
            Self::MaterialNotFound(index) => write!(f, "Material not found: {}", index),
        }
    }
}

impl From<&str> for MeshLoadError {
    fn from(s: &str) -> Self {
        Self::LoadFailed(s.to_string())
    }
}

pub fn load_mesh(path: PathBuf) -> Result<Box<[Drawelement]>, MeshLoadError> {
    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.generate_normals(|_| ());

    let path_str = path
        .to_str()
        .ok_or(MeshLoadError::InvalidPath(path.clone()))?;
    let scene = importer.read_file(path_str)?;

    let mut drawelements: Vec<Drawelement> = Vec::with_capacity(scene.num_meshes() as usize);
    let mut materials: Vec<Option<Material>> = Vec::with_capacity(scene.num_materials() as usize);

    for mat in scene.material_iter() {
        materials.push(None);
    }

    for mesh in scene.mesh_iter() {
        let index = mesh.material_index as usize;
        let material = materials
            .get_mut(index)
            .ok_or_else(|| MeshLoadError::MaterialNotFound(index))?
            .take();

        let mesh = Some(Mesh {});

        let drawelement = Drawelement { material, mesh };
        drawelements.push(drawelement);
    }
    Ok(drawelements.into_boxed_slice())
}
