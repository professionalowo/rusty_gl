use std::{ops::Deref, path::Path, rc::Rc};

use crate::{
    assimp::{AMaterial, material_key::MaterialKey},
    framework::{drawelement::Drawelement, material::MaterialConversionError, mesh::Mesh},
};

use super::{Material, MeshLoadError, NormalizeOptions};

enum MaterialProperty {
    Name,
}

impl MaterialKey for MaterialProperty {
    fn get_key(&self) -> &str {
        match self {
            Self::Name => "?mat.name",
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct SceneImport(Box<[Drawelement]>);

impl SceneImport {
    pub fn import<P>(path: P, normalize: NormalizeOptions) -> Result<Self, MeshLoadError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let base_path = path
            .parent()
            .ok_or_else(|| MeshLoadError::InvalidParent(path.to_path_buf()))?;

        let mut scene = load_ai_scene(path)?;

        normalize.normalize_scene(&mut scene);

        let mut drawelements: Vec<Drawelement> = Vec::with_capacity(scene.num_meshes() as _);
        let mut materials: Vec<Rc<Material>> = Vec::with_capacity(scene.num_materials() as _);

        for mat in scene.material_iter().map(AMaterial) {
            let name = mat
                .get_material_string(MaterialProperty::Name, 0, 0)
                .map_err(MaterialConversionError::AiError)?;
            let rc = Rc::new(Material::from_ai_mesh(&mat, name, base_path)?);
            materials.push(rc);
        }

        for aimesh in scene.mesh_iter() {
            let index = aimesh.material_index as usize;
            let material = materials[index].clone();

            let mesh = Mesh::from_ai_mesh(&aimesh)?;

            drawelements.push(Drawelement { material, mesh });
        }

        Ok(Self(drawelements.into_boxed_slice()))
    }

    pub fn elements(&self) -> &[Drawelement] {
        &self.0
    }
}

impl AsRef<[Drawelement]> for SceneImport {
    fn as_ref(&self) -> &[Drawelement] {
        &self.0
    }
}

impl From<Vec<Drawelement>> for SceneImport {
    fn from(value: Vec<Drawelement>) -> Self {
        Self(value.into_boxed_slice())
    }
}

impl From<Box<[Drawelement]>> for SceneImport {
    fn from(value: Box<[Drawelement]>) -> Self {
        Self(value)
    }
}

impl From<SceneImport> for Box<[Drawelement]> {
    fn from(value: SceneImport) -> Self {
        value.0
    }
}

fn load_ai_scene<'a>(path: &Path) -> Result<assimp::Scene<'a>, MeshLoadError> {
    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.generate_normals(|opt| opt.smooth = true);

    let path_str = &path
        .to_str()
        .ok_or_else(|| MeshLoadError::InvalidPath(path.to_path_buf()))?;
    let scene = importer.read_file(path_str)?;
    Ok(scene)
}
