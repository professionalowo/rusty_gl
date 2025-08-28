use std::{fmt, path::PathBuf};

pub struct Mesh {}

#[derive(Debug)]
pub enum MeshLoadError {
    LoadFailed(String),
    InvalidPath(PathBuf),
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshLoadError::LoadFailed(s) => write!(f, "Failed to load mesh: {}", s),
            MeshLoadError::InvalidPath(p) => write!(f, "Invalid path: {:?}", p),
        }
    }
}

impl From<&str> for MeshLoadError {
    fn from(s: &str) -> Self {
        Self::LoadFailed(s.to_string())
    }
}

pub fn load_mesh(path: PathBuf) -> Result<Mesh, MeshLoadError> {
    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.generate_normals(|_| ());

    let path_str = path
        .to_str()
        .ok_or(MeshLoadError::InvalidPath(path.clone()))?;
    let scene = importer.read_file(path_str)?;
    Ok(Mesh {})
}
