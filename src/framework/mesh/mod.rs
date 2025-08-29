use std::{ffi::CString, fmt, path::PathBuf, rc::Rc};

use crate::{
    framework::{
        assimp::AMaterial,
        drawelement::Drawelement,
        material::{Material, MaterialConversionError},
    },
    gl::{
        self,
        vao::VertexArrayObject,
        vbo::{Location, VBOError, VertexBufferObject},
    },
    math::{vec2::Vec2, vec3::Vec3},
};

#[derive(Debug, Default)]
pub struct Mesh {
    pub vao: VertexArrayObject,
    pub ibo: VertexBufferObject,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub vbos: Vec<(VertexBufferObject, gl::GLenum, u32)>,
    pub primitive_type: gl::GLenum,
}

impl Mesh {
    pub fn with_defaults() -> Self {
        Self {
            vao: VertexArrayObject::gen_vertex_arrays(),
            ibo: VertexBufferObject::gen_buffers(),
            num_vertices: 0,
            num_indices: 0,
            vbos: Vec::with_capacity(4),
            primitive_type: gl::GL_TRIANGLES,
        }
    }

    fn add_vbo<T>(&mut self, dimensions: u32, data: &[T]) -> Result<(), MeshLoadError> {
        if self.num_vertices != 0 && self.num_vertices != data.len() as u32 {
            return Err(MeshLoadError::MeshConversionFailed);
        }
        self.num_vertices = data.len() as u32;

        VertexArrayObject::bind_vertex_array(&self.vao);
        let vbo = VertexBufferObject::gen_buffers();
        VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, &vbo);
        VertexBufferObject::buffer_data(
            gl::GL_ARRAY_BUFFER,
            (data.len() as u32 * dimensions).into(),
			&data,
            gl::GL_STATIC_DRAW,
        )?;
        let loc = Location(self.vbos.len() as u32);
        VertexBufferObject::enable_vertex_attrib_array(&loc);
        VertexBufferObject::vertex_attrib_pointer::<T>(
            &loc,
            dimensions,
            gl::GL_FLOAT,
            false,
            None,
        )?;
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
        VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, &VertexBufferObject::zero());

        self.vbos.push((vbo, gl::GL_ARRAY_BUFFER, dimensions));

        Ok(())
    }

    pub fn bind(&self) {
        VertexArrayObject::bind_vertex_array(&self.vao);
    }
    pub fn unbind(&self) {
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
    }
    pub fn draw(&self) {
        gl::draw_elements(
            gl::GL_TRIANGLES,
            self.num_indices as i32,
            gl::GL_UNSIGNED_INT,
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
            positions.push(Vec3::from(
                mesh.get_vertex(i)
                    .ok_or(MeshLoadError::MeshConversionFailed)?,
            ));
            if mesh.has_normals() {
                normals.push(Vec3::from(
                    mesh.get_normal(i)
                        .ok_or(MeshLoadError::MeshConversionFailed)?,
                ));
            }
            if mesh.has_texture_coords(0) {
                let v = mesh
                    .get_texture_coord(0, i)
                    .ok_or(MeshLoadError::MeshConversionFailed)?;
                texcoords.push(Vec2::new(v.x, v.y))
            }
            if mesh.has_tangents_and_bitangents() {
                tangents.push(Vec3::from(
                    mesh.get_tangent(i)
                        .ok_or(MeshLoadError::MeshConversionFailed)?,
                ));
            }
        }

        let mut indices: Vec<u32> = Vec::with_capacity(mesh.num_faces() as usize * 3);
        for i in 0..mesh.num_faces() {
            let face = mesh
                .get_face(i)
                .ok_or(MeshLoadError::MeshConversionFailed)?;
            if face.num_indices == 3 {
                indices.push(face[0]);
                indices.push(face[1]);
                indices.push(face[2]);
            }
        }

        let mut m = Mesh::with_defaults();
        m.add_vbo(3, positions.as_slice())?;
        if !normals.is_empty() {
            m.add_vbo(3, normals.as_slice())?;
        }
        if !texcoords.is_empty() {
            m.add_vbo(2, texcoords.as_slice())?;
        }
        if !tangents.is_empty() {
            m.add_vbo(3, tangents.as_slice())?;
        }
        m.num_indices = indices.len() as u32;
        VertexArrayObject::bind_vertex_array(&m.vao);
        VertexBufferObject::bind_buffer(gl::GL_ELEMENT_ARRAY_BUFFER, &m.ibo);
        VertexBufferObject::buffer_data(
            gl::GL_ELEMENT_ARRAY_BUFFER,
            (indices.len() as u32 * 2).into(),
            &indices,
            gl::GL_STATIC_DRAW,
        )?;
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
        VertexBufferObject::bind_buffer(gl::GL_ELEMENT_ARRAY_BUFFER, &VertexBufferObject::zero());
        Ok(m)
    }
}

#[derive(Debug)]
pub enum MeshLoadError {
    LoadFailed(String),
    InvalidPath(PathBuf),
    MaterialNotFound(usize),
    MaterialConversionFailed(MaterialConversionError),
    MeshConversionFailed,
    VboError(VBOError),
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(s) => write!(f, "Failed to load mesh: {}", s),
            Self::InvalidPath(p) => write!(f, "Invalid path: {:?}", p),
            Self::MaterialNotFound(index) => write!(f, "Material not found: {}", index),
            Self::MaterialConversionFailed(e) => fmt::Display::fmt(e, f),
            Self::MeshConversionFailed => write!(f, "Mesh conversion failed"),
            Self::VboError(e) => fmt::Display::fmt(e, f),
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

pub fn load_mesh(path: PathBuf) -> Result<Box<[Drawelement]>, MeshLoadError> {
    let base_path = path
        .parent()
        .ok_or_else(|| MeshLoadError::InvalidPath(path.clone()))?;

    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.generate_normals(|_| ());

    let path_str = &path
        .to_str()
        .ok_or(MeshLoadError::InvalidPath(path.clone()))?;
    let scene = importer.read_file(path_str)?;

    let mut drawelements: Vec<Drawelement> = Vec::with_capacity(scene.num_meshes() as usize);
    let mut materials: Vec<Rc<Material>> = Vec::with_capacity(scene.num_materials() as usize);

    for mat in scene.material_iter() {
        let amat = AMaterial(mat);
        let rc = Rc::new(Material::from_ai_mesh(
            base_path,
            amat.get_material_string(
                CString::new("?mat.name").map_err(MaterialConversionError::NulError)?,
                0,
                0,
            )
            .map_err(MaterialConversionError::AiError)?,
            &amat,
        )?);
        materials.push(rc);
    }

    for mesh in scene.mesh_iter() {
        let index = mesh.material_index as usize;
        let material = materials
            .get_mut(index)
            .ok_or_else(|| MeshLoadError::MaterialNotFound(index))?
            .clone();

        let mesh = mesh.try_into()?;

        let drawelement = Drawelement { material, mesh };
        drawelements.push(drawelement);
    }
    Ok(drawelements.into_boxed_slice())
}

impl TryFrom<assimp::Mesh<'_>> for Mesh {
    type Error = MeshLoadError;

    fn try_from(value: assimp::Mesh) -> Result<Self, Self::Error> {
        Mesh::from_ai_mesh(&value)
    }
}
