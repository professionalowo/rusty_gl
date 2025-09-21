use std::{
    fmt,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    assimp::{AMaterial, material_key::MaterialKey},
    framework::{
        drawelement::Drawelement,
        material::{Material, MaterialConversionError},
        mesh::normalize::NormalizeOptions,
    },
};

use rmath::{vec2::Vec2, vec3::Vec3};

use gl_sys::{
    vao::VertexArrayObject,
    vbo::{Location, VBOError, VertexBufferObject},
};

pub mod normalize;

#[derive(Debug, Default, Clone)]
pub struct VboData {
    pub vbo: VertexBufferObject,
    pub buffer_type: gl_sys::bindings::GLenum,
    pub dimensions: u32,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub vao: VertexArrayObject,
    pub ibo: VertexBufferObject,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub vbos: [Option<VboData>; 4],
    pub primitive_type: gl_sys::bindings::GLenum,
}

impl Mesh {
    pub fn with_defaults() -> Self {
        Self {
            vao: VertexArrayObject::gen_vertex_arrays(),
            ibo: VertexBufferObject::gen_buffers(),
            num_vertices: 0,
            num_indices: 0,
            vbos: Default::default(),
            primitive_type: gl_sys::bindings::GL_TRIANGLES,
        }
    }

    pub fn bind(&self) {
        VertexArrayObject::bind_vertex_array(&self.vao);
    }
    pub fn unbind(&self) {
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
    }
    pub fn draw(&self) {
        gl_sys::draw_elements(
            gl_sys::bindings::GL_TRIANGLES,
            self.num_indices as i32,
            gl_sys::bindings::GL_UNSIGNED_INT,
        );
    }

    pub fn from_ai_mesh(mesh: &assimp::Mesh<'_>) -> Result<Self, MeshLoadError> {
        let mut positions: Vec<Vec3<f32>> = Vec::with_capacity(mesh.num_vertices() as usize);
        let mut normals: Vec<Vec3<f32>> = Vec::with_capacity(if mesh.has_normals() {
            mesh.num_vertices() as usize
        } else {
            0
        });
        let mut texcoords: Vec<Vec2<f32>> = Vec::with_capacity(if mesh.has_texture_coords(0) {
            mesh.num_vertices() as usize
        } else {
            0
        });
        let mut tangents: Vec<Vec3<f32>> =
            Vec::with_capacity(if mesh.has_tangents_and_bitangents() {
                mesh.num_vertices() as usize
            } else {
                0
            });

        for i in 0..mesh.num_vertices() {
            positions.push(Vec3::from(mesh.get_vertex(i).ok_or(
                MeshLoadError::MeshConversionFailed(format!("Could not get vertex[{i}]")),
            )?));
            if mesh.has_normals() {
                normals.push(Vec3::from(mesh.get_normal(i).ok_or(
                    MeshLoadError::MeshConversionFailed(format!("Could not get normal[{i}]")),
                )?));
            }
            if mesh.has_texture_coords(0) {
                let v = mesh
                    .get_texture_coord(0, i)
                    .ok_or(MeshLoadError::MeshConversionFailed(format!(
                        "Could not get texture_coord[{i}]"
                    )))?;
                texcoords.push(Vec2::new(v.x, v.y))
            }
            if mesh.has_tangents_and_bitangents() {
                tangents.push(Vec3::from(mesh.get_tangent(i).ok_or(
                    MeshLoadError::MeshConversionFailed(format!("Could not get tangent[{i}]")),
                )?));
            }
        }

        let indices: Vec<u32> = mesh
            .face_iter()
            .filter(|f| f.num_indices == 3)
            .flat_map(|f| [f[0], f[1], f[2]])
            .collect();

        let mut m = Self::with_defaults();
        m.add_vbo(0, 3, positions.as_slice())?;
        if !normals.is_empty() {
            m.add_vbo(1, 3, normals.as_slice())?;
        }
        if !texcoords.is_empty() {
            m.add_vbo(2, 2, texcoords.as_slice())?;
        }
        if !tangents.is_empty() {
            m.add_vbo(3, 3, tangents.as_slice())?;
        }
        m.num_indices = indices.len() as u32;
        VertexArrayObject::bind_vertex_array(&m.vao);
        VertexBufferObject::bind_buffer(gl_sys::bindings::GL_ELEMENT_ARRAY_BUFFER, &m.ibo);
        VertexBufferObject::buffer_data(
            gl_sys::bindings::GL_ELEMENT_ARRAY_BUFFER,
            &indices,
            gl_sys::bindings::GL_STATIC_DRAW,
        )?;
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
        VertexBufferObject::bind_buffer(
            gl_sys::bindings::GL_ELEMENT_ARRAY_BUFFER,
            &VertexBufferObject::zero(),
        );
        Ok(m)
    }

    fn add_vbo<T>(
        &mut self,
        index: usize,
        dimensions: u32,
        data: &[T],
    ) -> Result<(), MeshLoadError> {
        if self.num_vertices != 0 && self.num_vertices != data.len() as u32 {
            return Err(MeshLoadError::UnequalNumberOfVertices {
                expected: self.num_indices,
                actual: data.len() as u32,
            });
        }
        self.num_vertices = data.len() as u32;

        let buffer_type = gl_sys::bindings::GL_ARRAY_BUFFER;

        VertexArrayObject::bind_vertex_array(&self.vao);
        let vbo = VertexBufferObject::gen_buffers();
        VertexBufferObject::bind_buffer(buffer_type, &vbo);
        VertexBufferObject::buffer_data(buffer_type, &data, gl_sys::bindings::GL_STATIC_DRAW)?;
        let loc = Location(index as u32);

        VertexBufferObject::enable_vertex_attrib_array(&loc);
        VertexBufferObject::vertex_attrib_pointer(
            &loc,
            dimensions,
            gl_sys::bindings::GL_FLOAT,
            false,
            (dimensions * 4) as i32,
            None,
        )?;
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
        VertexBufferObject::bind_buffer(buffer_type, &VertexBufferObject::zero());

        self.vbos[index] = Some(VboData {
            vbo,
            buffer_type,
            dimensions,
        });

        Ok(())
    }
}

#[derive(Debug)]
pub enum MeshLoadError {
    LoadFailed(String),
    InvalidPath(PathBuf),
    InvalidParent(PathBuf),
    MaterialNotFound(usize),
    MaterialConversionFailed(MaterialConversionError),
    MeshConversionFailed(String),
    UnequalNumberOfVertices { expected: u32, actual: u32 },
    VboError(VBOError),
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(s) => write!(f, "Failed to load mesh: {}", s),
            Self::InvalidPath(p) => write!(f, "Invalid path: {:?}", p),
            Self::InvalidParent(p) => write!(f, "Invalid parent: {:?}", p),
            Self::MaterialNotFound(index) => write!(f, "Material not found: {}", index),
            Self::MaterialConversionFailed(e) => fmt::Display::fmt(e, f),
            Self::MeshConversionFailed(reason) => write!(f, "Mesh conversion failed: {:?}", reason),
            Self::VboError(e) => fmt::Display::fmt(e, f),
            Self::UnequalNumberOfVertices {
                expected: e,
                actual: a,
            } => write!(f, "Expected {e} but got {a} vertices"),
        }
    }
}

impl From<VBOError> for MeshLoadError {
    fn from(value: VBOError) -> Self {
        Self::VboError(value)
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

pub fn load_mesh<P>(
    path: P,
    normalize: NormalizeOptions,
) -> Result<Box<[Drawelement]>, MeshLoadError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let base_path = path
        .parent()
        .ok_or_else(|| MeshLoadError::InvalidParent(path.to_path_buf()))?;

    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.generate_normals(|opt| opt.smooth = true);

    let path_str = &path
        .to_str()
        .ok_or_else(|| MeshLoadError::InvalidPath(path.to_path_buf()))?;
    let mut scene = importer.read_file(path_str)?;

    normalize.normalize_scene(&mut scene);

    let mut drawelements: Vec<Drawelement> = Vec::with_capacity(scene.num_meshes() as usize);
    let mut materials: Vec<Rc<Material>> = Vec::with_capacity(scene.num_materials() as usize);

    for mat in scene.material_iter().map(AMaterial) {
        let name = mat
            .get_material_string(MaterialProperty::Name, 0, 0)
            .map_err(MaterialConversionError::AiError)?;
        let rc = Rc::new(Material::from_ai_mesh(&mat, name, base_path)?);
        materials.push(rc);
    }

    for aimesh in scene.mesh_iter() {
        let mesh = Mesh::from_ai_mesh(&aimesh)?;

        let index = aimesh.material_index as usize;
        let material = materials
            .get(index)
            .ok_or_else(|| MeshLoadError::MaterialNotFound(index))?
            .clone();

        drawelements.push(Drawelement { material, mesh });
    }
    Ok(drawelements.into_boxed_slice())
}
