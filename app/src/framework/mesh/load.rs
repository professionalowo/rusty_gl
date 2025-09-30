use std::{path::Path, rc::Rc};

use crate::{
    assimp::AMaterial,
    framework::{drawelement::Drawelement, mesh::Mesh},
};

use super::{Material, MeshLoadError, NormalizeOptions};

#[derive(Debug)]
#[repr(transparent)]
pub struct SceneImport(Box<[Drawelement]>);

impl SceneImport {
    pub fn import<P>(path: P, normalize: NormalizeOptions) -> Result<Self, MeshLoadError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let mut scene = load_ai_scene(path)?;
        normalize.normalize_scene(&mut scene);

        let base_path = match path.parent() {
            Some(parent) => parent,
            None => return Err(MeshLoadError::InvalidParent(path.to_path_buf())),
        }
        .to_path_buf();

        let mut materials = Vec::with_capacity(scene.num_materials() as _);

        for mat in scene.material_iter() {
            let material = Material::from_ai_material(&AMaterial(mat), &base_path)?;
            materials.push(Rc::new(material));
        }

        let mut drawelements = Vec::with_capacity(scene.num_meshes() as _);

        for aimesh in scene.mesh_iter() {
            let index = aimesh.material_index as usize;
            let material = materials[index].clone();

            let mesh = Mesh::from_ai_mesh(&aimesh)?;

            drawelements.push(Drawelement { material, mesh });
        }

        Ok(Self(drawelements.into_boxed_slice()))
    }

    #[inline]
    pub const fn elements(&self) -> &[Drawelement] {
        &self.0
    }
}

impl AsRef<[Drawelement]> for SceneImport {
	#[inline]
    fn as_ref(&self) -> &[Drawelement] {
        &self.0
    }
}

impl From<Vec<Drawelement>> for SceneImport {
	#[inline]
    fn from(value: Vec<Drawelement>) -> Self {
        Self(value.into_boxed_slice())
    }
}

impl From<Box<[Drawelement]>> for SceneImport {
	#[inline]
    fn from(value: Box<[Drawelement]>) -> Self {
        Self(value)
    }
}

impl From<SceneImport> for Box<[Drawelement]> {
	#[inline]
    fn from(SceneImport(elements): SceneImport) -> Self {
        elements
    }
}

fn load_ai_scene<'a>(path: &Path) -> Result<assimp::Scene<'a>, MeshLoadError> {
    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.generate_normals(|opt| opt.smooth = true);

    let path = match path.to_str() {
        Some(path) => path,
        None => return Err(MeshLoadError::InvalidPath(path.to_path_buf())),
    };

    let scene = match importer.read_file(path) {
        Ok(scene) => scene,
        Err(message) => return Err(MeshLoadError::LoadFailed(message.into())),
    };

    Ok(scene)
}
