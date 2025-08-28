use std::{fmt, path::PathBuf, rc::Rc};

use crate::{
    framework::{
        drawelement::Drawelement,
        material::{Material, MaterialConversionError},
    },
    gl,
};

#[derive(Debug, Default)]
pub struct Mesh {
    pub vao: gl::GLuint,
    pub ibo: gl::GLuint,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub vbo_ids: Vec<gl::GLuint>,
    pub vbo_types: Vec<gl::GLenum>,
    pub vbo_dims: Vec<u32>,
    pub primitive_type: gl::GLenum,
}

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
    MaterialConversionFailed(MaterialConversionError),
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(s) => write!(f, "Failed to load mesh: {}", s),
            Self::InvalidPath(p) => write!(f, "Invalid path: {:?}", p),
            Self::MaterialNotFound(index) => write!(f, "Material not found: {}", index),
            Self::MaterialConversionFailed(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<&str> for MeshLoadError {
    fn from(s: &str) -> Self {
        Self::LoadFailed(s.to_string())
    }
}

impl From<MaterialConversionError> for MeshLoadError {
    fn from(value: MaterialConversionError) -> Self {
        Self::MaterialConversionFailed(value)
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
    let mut materials: Vec<Rc<Material>> = Vec::with_capacity(scene.num_materials() as usize);

    for mat in scene.material_iter() {
        let rc = Rc::new(Material::default());
        materials.push(rc);
    }

    for mesh in scene.mesh_iter() {
        let index = mesh.material_index as usize;
        let material = materials
            .get_mut(index)
            .ok_or_else(|| MeshLoadError::MaterialNotFound(index))?
            .clone();

        let mesh = Mesh::default();

        let drawelement = Drawelement { material, mesh };
        drawelements.push(drawelement);
    }
    Ok(drawelements.into_boxed_slice())
}
